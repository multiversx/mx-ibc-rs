#![no_std]

multiversx_sc::imports!();

pub mod client_logic;
pub mod mock_types;
pub mod views;

#[multiversx_sc::contract]
pub trait Mock: client_common::CommonClientLogicModule + client_logic::ClientLogicModule {
    #[init]
    fn init(&self, ibc_handler: ManagedAddress) {
        self.ibc_handler().set(ibc_handler);
    }

    #[upgrade]
    fn upgrade(&self) {}
}
