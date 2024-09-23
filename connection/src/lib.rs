#![no_std]

multiversx_sc::imports!();

pub mod connection_lib;
pub mod connection_types;

#[multiversx_sc::contract]
pub trait Connection: connection_lib::ConnectionLibModule {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
