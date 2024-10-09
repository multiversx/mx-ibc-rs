use common_types2::{
    channel_types::{channel, upgrade},
    ClientType, Sequence, UnixTimestamp,
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct ClientInfo {
    pub client_type: ClientType,
    pub client_impl: Addr,
}

#[cw_serde]
#[derive(Default)]
pub struct HostInfo {
    pub next_client_seq: Sequence,
    pub next_connection_seq: Sequence,
    pub next_channel_seq: Sequence,
    pub expected_time_per_block: UnixTimestamp,
}

#[cw_serde]
#[derive(Copy)]
pub struct RecvStartSequence {
    pub seq: Sequence,
    pub prev_seq: Sequence,
}

#[cw_serde]
pub struct ChannelInfo {
    pub channel: channel::Data,
    pub next_seq_send: Sequence,
    pub next_seq_recv: Sequence,
    pub next_seq_ack: Sequence,
    pub upgrade: upgrade::Data,
    pub latest_error_rec_seq: Sequence,
    pub recv_start_seq: RecvStartSequence,
    pub ack_start_seq: Sequence,
}

pub mod host_storage_keys {
    use common_types2::{
        connection_types::connection_end, ChannelId, ClientId, ClientType, ConnectionId, Hash,
        PortId,
    };
    use cosmwasm_std::Addr;
    use cw_storage_plus::{Item, Map};

    use super::{ChannelInfo, ClientInfo, HostInfo};

    pub const HOST_INFO: Item<HostInfo> = Item::new("hostInfo");

    pub const COMMITMENTS: Map<&Hash, Hash> = Map::new("commitments");
    pub const CLIENT_REGISTRY: Map<&ClientType, Addr> = Map::new("clientReg");
    pub const CLIENT_INFO: Map<&ClientId, ClientInfo> = Map::new("clientInfo");
    pub const PORT_CAPABILITIES: Map<&PortId, Addr> = Map::new("portCap");
    pub const CHANNEL_CAPABILITIES: Map<(&PortId, &ChannelId), Addr> = Map::new("channelCap");
    pub const CONNECTION_INFO: Map<&ConnectionId, connection_end::Data> = Map::new("connInfo");
    pub const CHANNEL_INFO: Map<(&PortId, &ChannelId), ChannelInfo> = Map::new("channelInfo");
}

pub mod host_helpers {
    use common_modules2::utils::std_err;
    use common_types2::{
        connection_types::connection_end, ChannelId, ClientId, ConnectionId, Hash, PortId,
        Sequence, UnixTimestamp,
    };
    use cosmwasm_std::{StdError, StdResult, Storage};

    use super::{
        host_storage_keys::{CHANNEL_INFO, CLIENT_INFO, COMMITMENTS, CONNECTION_INFO, HOST_INFO},
        ChannelInfo, ClientInfo, HostInfo,
    };

    // TODO: Add to queries
    pub fn get_commitment(storage: &dyn Storage, commitment_hash: &Hash) -> StdResult<Hash> {
        COMMITMENTS.load(storage, commitment_hash)
    }

    /// calculates the block delay based on the expected time per block
    pub fn calculate_block_delay(
        storage: &dyn Storage,
        time_delay: UnixTimestamp,
    ) -> StdResult<UnixTimestamp> {
        if time_delay == 0 {
            return Ok(0);
        }

        let host_info = HOST_INFO.load(storage)?;
        if host_info.expected_time_per_block == 0 {
            return Ok(0);
        }

        let block_delay = (time_delay + host_info.expected_time_per_block - 1)
            / host_info.expected_time_per_block;
        Ok(block_delay)
    }

    pub fn get_next_client_seq(storage: &mut dyn Storage) -> StdResult<Sequence> {
        let host_info_result: Result<_, StdError> =
            HOST_INFO.update(storage, |mut host_info: HostInfo| {
                host_info.next_client_seq += 1;

                Ok(host_info)
            });

        Ok(host_info_result?.next_client_seq - 1)
    }

    pub fn get_next_connection_seq(storage: &mut dyn Storage) -> StdResult<Sequence> {
        let host_info_result: Result<_, StdError> =
            HOST_INFO.update(storage, |mut host_info: HostInfo| {
                host_info.next_connection_seq += 1;

                Ok(host_info)
            });

        Ok(host_info_result?.next_connection_seq - 1)
    }

    pub fn get_next_channel_seq(storage: &mut dyn Storage) -> StdResult<Sequence> {
        let host_info_result: Result<_, StdError> =
            HOST_INFO.update(storage, |mut host_info: HostInfo| {
                host_info.next_channel_seq += 1;

                Ok(host_info)
            });

        Ok(host_info_result?.next_channel_seq - 1)
    }

    #[inline]
    pub fn try_get_client_info(
        storage: &dyn Storage,
        client_id: &ClientId,
    ) -> StdResult<ClientInfo> {
        CLIENT_INFO.load(storage, client_id)
    }

    #[inline]
    pub fn try_get_connection_info(
        storage: &dyn Storage,
        connection_id: &ConnectionId,
    ) -> StdResult<connection_end::Data> {
        CONNECTION_INFO.load(storage, connection_id)
    }

    #[inline]
    pub fn try_get_channel_info(
        storage: &dyn Storage,
        port_id: &PortId,
        channel_id: &ChannelId,
    ) -> StdResult<ChannelInfo> {
        CHANNEL_INFO.load(storage, (port_id, channel_id))
    }
}
