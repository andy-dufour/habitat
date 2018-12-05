// Note that this file is meant to be included, not used standalone. See src/manager/mod.rs for
// more details.
lazy_static! {
    static ref SVC_LOAD_COUNTER: Counter = register_counter!(opts!(
        "hab_sup_rpc_call_total",
        "Total number of RPC calls",
        labels!{"command" => "svc_load",}
    )).unwrap();
    static ref SVC_LOAD_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "hab_sup_rpc_call_request_duration_seconds",
        "The latency for RPC calls",
        &["svc_load"]
    ).unwrap();
    static ref SVC_UNLOAD_COUNTER: Counter = register_counter!(opts!(
        "hab_sup_rpc_call_total",
        "Total number of RPC calls",
        labels!{"command" => "svc_unload",}
    )).unwrap();
    static ref SVC_UNLOAD_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "hab_sup_rpc_call_request_duration_seconds",
        "The latency for RPC calls",
        &["svc_unload"]
    ).unwrap();
    static ref SVC_START_COUNTER: Counter = register_counter!(opts!(
        "hab_sup_rpc_call_total",
        "Total number of RPC calls",
        labels!{"command" => "svc_start",}
    )).unwrap();
    static ref SVC_START_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "hab_sup_rpc_call_request_duration_seconds",
        "The latency for RPC calls",
        &["svc_start"]
    ).unwrap();
    static ref SVC_STOP_COUNTER: Counter = register_counter!(opts!(
        "hab_sup_rpc_call_total",
        "Total number of RPC calls",
        labels!{"command" => "svc_stop",}
    )).unwrap();
    static ref SVC_STOP_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "hab_sup_rpc_call_request_duration_seconds",
        "The latency for RPC calls",
        &["svc_stop"]
    ).unwrap();
    static ref SVC_STATUS_COUNTER: Counter = register_counter!(opts!(
        "hab_sup_rpc_call_total",
        "Total number of RPC calls",
        labels!{"command" => "svc_status",}
    )).unwrap();
    static ref SVC_STATUS_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "hab_sup_rpc_call_request_duration_seconds",
        "The latency for RPC calls",
        &["svc_status"]
    ).unwrap();
}
