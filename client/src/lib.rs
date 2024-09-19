#![no_std]

multiversx_sc::imports!();

pub mod merkle_proof;
pub mod qbft;
pub mod update_clients;

#[multiversx_sc::contract]
pub trait Client:
    update_clients::UpdateClientsModule
    + merkle_proof::MerkleProofModule
    + qbft::QbftModule
    + host::host_views::HostViewsModule
    + host::storage::StorageModule
{
    #[init]
    fn init(&self, ibc_handler: ManagedAddress) {
        self.ibc_handler().set(ibc_handler);
    }

    #[upgrade]
    fn upgrade(&self) {}
}
