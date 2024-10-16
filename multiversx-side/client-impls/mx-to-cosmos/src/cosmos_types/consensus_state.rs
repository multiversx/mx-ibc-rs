use common_types::{Hash, UnixTimestamp};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct Data<M: ManagedTypeApi> {
    pub timestamp: UnixTimestamp,
    pub root: Hash<M>,
    pub validators: ManagedVec<M, ManagedAddress<M>>, // TODO: Why was this bytes[]? Also, might be able to use ArrayVec over ManagedVec
}
