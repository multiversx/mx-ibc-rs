#![no_std]

multiversx_sc::imports!();

pub mod commitment;
pub mod host_views;
pub mod module_manager;
pub mod storage;

#[multiversx_sc::contract]
pub trait Ibc:
    module_manager::ModuleManagerModule
    + host_views::HostViewsModule
    + commitment::CommitmentModule
    + storage::StorageModule
{
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
