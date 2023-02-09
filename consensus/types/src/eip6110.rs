use crate::*;

/* EIP-6110
class DepositReceipt(Container):
    pubkey: BLSPubkey
    withdrawal_credentials: Bytes32
    amount: Gwei
    signature: BLSSignature
    index: uint64

class IndexedDepositData(Container):
    pubkey: BLSPubkey
    withdrawal_credentials: Bytes32
    amount: Gwei
    index: uint64
    epoch: Epoch
 */

pub struct DepositReceipt {
    pub pubkey: PublicKey,
    pub withdrawal_credentials: Hash256,
    pub amount: u64,
    pub signature: Signature,
    pub index: u64,
}

pub struct IndexedDepositData {
    pub pubkey: PublicKey,
    pub withdrawal_credentials: Hash256,
    pub amount: u64,
    pub index: u64,
    pub epoch: Epoch,
}
