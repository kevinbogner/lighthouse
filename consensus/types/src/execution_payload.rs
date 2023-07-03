use crate::{test_utils::TestRandom, *};
use derivative::Derivative;
use serde_derive::{Deserialize, Serialize};
use ssz::{Decode, Encode};
use ssz_derive::{Decode, Encode};
use test_random_derive::TestRandom;
use tree_hash_derive::TreeHash;

pub type Transaction<N> = VariableList<u8, N>;
pub type Transactions<T> = VariableList<
    Transaction<<T as EthSpec>::MaxBytesPerTransaction>,
    <T as EthSpec>::MaxTransactionsPerPayload,
>;

pub type Withdrawals<T> = VariableList<Withdrawal, <T as EthSpec>::MaxWithdrawalsPerPayload>;

pub type DepositReceipts<T> =
    VariableList<DepositReceipt, <T as EthSpec>::MaxDepositReceiptsPerPayload>;

#[superstruct(
    variants(Merge, Capella, Deneb, Eip6110),
    variant_attributes(
        derive(
            Default,
            Debug,
            Clone,
            Serialize,
            Deserialize,
            Encode,
            Decode,
            TreeHash,
            TestRandom,
            Derivative,
            arbitrary::Arbitrary
        ),
        derivative(PartialEq, Hash(bound = "T: EthSpec")),
        serde(bound = "T: EthSpec", deny_unknown_fields),
        arbitrary(bound = "T: EthSpec")
    ),
    cast_error(ty = "Error", expr = "BeaconStateError::IncorrectStateVariant"),
    partial_getter_error(ty = "Error", expr = "BeaconStateError::IncorrectStateVariant"),
    map_into(FullPayload, BlindedPayload),
    map_ref_into(ExecutionPayloadHeader)
)]
#[derive(
    Debug, Clone, Serialize, Encode, Deserialize, TreeHash, Derivative, arbitrary::Arbitrary,
)]
#[derivative(PartialEq, Hash(bound = "T: EthSpec"))]
#[serde(bound = "T: EthSpec", untagged)]
#[arbitrary(bound = "T: EthSpec")]
#[ssz(enum_behaviour = "transparent")]
#[tree_hash(enum_behaviour = "transparent")]
pub struct ExecutionPayload<T: EthSpec> {
    #[superstruct(getter(copy))]
    pub parent_hash: ExecutionBlockHash,
    #[superstruct(getter(copy))]
    pub fee_recipient: Address,
    #[superstruct(getter(copy))]
    pub state_root: Hash256,
    #[superstruct(getter(copy))]
    pub receipts_root: Hash256,
    #[serde(with = "ssz_types::serde_utils::hex_fixed_vec")]
    pub logs_bloom: FixedVector<u8, T::BytesPerLogsBloom>,
    #[superstruct(getter(copy))]
    pub prev_randao: Hash256,
    #[serde(with = "serde_utils::quoted_u64")]
    #[superstruct(getter(copy))]
    pub block_number: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    #[superstruct(getter(copy))]
    pub gas_limit: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    #[superstruct(getter(copy))]
    pub gas_used: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    #[superstruct(getter(copy))]
    pub timestamp: u64,
    #[serde(with = "ssz_types::serde_utils::hex_var_list")]
    pub extra_data: VariableList<u8, T::MaxExtraDataBytes>,
    #[serde(with = "serde_utils::quoted_u256")]
    #[superstruct(getter(copy))]
    pub base_fee_per_gas: Uint256,
    #[superstruct(getter(copy))]
    pub block_hash: ExecutionBlockHash,
    #[serde(with = "ssz_types::serde_utils::list_of_hex_var_list")]
    pub transactions: Transactions<T>,
    #[superstruct(only(Capella, Deneb, Eip6110))]
    pub withdrawals: Withdrawals<T>,
    #[superstruct(only(Deneb, Eip6110))]
    #[serde(with = "serde_utils::quoted_u256")]
    #[superstruct(getter(copy))]
    pub excess_data_gas: Uint256,
    #[superstruct(only(Eip6110))]
    pub deposit_receipts: DepositReceipts<T>,
}

impl<'a, T: EthSpec> ExecutionPayloadRef<'a, T> {
    // this emulates clone on a normal reference type
    pub fn clone_from_ref(&self) -> ExecutionPayload<T> {
        map_execution_payload_ref!(&'a _, self, move |payload, cons| {
            cons(payload);
            payload.clone().into()
        })
    }
}

