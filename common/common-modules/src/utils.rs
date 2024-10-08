use common_types::Timestamp;

multiversx_sc::imports!();

const NANO_SECONDS_MULT: u64 = 1_000_000_000;

#[multiversx_sc::module]
pub trait UtilsModule {
    fn require_valid_address(&self, address: &ManagedAddress) {
        let own_sc_address = self.blockchain().get_sc_address();
        require!(
            address != &own_sc_address && !address.is_zero(),
            "Invalid address"
        );
    }

    fn checked_timestamp_to_unix_mul(&self, timestamp: Timestamp) -> Timestamp {
        match timestamp.checked_mul(NANO_SECONDS_MULT) {
            Some(result) => result,
            None => sc_panic!("Overlow!!!"),
        }
    }
}
