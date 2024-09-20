#![no_std]

multiversx_sc::imports!();

pub mod events;
pub mod merkle_proof;
pub mod update_clients;

#[multiversx_sc::contract]
pub trait Client:
    update_clients::UpdateClientsModule
    + merkle_proof::MerkleProofModule
    + events::EventsModule
    + host::commitment::CommitmentModule
    + host::host_config::HostConfigModule
    + host::host_views::HostViewsModule
    + host::module_manager::ModuleManagerModule
    + host::storage::StorageModule
    + common_modules::client_lib::ClientLibModule
    + common_modules::host_lib::HostLibModule
    + common_modules::utils::UtilsModule
{
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
