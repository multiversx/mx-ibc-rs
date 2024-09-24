use client_common::{VerifyMembershipArgs, VerifyNonMembershipArgs};
use common_types::{
    channel_types::{
        channel::{self, Order},
        height,
    },
    connection_types::connection_end,
    ChannelId, ClientId, Hash, PortId, Sequence, Timestamp,
};

use crate::channel_libs::packet_types::{MsgTimeoutPacket, Packet, TimeoutArgs};

multiversx_sc::imports!();

static UNEXPECTED_PACKET_DEST_ERR_MSG: &[u8] = b"Unexpected packet destination";

mod generic_client_proxy {
    use client_common::{VerifyMembershipArgs, VerifyNonMembershipArgs};
    use common_types::{channel_types::height, ClientId, Timestamp};

    multiversx_sc::imports!();

    #[multiversx_sc::proxy]
    pub trait GenericClientProxy {
        #[view(getTimestampAtHeight)]
        fn get_timestamp_at_height(
            &self,
            client_id: &ClientId<Self::Api>,
            height: &height::Data,
        ) -> Timestamp;

        #[view(verifyMembership)]
        fn verify_membership(&self, args: VerifyMembershipArgs<Self::Api>) -> bool;

        #[view(verifyNonMembership)]
        fn verify_non_membership(&self, args: VerifyNonMembershipArgs<Self::Api>) -> bool;
    }
}

