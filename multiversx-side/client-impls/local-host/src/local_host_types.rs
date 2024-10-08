pub mod client_state {
    use common_types::channel_types::height;

    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    #[derive(TypeAbi, TopEncode, TopDecode)]
    pub struct Data {
        pub latest_height: height::Data,
    }

    impl Data {
        #[inline]
        pub fn new(latest_height: height::Data) -> Self {
            Self { latest_height }
        }
    }
}

pub mod consensus_state {
    use common_types::UnixTimestamp;

    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    #[derive(TypeAbi, TopEncode, TopDecode)]
    pub struct Data {
        pub timestamp: UnixTimestamp,
    }

    impl Data {
        #[inline]
        pub fn new(timestamp: UnixTimestamp) -> Self {
            Self { timestamp }
        }
    }
}
