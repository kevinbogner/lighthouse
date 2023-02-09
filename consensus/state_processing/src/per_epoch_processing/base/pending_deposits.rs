pub fn process_pending_deposits<T: EthSpec>(
    state: &mut BeaconState<T>,
    validator_statuses: &mut ValidatorStatuses,
    spec: &ChainSpec,
) -> Result<(), Error> {
   
    Ok(())
}

/*
def process_pending_deposits(state: BeaconState) -> None:
    finalized_epoch = state.finalized_checkpoint.epoch

    next_pending_deposit_index = 0
    for pending_deposit in state.pending_deposits:
        # Preserve deposits per epoch boundary
        if next_pending_deposit_index >= MAX_DEPOSITS * SLOTS_PER_EPOCH:
            break

        # Apply only finalized deposits
        if pending_deposit.epoch >= finalized_epoch:
            break

        # Skip already applied deposits
        if pending_deposit.index >= state.eth1_deposit_index:
            apply_indexed_deposit_data(state, pending_deposit)
            state.eth1_deposit_index += 1

        next_pending_deposit_index += 1

    state.pending_deposit = state.pending_deposit[next_pending_deposit_index:]
*/
