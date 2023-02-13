pub fn get_validator_from_indexed_deposit_data(indexed_deposit_data: &IndexedDepositData) -> Validator {
    let amount = indexed_deposit_data.amount;
    let effective_balance = std::cmp::min(amount - amount % EFFECTIVE_BALANCE_INCREMENT, MAX_EFFECTIVE_BALANCE);
    
    Validator {
        pubkey: indexed_deposit_data.pubkey.clone(),
        withdrawal_credentials: indexed_deposit_data.withdrawal_credentials.clone(),
        activation_eligibility_epoch: FAR_FUTURE_EPOCH,
        activation_epoch: FAR_FUTURE_EPOCH,
        exit_epoch: FAR_FUTURE_EPOCH,
        withdrawable_epoch: FAR_FUTURE_EPOCH,
        effective_balance,
    }
}

pub fn apply_indexed_deposit_data<T: EthSpec>(
    state: &mut BeaconState<T>,
    indexed_deposit_data: &IndexedDepositData,
) {
    let pubkey = &indexed_deposit_data.pubkey;
    let amount = indexed_deposit_data.amount;
    let validator_pubkeys = state.validators.iter().map(|v| &v.pubkey).collect::<Vec<_>>();
    
    if !validator_pubkeys.contains(&pubkey) {
        // Add validator and balance entries
        let validator = get_validator_from_indexed_deposit_data(indexed_deposit_data);
        state.validators.push(validator);
        state.balances.push(amount);
    } else {
        // Increase balance by deposit amount
        let index = ValidatorIndex(validator_pubkeys.iter().position(|&v| v == pubkey).unwrap() as u64);
        increase_balance(state, index, amount);
    }
}

pub fn process_pending_deposits<T: EthSpec>(
    state: &mut BeaconState<T>,
) {
    let finalized_epoch = state.finalized_checkpoint.epoch;
    let slots_per_epoch = T::slots_per_epoch();
    let max_deposits = T::max_deposits();

    let mut next_pending_deposit_index = 0;

    for pending_deposit in &state.pending_deposits {
        // Preserve deposits per epoch boundary
        if next_pending_deposit_index >= max_deposits * slots_per_epoch {
            break;
        }

        // Apply only finalized deposits
        if pending_deposit.epoch >= finalized_epoch {
            break;
        }

        // Skip already applied deposits
        if pending_deposit.index >= state.eth1_deposit_index {
            apply_indexed_deposit_data(state, pending_deposit);
            state.eth1_deposit_index += 1;
        }

        next_pending_deposit_index += 1;
    }

    state.pending_deposits = state.pending_deposits[next_pending_deposit_index..].to_vec();
}

