use common_modules2::utils::{keccak256, vec_u8_to_str};
use common_types2::{ChannelId, ClientId, ConnectionId, Hash, Path, PortId, Sequence};

// https://github.com/hyperledger-labs/yui-ibc-solidity/blob/main/contracts/core/24-host/IBCCommitment.sol
// https://github.com/cosmos/ibc/tree/main/spec/core/ics-024-host-requirements#path-space

/// "clients/{identifier}/clientState"
pub fn get_client_state_path(client_id: &ClientId) -> Path {
    format!("clients/{}/clientState", vec_u8_to_str(client_id.clone()))
        .as_bytes()
        .to_vec()
}

/// "clients/{identifier}/consensusStates/{revision_number}-{height}"
pub fn get_consensus_state_path(
    client_id: &ClientId,
    revision_number: u64,
    revision_height: u64,
) -> Path {
    format!(
        "clients/{}/consensusStates/{}-{}",
        vec_u8_to_str(client_id.clone()),
        revision_number,
        revision_height
    )
    .as_bytes()
    .to_vec()
}

/// "connections/{identifier}"
pub fn get_connection_path(connection_id: &ConnectionId) -> Path {
    format!("connections/{}", vec_u8_to_str(connection_id.clone()))
        .as_bytes()
        .to_vec()
}

/// "channelEnds/ports/{identifier}/channels/{identifier}"
pub fn get_channel_path(port_id: &PortId, channel_id: &ChannelId) -> Path {
    format!(
        "channelEnds/ports/{}/channels/{}",
        vec_u8_to_str(port_id.clone()),
        vec_u8_to_str(channel_id.clone())
    )
    .as_bytes()
    .to_vec()
}

/// "commitments/ports/{identifier}/channels/{identifier}/sequences/{sequence}"
pub fn get_packet_commitment_path(
    port_id: &PortId,
    channel_id: &ChannelId,
    sequence: Sequence,
) -> Path {
    format!(
        "commitments/ports/{}/channels/{}/sequences/{}",
        vec_u8_to_str(port_id.clone()),
        vec_u8_to_str(channel_id.clone()),
        sequence
    )
    .as_bytes()
    .to_vec()
}

/// "acks/ports/{identifier}/channels/{identifier}/sequences/{sequence}"
pub fn get_packet_acknowledgement_commitment_path(
    port_id: &PortId,
    channel_id: &ChannelId,
    sequence: Sequence,
) -> Path {
    format!(
        "acks/ports/{}/channels/{}/sequences/{}",
        vec_u8_to_str(port_id.clone()),
        vec_u8_to_str(channel_id.clone()),
        sequence
    )
    .as_bytes()
    .to_vec()
}

/// "receipts/ports/{identifier}/channels/{identifier}/sequences/{sequence}"
pub fn get_packet_receipt_commitment_path(
    port_id: &PortId,
    channel_id: &ChannelId,
    sequence: Sequence,
) -> Path {
    format!(
        "receipts/ports/{}/channels/{}/sequences/{}",
        vec_u8_to_str(port_id.clone()),
        vec_u8_to_str(channel_id.clone()),
        sequence
    )
    .as_bytes()
    .to_vec()
}

/// "nextSequenceSend/ports/{identifier}/channels/{identifier}"
pub fn get_next_seq_send_commitment_path(port_id: &PortId, channel_id: &ChannelId) -> Path {
    format!(
        "nextSequenceSend/ports/{}/channels/{}",
        vec_u8_to_str(port_id.clone()),
        vec_u8_to_str(channel_id.clone()),
    )
    .as_bytes()
    .to_vec()
}

/// "nextSequenceRecv/ports/{identifier}/channels/{identifier}"
pub fn get_next_seq_recv_commitment_path(port_id: &PortId, channel_id: &ChannelId) -> Path {
    format!(
        "nextSequenceRecv/ports/{}/channels/{}",
        vec_u8_to_str(port_id.clone()),
        vec_u8_to_str(channel_id.clone()),
    )
    .as_bytes()
    .to_vec()
}

/// "nextSequenceAck/ports/{identifier}/channels/{identifier}"
pub fn get_next_seq_ack_commitment_path(port_id: &PortId, channel_id: &ChannelId) -> Path {
    format!(
        "nextSequenceAck/ports/{}/channels/{}",
        vec_u8_to_str(port_id.clone()),
        vec_u8_to_str(channel_id.clone()),
    )
    .as_bytes()
    .to_vec()
}

/// "channelUpgrades/upgrades/ports/{identifier}/channels/{identifier}"
pub fn get_channel_upgrade_path(port_id: &PortId, channel_id: &ChannelId) -> Path {
    format!(
        "channelUpgrades/upgrades/ports/{}/channels/{}",
        vec_u8_to_str(port_id.clone()),
        vec_u8_to_str(channel_id.clone()),
    )
    .as_bytes()
    .to_vec()
}

/// "channelUpgrades/upgradeError/ports/{identifier}/channels/{identifier}"
pub fn get_channel_upgrade_error_path(port_id: &PortId, channel_id: &ChannelId) -> Path {
    format!(
        "channelUpgrades/upgradeError/ports/{}/channels/{}",
        vec_u8_to_str(port_id.clone()),
        vec_u8_to_str(channel_id.clone()),
    )
    .as_bytes()
    .to_vec()
}

// key gen

pub fn get_client_state_commitment_key(client_id: &ClientId) -> Hash {
    keccak256(&get_client_state_path(client_id))
}

pub fn get_consensus_state_commitment_key(
    client_id: &ClientId,
    revision_number: u64,
    revision_height: u64,
) -> Hash {
    keccak256(&get_consensus_state_path(
        client_id,
        revision_number,
        revision_height,
    ))
}

pub fn get_connection_commitment_key(connection_id: &ConnectionId) -> Hash {
    keccak256(&get_connection_path(connection_id))
}

pub fn get_channel_commitment_key(port_id: &PortId, channel_id: &ChannelId) -> Hash {
    keccak256(&get_channel_path(port_id, channel_id))
}

pub fn get_next_seq_recv_commitment_key(port_id: &PortId, channel_id: &ChannelId) -> Hash {
    keccak256(&get_next_seq_recv_commitment_path(port_id, channel_id))
}

pub fn get_packet_commitment_key(
    port_id: &PortId,
    channel_id: &ChannelId,
    sequence: Sequence,
) -> Hash {
    keccak256(&get_packet_commitment_path(port_id, channel_id, sequence))
}

pub fn get_packet_acknowledgement_commitment_key(
    port_id: &PortId,
    channel_id: &ChannelId,
    sequence: Sequence,
) -> Hash {
    keccak256(&get_packet_acknowledgement_commitment_path(
        port_id, channel_id, sequence,
    ))
}

pub fn get_packet_receipt_commitment_key(
    port_id: &PortId,
    channel_id: &ChannelId,
    sequence: Sequence,
) -> Hash {
    keccak256(&get_packet_receipt_commitment_path(
        port_id, channel_id, sequence,
    ))
}

pub fn get_channel_upgrade_commitment_key(port_id: &PortId, channel_id: &ChannelId) -> Hash {
    keccak256(&get_channel_upgrade_path(port_id, channel_id))
}

pub fn get_channel_upgrade_error_commitment_key(port_id: &PortId, channel_id: &ChannelId) -> Hash {
    keccak256(&get_channel_upgrade_error_path(port_id, channel_id))
}
