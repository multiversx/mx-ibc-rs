#![no_std]

multiversx_sc::imports!();

pub mod merkle_proof;
pub mod update_clients;

#[multiversx_sc::contract]
pub trait Client:
    update_clients::UpdateClientsModule
    + merkle_proof::MerkleProofModule
    + host::host_views::HostViewsModule
    + host::storage::StorageModule
{
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
