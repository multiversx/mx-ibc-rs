#![no_std]

multiversx_sc::imports!();

pub mod conn_endpoints;
pub mod conn_lib;
pub mod conn_types;
pub mod events;

#[multiversx_sc::contract]
pub trait Connection:
    conn_lib::ConnectionLibModule
    + conn_endpoints::ConnectionEndpointsModule
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
