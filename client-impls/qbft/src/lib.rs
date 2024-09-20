#![no_std]

multiversx_sc::imports!();

pub mod client_logic;
pub mod qbft_types;
pub mod views;

// Ignore this whole crate, still work in progress. Not even entirely sure we need something like this yet.

#[multiversx_sc::contract]
pub trait Qbft:
    client_logic::ClientLogicModule
    + views::ViewsModule
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
