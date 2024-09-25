#![no_std]

multiversx_sc::imports!();

pub mod channel_libs;
pub mod interfaces;
pub mod packet_handling;

#[multiversx_sc::contract]
pub trait Channel:
    channel_libs::ibc_channel_lib::IbcChannelLibModule
    + channel_libs::events::EventsModule
    + packet_handling::timeout::TimeoutModule
    + packet_handling::membership::MembershipModule
    + packet_handling::send::SendModule
    + host::commitment::CommitmentModule
    + host::host_config::HostConfigModule
    + host::host_views::HostViewsModule
    + host::module_manager::ModuleManagerModule
    + host::storage::StorageModule
    + common_modules::client_lib::ClientLibModule
    + common_modules::host_lib::HostLibModule
    + common_modules::utils::UtilsModule
{
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
