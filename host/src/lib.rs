#![no_std]

multiversx_sc::imports!();

pub mod commitment;
pub mod storage;

#[multiversx_sc::contract]
pub trait Ibc: commitment::CommitmentModule + storage::StorageModule {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
