use common_types::EncodedHeight;

use super::parts::{Commit, CosmWasmHeaderPart, ValidatorSet};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TypeAbi, TopDecode)]
pub struct SignedCosmWasmHeaderFull<M: ManagedTypeApi> {
    pub header: CosmWasmHeaderPart<M>,
    pub commit: Commit<M>,
    pub validator_set: ValidatorSet<M>,
    pub trusted_height: EncodedHeight<M>,
}
