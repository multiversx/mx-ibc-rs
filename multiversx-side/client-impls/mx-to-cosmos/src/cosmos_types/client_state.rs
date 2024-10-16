use common_types::{channel_types::height, ChainId};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct Data<M: ManagedTypeApi> {
    pub chain_id: ChainId<M>,
    pub ibc_store_address: ManagedAddress<M>,
    pub latest_height: height::Data,
    pub trusting_period: u64, // TODO: Likely timestamp
    pub max_clock_drift: u64, // TODO: No idea what this means, likely a timestamp too?
}
