// Note that this file is meant to be included, not used standalone. See src/http_gateway.rs for
// more details.
lazy_static! {
    static ref HTTP_BUTTERFLY_COUNTER: Counter = register_counter!(opts!(
        "hab_sup_http_gateway_total",
        "Total number of HTTP gateway requests",
        labels!{"endpoint" => "butterfly",}
    )).unwrap();
    static ref HTTP_BUTTERFLY_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "hab_sup_http_gateway_request_duration_seconds",
        "The latency for HTTP gateway requests",
        &["butterfly"]
    ).unwrap();
    static ref HTTP_CENSUS_COUNTER: Counter = register_counter!(opts!(
        "hab_sup_http_gateway_total",
        "Total number of HTTP gateway requests",
        labels!{"endpoint" => "census",}
    )).unwrap();
    static ref HTTP_CENSUS_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "hab_sup_http_gateway_request_duration_seconds",
        "The latency for HTTP gateway requests",
        &["census"]
    ).unwrap();
    static ref HTTP_SERVICES_COUNTER: Counter = register_counter!(opts!(
        "hab_sup_http_gateway_total",
        "Total number of HTTP gateway requests",
        labels!{"endpoint" => "services",}
    )).unwrap();
    static ref HTTP_SERVICES_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "hab_sup_http_gateway_request_duration_seconds",
        "The latency for HTTP gateway requests",
        &["services"]
    ).unwrap();
    static ref HTTP_CONFIG_COUNTER: Counter = register_counter!(opts!(
        "hab_sup_http_gateway_total",
        "Total number of HTTP gateway requests",
        labels!{"endpoint" => "config",}
    )).unwrap();
    static ref HTTP_CONFIG_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "hab_sup_http_gateway_request_duration_seconds",
        "The latency for HTTP gateway requests",
        &["config"]
    ).unwrap();
    static ref HTTP_HEALTH_COUNTER: Counter = register_counter!(opts!(
        "hab_sup_http_gateway_total",
        "Total number of HTTP gateway requests",
        labels!{"endpoint" => "health",}
    )).unwrap();
    static ref HTTP_HEALTH_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "hab_sup_http_gateway_request_duration_seconds",
        "The latency for HTTP gateway requests",
        &["health"]
    ).unwrap();
    static ref HTTP_SERVICE_COUNTER: Counter = register_counter!(opts!(
        "hab_sup_http_gateway_total",
        "Total number of HTTP gateway requests",
        labels!{"endpoint" => "service",}
    )).unwrap();
    static ref HTTP_SERVICE_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "hab_sup_http_gateway_request_duration_seconds",
        "The latency for HTTP gateway requests",
        &["service"]
    ).unwrap();
    static ref HTTP_METRICS_COUNTER: Counter = register_counter!(opts!(
        "hab_sup_http_gateway_total",
        "Total number of HTTP gateway requests",
        labels!{"endpoint" => "metrics",}
    )).unwrap();
    static ref HTTP_METRICS_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "hab_sup_http_gateway_request_duration_seconds",
        "The latency for HTTP gateway requests",
        &["metrics"]
    ).unwrap();
    static ref HTTP_DOC_COUNTER: Counter = register_counter!(opts!(
        "hab_sup_http_gateway_total",
        "Total number of HTTP gateway requests",
        labels!{"endpoint" => "doc",}
    )).unwrap();
    static ref HTTP_DOC_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "hab_sup_http_gateway_request_duration_seconds",
        "The latency for HTTP gateway requests",
        &["doc"]
    ).unwrap();
}
