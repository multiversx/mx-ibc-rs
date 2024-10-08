#![no_std]

multiversx_sc::imports!();

pub mod channel_types;
pub mod connection_types;

pub const HASH_LENGTH: usize = 32;
pub type Hash<M> = ManagedByteArray<M, HASH_LENGTH>;

pub type Timestamp = u64;
pub type Sequence = u64;

pub type ChainId<M> = ManagedByteArray<M, 32>;
pub type ClientId<M> = ManagedBuffer<M>;
pub type ClientType<M> = ManagedBuffer<M>;
pub type ConnectionId<M> = ManagedBuffer<M>;
pub type ChannelId<M> = ManagedBuffer<M>;
pub type PortId<M> = ManagedBuffer<M>;
pub type Path<M> = ManagedBuffer<M>;
