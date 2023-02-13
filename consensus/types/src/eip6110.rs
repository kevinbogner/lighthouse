use crate::*;
use serde::{Deserialize, Serialize};

// EIP-6110

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepositReceipt {
    pub pubkey: PublicKey,
    pub withdrawal_credentials: Hash256,
    pub amount: u64,
    pub signature: Signature,
    pub index: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexedDepositData {
    pub pubkey: PublicKey,
    pub withdrawal_credentials: Hash256,
    pub amount: u64,
    pub index: u64,
    pub epoch: Epoch,
}
