#![no_std]

multiversx_sc::imports!();

pub mod commitment;
pub mod host_config;
pub mod host_views;
pub mod module_manager;
pub mod storage;

#[multiversx_sc::contract]
pub trait Ibc:
    host_config::HostConfigModule
    + module_manager::ModuleManagerModule
    + host_views::HostViewsModule
    + commitment::CommitmentModule
    + storage::StorageModule
    + common_modules::client_lib::ClientLibModule
    + common_modules::host_lib::HostLibModule
    + common_modules::check_char::CheckCharModule
    + common_modules::utils::UtilsModule
{
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
