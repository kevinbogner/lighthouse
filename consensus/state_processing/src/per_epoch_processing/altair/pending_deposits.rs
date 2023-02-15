use crate::{common::increase_balance, EpochProcessingError};
use safe_arith::SafeArith;
use types::{
    eip6110::IndexedDepositData, BeaconState, ChainSpec, EthSpec, ParticipationFlags, Unsigned,
    Validator,
};

pub fn get_validator_from_indexed_deposit_data<T: EthSpec>(
    indexed_deposit_data: &IndexedDepositData,
    spec: &ChainSpec,
) -> Result<Validator, EpochProcessingError> {
    let amount = indexed_deposit_data.amount;
    let effective_balance = std::cmp::min(
        amount.safe_sub(amount.safe_rem(spec.effective_balance_increment)?)?,
        spec.max_effective_balance,
    );

    let validator = Validator {
        pubkey: indexed_deposit_data.pubkey,
        withdrawal_credentials: indexed_deposit_data.withdrawal_credentials.clone(),
        activation_eligibility_epoch: spec.far_future_epoch,
        activation_epoch: spec.far_future_epoch,
        exit_epoch: spec.far_future_epoch,
        withdrawable_epoch: spec.far_future_epoch,
        effective_balance,
        slashed: false,
    };

    Ok(validator)
}

pub fn apply_indexed_deposit_data<T: EthSpec>(
    state: &mut BeaconState<T>,
    indexed_deposit_data: &IndexedDepositData,
    spec: &ChainSpec,
) -> Result<(), EpochProcessingError> {
    let pubkey = &indexed_deposit_data.pubkey;
    let amount = indexed_deposit_data.amount;
    let validator_pubkeys = state
        .validators()
        .iter()
        .map(|v| &v.pubkey)
        .collect::<Vec<_>>();

    if !validator_pubkeys.contains(&pubkey) {
        // Add validator and balance entries
        let validator = get_validator_from_indexed_deposit_data::<T>(indexed_deposit_data, spec)?;
        state.validators().push(validator);
        state.balances().push(amount);

        // Altair or later initializations.
        if let Ok(previous_epoch_participation) = state.previous_epoch_participation_mut() {
            previous_epoch_participation.push(ParticipationFlags::default())?;
        }
        if let Ok(current_epoch_participation) = state.current_epoch_participation_mut() {
            current_epoch_participation.push(ParticipationFlags::default())?;
        }
        if let Ok(inactivity_scores) = state.inactivity_scores_mut() {
            inactivity_scores.push(0)?;
        }
    } else {
        // Increase balance by deposit amount
        let index = validator_pubkeys.iter().position(|&v| v == pubkey).unwrap();
        increase_balance(state, index, amount)?;

        increase_balance(state, index, amount)?;
    }

    Ok(())
}

pub fn process_pending_deposits<T: EthSpec>(
    state: &mut BeaconState<T>,
    spec: &ChainSpec,
) -> Result<(), EpochProcessingError> {
    let finalized_epoch = state.finalized_checkpoint().epoch;
    let slots_per_epoch = T::slots_per_epoch();
    let max_deposits = <T as EthSpec>::MaxDeposits::to_u64();

    let mut next_pending_deposit_index = 0;

    for pending_deposit in state.pending_deposits() {
        // Preserve deposits per epoch boundary
        if next_pending_deposit_index >= max_deposits * slots_per_epoch {
            break;
        }

        // Apply only finalized deposits
        if pending_deposit.epoch >= finalized_epoch {
            break;
        }

        // Skip already applied deposits
        if pending_deposit.index >= state.eth1_deposit_index() {
            apply_indexed_deposit_data(state, pending_deposit, spec)?;
            state.eth1_deposit_index_mut().safe_add_assign(1)?;
        }

        next_pending_deposit_index += 1;
    }

    // TODO: Implement the drain method for VariableList or replace it with a workaround
    state.pending_deposits().drain(..next_pending_deposit_index);

    Ok(())
}
