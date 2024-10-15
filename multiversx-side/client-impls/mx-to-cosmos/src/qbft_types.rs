pub mod client_state {
    use common_types::{channel_types::height, ChainId};

    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    #[derive(TypeAbi, TopEncode, TopDecode)]
    pub struct Data<M: ManagedTypeApi> {
        pub chain_id: ChainId<M>,
        pub ibc_store_address: ManagedAddress<M>,
        pub latest_height: height::Data,
        pub trusting_period: u64, // TODO: Likely timestamp
        pub max_clock_drift: u64, // TODO: No idea what this means, likely a timestamp too?
    }
}

pub mod consensus_state {
    use common_types::{Hash, UnixTimestamp};

    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    #[derive(TypeAbi, TopEncode, TopDecode)]
    pub struct Data<M: ManagedTypeApi> {
        pub timestamp: UnixTimestamp,
        pub root: Hash<M>,
        pub validators: ManagedVec<M, ManagedAddress<M>>, // TODO: Why was this bytes[]? Also, might be able to use ArrayVec over ManagedVec
    }
}

pub mod header {
    use common_types::{channel_types::height, ChainId, Hash, UnixTimestamp};

    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    #[derive(TypeAbi, TopEncode, TopDecode)]
    pub struct Data<M: ManagedTypeApi> {
        pub besu_header: ManagedBuffer<M>,
        pub seals: ManagedVec<M, ManagedBuffer<M>>,
        pub trusted_height: height::Data,
        pub account_state_proof: Hash<M>,
    }

    // ^ Might not need that

    #[derive(TypeAbi, TopDecode, NestedDecode)]
    pub struct HeaderVersion {
        pub block: u64,
        pub app: u32,
    }

    #[derive(TypeAbi, TopDecode, NestedDecode)]
    pub struct PartSetHeader<M: ManagedTypeApi> {
        pub total: u64,
        pub hash: Hash<M>,
    }

    #[derive(TypeAbi, TopDecode, NestedDecode)]
    pub struct BlockId<M: ManagedTypeApi> {
        pub hash: Hash<M>,
        pub part_set_header: PartSetHeader<M>,
    }

    #[derive(TypeAbi, TopDecode, NestedDecode)]
    pub struct CosmWasmHeaderPart<M: ManagedTypeApi> {
        pub version: HeaderVersion,
        pub chain_id: ChainId<M>,
        pub height: u64, // TODO: Maybe BigUint (i.e. two concatenated u64)
        pub time: UnixTimestamp,
        pub last_block_id: BlockId<M>,
        pub last_commit_hash: Hash<M>,
        pub data_hash: Hash<M>,
        pub validators_hash: Hash<M>,
        pub next_validators_hash: Hash<M>,
        pub consensus_hash: Hash<M>,
        pub app_hash: Hash<M>,
        pub last_results_hash: Hash<M>,
        pub evidence_hash: Hash<M>,
        pub proposer_address: ManagedAddress<M>,
    }

    // TODO: Fix len
    pub type Signature<M> = ManagedByteArray<M, 2048>;

    #[derive(TypeAbi, TopDecode, NestedDecode, ManagedVecItem)]
    pub struct SignatureData<M: ManagedTypeApi> {
        pub block_id_flag: u64, // TODO: Unsure if u64 or simply u8
        pub validator: ManagedAddress<M>,
        pub timestamp: UnixTimestamp,
        pub signature: Signature<M>,
    }

    #[derive(TypeAbi, TopDecode, NestedDecode)]
    pub struct Commit<M: ManagedTypeApi> {
        pub height: u64, // TODO: Maybe BigUint (i.e. two concatenated u64),
        pub round: u64,
        pub block_id: BlockId<M>,
        pub signatures: ManagedVec<M, SignatureData<M>>,
    }

    #[derive(TypeAbi, TopDecode)]
    pub struct SignedCosmWasmHeaderFull<M: ManagedTypeApi> {
        pub header: CosmWasmHeaderPart<M>,
        pub commit: Commit<M>,
    }
}
