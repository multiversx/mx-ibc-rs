use common_types::{ClientId, PortId, Timestamp};

use crate::storage::HostInfo;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait HostConfigModule:
    crate::module_manager::ModuleManagerModule
    + crate::storage::StorageModule
    + common_modules::client_lib::ClientLibModule
    + common_modules::host_lib::HostLibModule
    + common_modules::utils::UtilsModule
{
    fn set_expected_time_per_block(&self, exp_time_per_block: Timestamp) {
        let mapper = self.host_info();
        if !mapper.is_empty() {
            mapper.update(|host_info| host_info.expected_time_per_block = exp_time_per_block);

            return;
        }

        let default_host_value = HostInfo {
            expected_time_per_block: exp_time_per_block,
            ..Default::default()
        };
        mapper.set(default_host_value);
    }

    fn register_client(&self, client_id: &ClientId<Self::Api>, client: &ManagedAddress) {
        require!(self.is_valid_client_id(client_id), "Invalid client ID");

        let mapper = self.client_registry(client_id);
        require!(mapper.is_empty(), "Client already exists");
        self.require_valid_address(client);

        mapper.set(client);
    }

    fn bind_port(&self, port_id: &PortId<Self::Api>, module: &ManagedAddress) {
        require!(self.is_valid_port_id(port_id), "Invalid Port ID");
        self.require_valid_address(module);

        self.claim_port_capability(port_id, module);
    }
}
