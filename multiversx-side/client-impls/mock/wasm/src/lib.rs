// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           12
// Async Callback (empty):               1
// Total number of exported functions:  15

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    mock
    (
        init => init
        upgrade => upgrade
        getIbcHandler => ibc_handler
        initializeClient => initialize_client
        setStatus => set_status
        updateClient => update_client
        getTimestampAtHeight => get_timestamp_at_height
        getLatestHeight => get_latest_height
        getStatus => get_status
        getLatestInfo => get_latest_info
        verifyMembership => verify_membership
        verifyNonMembership => verify_non_membership
        getClientState => get_client_state
        getConsensusState => get_consensus_state
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}