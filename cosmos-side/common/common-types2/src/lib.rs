use connection_types::version;

pub mod channel_types;
pub mod connection_types;

pub const HASH_LENGTH: usize = 32;
pub type Hash = [u8; HASH_LENGTH];

pub type UnixTimestamp = u64;
pub type Sequence = u64;

pub type ChainId = [u8; 32];
pub type ClientId = Vec<u8>;
pub type ClientType = Vec<u8>;
pub type ConnectionId = Vec<u8>;
pub type ChannelId = Vec<u8>;
pub type Feature = Vec<u8>;
pub type FeatureId = Vec<u8>;
pub type PortId = Vec<u8>;
pub type Path = Vec<u8>;
pub type Version = Vec<u8>;

pub type VersionVec = Vec<version::Data>;
pub type FeatureVec = Vec<Feature>;
pub type ConnectionHops = Vec<ConnectionId>;
