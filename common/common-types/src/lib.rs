#![no_std]

multiversx_sc::imports!();

// TODO: Likely don't need all the "is_empty" impls
// TODO: See if you need any of this: https://github.com/hyperledger-labs/yui-ibc-solidity/blob/main/contracts/proto/QBFT.sol

pub mod channel;
pub mod connection;

pub type Timestamp = u64;

pub type Path<M> = ManagedBuffer<M>;
pub type FixedLengthBuffer<M> = ManagedByteArray<M, 32>;

pub type ClientId<M> = ManagedBuffer<M>;
pub type ConnectionId<M> = ManagedBuffer<M>;
pub type ChannelId<M> = ManagedBuffer<M>;
pub type PortId<M> = ManagedBuffer<M>;
