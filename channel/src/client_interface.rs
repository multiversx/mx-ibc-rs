pub mod generic_client_proxy {
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
