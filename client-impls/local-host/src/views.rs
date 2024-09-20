multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ViewsModule:
    client_common::CommonClientLogicModule + crate::client_logic::ClientLogicModule
{
}
