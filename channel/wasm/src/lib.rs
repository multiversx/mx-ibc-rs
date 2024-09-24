// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                            8
// Async Callback (empty):               1
// Total number of exported functions:  11

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    channel
    (
        init => init
        upgrade => upgrade
        timeoutPacket => timeout_packet
        setExpectedTimePerBlock => set_expected_time_per_block
        registerClient => register_client
        bindPort => bind_port
        getHostTimestamp => get_host_timestamp
        getCommitmentPrefix => get_commitment_prefix
        checkAndGetClient => check_and_get_client
        getCommitment => get_commitment
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