#[multiversx_sc::module]
pub trait PacketTimeoutModule:
    host::module_manager::ModuleManagerModule
    + host::storage::StorageModule
    + host::commitment::CommitmentModule
    + common_modules::utils::UtilsModule
    + crate::channel_libs::events::EventsModule
{
    #[endpoint(timeoutPacket)]
    fn timeout_packet(&self, args: MsgTimeoutPacket<Self::Api>) {
        let mut channel_info =
            self.try_get_channel_info(&args.packet.source_port, &args.packet.source_channel);
        let mut channel = &mut channel_info.channel;
        self.check_expected_args(&args.packet, &channel);

        let connection_info = self.try_get_connection_info(&channel.connection_hops.get(0));
        let client_info = self.try_get_client_info(&connection_info.client_id);
        self.check_timeout_reached(
            &args.packet,
            args.proof_height,
            client_info.client_impl.clone(),
            connection_info.client_id.clone(),
        );

        let commitment_mapper = self.check_and_get_commitment_mapper(
            &args.packet.source_port,
            &args.packet.source_channel,
            args.packet.sequence,
        );
        let commitment = commitment_mapper.get();
        let packet_commitment = self.get_packet_commitment(&args.packet);
        require!(
            commitment == packet_commitment,
            "Packet commitment mismatch"
        );

        self.check_channel_membership(
            client_info.client_impl,
            &connection_info,
            &mut channel,
            &args,
        );

        commitment_mapper.clear();

        let ibc_module =
            self.lookup_module_by_channel(&args.packet.source_port, &args.packet.source_channel);

        self.timeout_packet_event(&args.packet);

        /*
        lookupModuleByChannel(msg_.packet.sourcePort, msg_.packet.sourceChannel).onTimeoutPacket(
            msg_.packet, _msgSender()
        );
         */

        // TODO: Set in storage

        // TODO: Check args for other function
    }

    fn check_expected_args(&self, packet: &Packet<Self::Api>, channel: &channel::Data<Self::Api>) {
        require!(
            matches!(channel.state, channel::State::Open),
            "Unexpected channel state"
        );
        require!(
            packet.dest_port == channel.counterparty.port_id,
            UNEXPECTED_PACKET_DEST_ERR_MSG
        );
        require!(
            packet.dest_channel == channel.counterparty.channel_id,
            UNEXPECTED_PACKET_DEST_ERR_MSG
        );
    }

    fn check_timeout_reached(
        &self,
        packet: &Packet<Self::Api>,
        proof_height: height::Data,
        client_impl: ManagedAddress,
        client_id: ClientId<Self::Api>,
    ) {
        if !packet.timeout_height.is_zero() && proof_height >= packet.timeout_height {
            return;
        }

        let timestamp_at_height: Timestamp = self
            .generic_client_proxy_impl(client_impl)
            .get_timestamp_at_height(client_id, proof_height)
            .execute_on_dest_context();
        require!(
            packet.timeout_timestamp != 0 && timestamp_at_height >= packet.timeout_timestamp,
            "Channel timeout not reached"
        );
    }

    fn check_and_get_commitment_mapper(
        &self,
        src_port: &PortId<Self::Api>,
        src_channel: &ChannelId<Self::Api>,
        seq: Sequence,
    ) -> SingleValueMapper<Hash<Self::Api>> {
        let comm_hash = self.get_packet_commitment_key(src_port, src_channel, seq);
        let comm_mapper = self.commitments(&comm_hash);
        require!(!comm_mapper.is_empty(), "Packet commitment not found");

        comm_mapper
    }

    fn get_packet_commitment(&self, packet: &Packet<Self::Api>) -> Hash<Self::Api> {
        let hashed_data = self.crypto().sha256(&packet.data);

        let encoded_timeout_timestamp = self.encode_to_buffer(&packet.timeout_timestamp);
        let encoded_revision_number = self.encode_to_buffer(&packet.timeout_height.revision_number);
        let encoded_revision_height = self.encode_to_buffer(&packet.timeout_height.revision_height);
        let encoded_hashed_data = self.encode_to_buffer(&hashed_data);

        let mut encoded_buffer = ManagedBuffer::new();
        encoded_buffer = encoded_buffer.concat(encoded_timeout_timestamp);
        encoded_buffer = encoded_buffer.concat(encoded_revision_number);
        encoded_buffer = encoded_buffer.concat(encoded_revision_height);
        encoded_buffer = encoded_buffer.concat(encoded_hashed_data);

        let first_hashing = self.crypto().sha256(&encoded_buffer);

        self.crypto().keccak256(first_hashing.as_managed_buffer())
    }

    fn check_channel_membership(
        &self,
        client_impl: ManagedAddress,
        connection_info: &connection_end::Data<Self::Api>,
        channel_info: &mut channel::Data<Self::Api>,
        timeout_args: &dyn TimeoutArgs<Self::Api>,
    ) {
        match channel_info.ordering {
            Order::Ordered => self.check_channel_ordered_membership(
                client_impl,
                &connection_info,
                channel_info,
                timeout_args,
            ),
            Order::Unordered => {
                self.check_channel_unordered_membership(client_impl, &connection_info, timeout_args)
            }
            Order::NoneUnspecified => sc_panic!("Unknown channel order"),
        };
    }

    fn check_channel_ordered_membership(
        &self,
        client_impl: ManagedAddress,
        connection_info: &connection_end::Data<Self::Api>,
        channel_info: &mut channel::Data<Self::Api>,
        timeout_args: &dyn TimeoutArgs<Self::Api>,
    ) {
        let packet = timeout_args.get_packet();
        require!(
            packet.sequence >= timeout_args.get_next_seq_recv(),
            "Packet may already be received"
        );

        let encoded_value = self.encode_to_buffer(&timeout_args.get_next_seq_recv());
        let membership_args = VerifyMembershipArgs {
            client_id: connection_info.client_id.clone(),
            height: timeout_args.get_proof_height(),
            delay_time_period: connection_info.delay_period,
            delay_block_period: self.calculate_block_delay(connection_info.delay_period),
            proof: timeout_args.get_proof().clone(),
            prefix: connection_info.counterparty.prefix.key_prefix.clone(),
            path: self.get_next_seq_recv_commitment_path(&packet.dest_port, &packet.dest_channel),
            value: encoded_value,
        };
        let membership_result: bool = self
            .generic_client_proxy_impl(client_impl)
            .verify_membership(membership_args)
            .execute_on_dest_context();
        require!(membership_result, "Failed to verify next seq receive");

        channel_info.state = channel::State::Closed;
    }

    fn check_channel_unordered_membership(
        &self,
        client_impl: ManagedAddress,
        connection_info: &connection_end::Data<Self::Api>,
        timeout_args: &dyn TimeoutArgs<Self::Api>,
    ) {
        let packet = timeout_args.get_packet();
        let path = self.get_packet_receipt_commitment_path(
            &packet.dest_port,
            &packet.dest_channel,
            packet.sequence,
        );
        let non_membership_args = VerifyNonMembershipArgs {
            client_id: connection_info.client_id.clone(),
            height: timeout_args.get_proof_height(),
            delay_time_period: connection_info.delay_period,
            delay_block_period: self.calculate_block_delay(connection_info.delay_period),
            proof: timeout_args.get_proof().clone(),
            prefix: connection_info.counterparty.prefix.key_prefix.clone(),
            path,
        };
        let non_membership_result: bool = self
            .generic_client_proxy_impl(client_impl)
            .verify_non_membership(non_membership_args)
            .execute_on_dest_context();
        require!(
            non_membership_result,
            "Failed to verify packet receipt absence"
        );
    }

    /// calculates the block delay based on the expected time per block
    fn calculate_block_delay(&self, time_delay: Timestamp) -> Timestamp {
        if time_delay == 0 {
            return 0;
        }

        let host_info = self.host_info().get();
        if host_info.expected_time_per_block == 0 {
            return 0;
        }

        (time_delay + host_info.expected_time_per_block - 1) / host_info.expected_time_per_block
    }

    #[proxy]
    fn generic_client_proxy_impl(
        &self,
        sc_address: ManagedAddress,
    ) -> generic_client_proxy::GenericClientProxy<Self::Api>;
}
