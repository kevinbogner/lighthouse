// Copy the structure of an existing API handler, e.g. get_lighthouse_block_rewards.

pub fn compute_sync_committee_rewards<T: BeaconChainTypes>(
    chain: &BeaconChain<T>,
    state: BeaconState<T::EthSpec>,
    block: SignedBlindedBeaconBlock<T::EthSpec>,
) -> Result<SyncCommitteeRewards, Error> {

    let get_lighthouse_sync_committee_rewards = warp::path("lighthouse")
    .and(warp::path("analysis"))
    .and(warp::path("sync_committee_rewards"))
    .and(warp::query::<eth2::lighthouse::SyncCommitteeRewardsQuery>())
    .and(warp::path::end())
    .and(chain_filter.clone())
    .and(log_filter.clone())
    .and_then(|query, chain, log| {
        blocking_json_task(move || sync_committee_rewards::get_sync_committee_rewards(query, chain, log))
    });
}

// Load a block with chain.get_blinded_block(block_root).
pub fn get_blinded_block(
    &self,
    block_root: &Hash256,
) -> Result<Option<SignedBlindedBeaconBlock<T::EthSpec>>, Error> {
    Ok(self.store.get_blinded_block(block_root)?)
}

// Load a state with chain.get_state(state_root, None)
pub fn get_state(
    &self,
    state_root: &Hash256,
    slot: Option<Slot>,
) -> Result<Option<BeaconState<T::EthSpec>>, Error> {
    Ok(self.store.get_state(state_root, slot)?)
}

// Convert a slot into the canonical block root from that slot: block_id.root(&chain).
pub fn root(&self, chain: &BeaconChain<T>) -> Result<Option<Hash256>, Error> {
    match self {
        BlockId::Root(root) => Ok(Some(*root)),
        BlockId::Slot(slot) => {
            let root = chain
                .store
                .get_canonical_block_root(*slot)
                .map_err(Error::BeaconChainError)?;
            Ok(root)
        }
    }
}

//Once we have the block(s) and state that we need, we can compute the rewards using snippets of logic extracted from consensus/state_processing.

//We want to avoid modifying consensus/state_processing because that code needs to be fast (no extra calculations) and correct (modifying it is medium-risk).

//For some of the APIs we might want to use the BlockReplayer. This will be useful if we want to diff the pre-state against the post-state. In hindsight I think this will probably only be relevant for the attestation rewards API where we need to replay multiple blocks.
