#![no_std]

multiversx_sc::imports!();

pub type Path<M> = ManagedBuffer<M>;
pub type FixedLengthBuffer<M> = ManagedByteArray<M, 32>;

pub type ClientId<M> = ManagedBuffer<M>;
pub type ConnectionId<M> = ManagedBuffer<M>;
pub type ChannelId<M> = ManagedBuffer<M>;
pub type PortId<M> = ManagedBuffer<M>;
