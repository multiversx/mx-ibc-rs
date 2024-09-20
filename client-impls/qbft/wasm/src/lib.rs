// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           11
// Async Callback (empty):               1
// Total number of exported functions:  14

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    qbft
    (
        init => init
        upgrade => upgrade
        getIbcHandler => ibc_handler
        initializeClient => initialize_client
        updateClient => update_client
        getTimestampAtHeight => get_timestamp_at_height
        getLatestHeight => get_latest_height
        getStatus => get_status
        getLatestInfo => get_latest_info
        getHostTimestamp => get_host_timestamp
        getCommitmentPrefix => get_commitment_prefix
        checkAndGetClient => check_and_get_client
        getCommitment => get_commitment
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
