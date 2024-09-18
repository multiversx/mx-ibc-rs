multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait UtilsModule {
    fn require_valid_address(&self, address: &ManagedAddress) {
        let own_sc_address = self.blockchain().get_sc_address();
        require!(
            address != &own_sc_address && !address.is_zero(),
            "Invalid address"
        );
    }
}
