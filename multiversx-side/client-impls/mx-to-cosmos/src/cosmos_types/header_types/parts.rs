use common_types::{ChainId, Hash, UnixTimestamp};

use super::helpers::{BlockId, CosmosAddress, HeaderVersion, Proposer, SignatureData, Validator};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TypeAbi, TopDecode, NestedDecode)]
pub struct CosmWasmHeaderPart<M: ManagedTypeApi> {
    pub version: HeaderVersion,
    pub chain_id: ChainId<M>,
    pub height: u64, // TODO: Maybe BigUint (i.e. two concatenated u64)
    pub time: UnixTimestamp,
    pub last_block_id: BlockId<M>,
    pub last_commit_hash: Hash<M>,
    pub data_hash: Hash<M>,
    pub validators_hash: Hash<M>,
    pub next_validators_hash: Hash<M>,
    pub consensus_hash: Hash<M>,
    pub app_hash: Hash<M>,
    pub last_results_hash: Hash<M>,
    pub evidence_hash: Hash<M>,
    pub proposer_address: CosmosAddress<M>,
}

#[derive(TypeAbi, TopDecode, NestedDecode)]
pub struct Commit<M: ManagedTypeApi> {
    pub height: u64, // TODO: Maybe BigUint (i.e. two concatenated u64),
    pub round: u64,
    pub block_id: BlockId<M>,
    pub signatures: ManagedVec<M, SignatureData<M>>,
}

#[derive(TypeAbi, TopDecode, NestedDecode)]
pub struct ValidatorSet<M: ManagedTypeApi> {
    pub validators: ManagedVec<M, Validator<M>>,
    pub proposer: Proposer<M>,
    pub total_voting_power: u64,
}