impl<T: EthSpec> ExecutionPayload<T> {
    pub fn from_ssz_bytes(bytes: &[u8], fork_name: ForkName) -> Result<Self, ssz::DecodeError> {
        match fork_name {
            ForkName::Base | ForkName::Altair => Err(ssz::DecodeError::BytesInvalid(format!(
                "unsupported fork for ExecutionPayload: {fork_name}",
            ))),
            ForkName::Merge => ExecutionPayloadMerge::from_ssz_bytes(bytes).map(Self::Merge),
            ForkName::Capella => ExecutionPayloadCapella::from_ssz_bytes(bytes).map(Self::Capella),
            ForkName::Deneb => ExecutionPayloadDeneb::from_ssz_bytes(bytes).map(Self::Deneb),
            ForkName::Eip6110 => ExecutionPayloadEip6110::from_ssz_bytes(bytes).map(Self::Eip6110),
        }
    }

    #[allow(clippy::integer_arithmetic)]
    /// Returns the maximum size of an execution payload.
    pub fn max_execution_payload_merge_size() -> usize {
        // Fixed part
        ExecutionPayloadMerge::<T>::default().as_ssz_bytes().len()
            // Max size of variable length `extra_data` field
            + (T::max_extra_data_bytes() * <u8 as Encode>::ssz_fixed_len())
            // Max size of variable length `transactions` field
            + (T::max_transactions_per_payload() * (ssz::BYTES_PER_LENGTH_OFFSET + T::max_bytes_per_transaction()))
    }

    #[allow(clippy::integer_arithmetic)]
    /// Returns the maximum size of an execution payload.
    pub fn max_execution_payload_capella_size() -> usize {
        // Fixed part
        ExecutionPayloadCapella::<T>::default().as_ssz_bytes().len()
            // Max size of variable length `extra_data` field
            + (T::max_extra_data_bytes() * <u8 as Encode>::ssz_fixed_len())
            // Max size of variable length `transactions` field
            + (T::max_transactions_per_payload() * (ssz::BYTES_PER_LENGTH_OFFSET + T::max_bytes_per_transaction()))
            // Max size of variable length `withdrawals` field
            + (T::max_withdrawals_per_payload() * <Withdrawal as Encode>::ssz_fixed_len())
    }

    #[allow(clippy::integer_arithmetic)]
    /// Returns the maximum size of an execution payload.
    pub fn max_execution_payload_deneb_size() -> usize {
        // Fixed part
        ExecutionPayloadDeneb::<T>::default().as_ssz_bytes().len()
            // Max size of variable length `extra_data` field
            + (T::max_extra_data_bytes() * <u8 as Encode>::ssz_fixed_len())
            // Max size of variable length `transactions` field
            + (T::max_transactions_per_payload() * (ssz::BYTES_PER_LENGTH_OFFSET + T::max_bytes_per_transaction()))
            // Max size of variable length `withdrawals` field
            + (T::max_withdrawals_per_payload() * <Withdrawal as Encode>::ssz_fixed_len())
    }

    #[allow(clippy::integer_arithmetic)]
    /// Returns the maximum size of an execution payload.
    pub fn max_execution_payload_eip6110_size() -> usize {
        // Fixed part
        ExecutionPayloadEip6110::<T>::default().as_ssz_bytes().len()
            // Max size of variable length `extra_data` field
            + (T::max_extra_data_bytes() * <u8 as Encode>::ssz_fixed_len())
            // Max size of variable length `transactions` field
            + (T::max_transactions_per_payload() * (ssz::BYTES_PER_LENGTH_OFFSET + T::max_bytes_per_transaction()))
            // Max size of variable length `deposit_receipts` field
            + (T::max_deposit_receipts_per_payload() * <DepositReceipt as Encode>::ssz_fixed_len())
    }
}

impl<T: EthSpec> ForkVersionDeserialize for ExecutionPayload<T> {
    fn deserialize_by_fork<'de, D: serde::Deserializer<'de>>(
        value: serde_json::value::Value,
        fork_name: ForkName,
    ) -> Result<Self, D::Error> {
        let convert_err = |e| {
            serde::de::Error::custom(format!("ExecutionPayload failed to deserialize: {:?}", e))
        };

        Ok(match fork_name {
            ForkName::Merge => Self::Merge(serde_json::from_value(value).map_err(convert_err)?),
            ForkName::Capella => Self::Capella(serde_json::from_value(value).map_err(convert_err)?),
            ForkName::Deneb => Self::Deneb(serde_json::from_value(value).map_err(convert_err)?),
            ForkName::Eip6110 => Self::Eip6110(serde_json::from_value(value).map_err(convert_err)?),
            ForkName::Base | ForkName::Altair => {
                return Err(serde::de::Error::custom(format!(
                    "ExecutionPayload failed to deserialize: unsupported fork '{}'",
                    fork_name
                )));
            }
        })
    }
}

impl<T: EthSpec> ExecutionPayload<T> {
    pub fn fork_name(&self) -> ForkName {
        match self {
            ExecutionPayload::Merge(_) => ForkName::Merge,
            ExecutionPayload::Capella(_) => ForkName::Capella,
            ExecutionPayload::Deneb(_) => ForkName::Deneb,
            ExecutionPayload::Eip6110(_) => ForkName::Eip6110,
        }
    }
}
