use crate::{test_utils::TestRandom, *};
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};

use test_random_derive::TestRandom;
use tree_hash_derive::TreeHash;

// EIP-6110

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode, TreeHash, TestRandom,
)]
pub struct DepositReceipt {
    pub pubkey: PublicKeyBytes,
    pub withdrawal_credentials: Hash256,
    pub amount: u64,
    pub signature: Signature,
    pub index: u64,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode, TreeHash, TestRandom,
)]
pub struct IndexedDepositData {
    pub pubkey: PublicKeyBytes,
    pub withdrawal_credentials: Hash256,
    pub amount: u64,
    pub index: u64,
    pub epoch: Epoch,
}
