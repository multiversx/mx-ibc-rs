use common_types::{channel_types::height, Hash, UnixTimestamp};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct Data<M: ManagedTypeApi> {
    pub besu_header: ManagedBuffer<M>,
    pub seals: ManagedVec<M, ManagedBuffer<M>>,
    pub trusted_height: height::Data,
    pub account_state_proof: Hash<M>,
}

// ^ Might not need that

pub type Signature<M> = ManagedByteArray<M, 64>;
pub type CosmosAddress<M> = ManagedByteArray<M, 20>;
pub type Ed25519Sum<M> = ManagedByteArray<M, 32>;

#[derive(TypeAbi, TopDecode, NestedDecode)]
pub struct HeaderVersion {
    pub block: u64,
    pub app: u32,
}

#[derive(TypeAbi, TopDecode, NestedDecode)]
pub struct PartSetHeader<M: ManagedTypeApi> {
    pub total: u64,
    pub hash: Hash<M>,
}

#[derive(TypeAbi, TopDecode, NestedDecode)]
pub struct BlockId<M: ManagedTypeApi> {
    pub hash: Hash<M>,
    pub part_set_header: PartSetHeader<M>,
}

#[derive(TypeAbi, TopDecode, NestedDecode, ManagedVecItem)]
pub struct SignatureData<M: ManagedTypeApi> {
    pub block_id_flag: u64, // TODO: Unsure if u64 or simply u8
    pub validator: CosmosAddress<M>,
    pub timestamp: UnixTimestamp,
    pub signature: Signature<M>,
}

#[derive(TypeAbi, TopDecode, NestedDecode, ManagedVecItem)]
pub struct Sum<M: ManagedTypeApi> {
    pub ed25519: Ed25519Sum<M>,
}

#[derive(TypeAbi, TopDecode, NestedDecode, ManagedVecItem)]
pub struct PublicKey<M: ManagedTypeApi> {
    pub sum: Sum<M>,
}

#[derive(TypeAbi, TopDecode, NestedDecode, ManagedVecItem)]
pub struct Validator<M: ManagedTypeApi> {
    pub address: CosmosAddress<M>,
    pub public_key: PublicKey<M>,
    pub voting_power: u64,
    pub proposer_priority: i64, // TODO: Make sure decoding signed integers works
}

pub type Proposer<M> = Validator<M>;
