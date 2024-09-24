use common_types::{Hash, HASH_LENGTH};

use super::packet_types::PacketReceipt;

const EMPTY_HASH: &[u8; HASH_LENGTH] = &[0u8; HASH_LENGTH];

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait IbcChannelLibModule: common_modules::utils::UtilsModule {
    fn receipt_commitment_to_receipt(&self, commitment: &Hash<Self::Api>) -> PacketReceipt {
        if commitment == &Hash::new_from_bytes(EMPTY_HASH) {
            return PacketReceipt::None;
        }

        let encoded_success = self.encode_to_buffer(&PacketReceipt::Successful);
        let successful_hash = self.crypto().keccak256(&encoded_success);
        if commitment == &successful_hash {
            return PacketReceipt::Successful;
        }

        sc_panic!("Unknown channel packet receipt commitment");
    }
}
