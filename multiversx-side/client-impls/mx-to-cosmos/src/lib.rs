#![no_std]

multiversx_sc::imports!();

pub mod client_logic;
pub mod cosmos_types;
pub mod views;

#[multiversx_sc::contract]
pub trait MxToCosmosClient:
    client_common::CommonClientLogicModule
    + client_logic::ClientLogicModule
    + views::ViewsModule
    + host::host_views::HostViewsModule
    + host::storage::StorageModule
    + common_modules::utils::UtilsModule
{
    #[init]
    fn init(&self, ibc_handler: ManagedAddress) {
        self.set_ibc_handler(&ibc_handler);
    }

    #[upgrade]
    fn upgrade(&self) {}
}
