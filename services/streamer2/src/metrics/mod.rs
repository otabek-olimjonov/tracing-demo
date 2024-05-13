

pub fn record_websocket_connect() {
    metrics::gauge!("websocket_connections_total").increment(1);
}

pub fn record_websocket_duration(client_id: &str, duration: f64) {
    let labels = [
        ("client_id", client_id.to_string()),
    ];

    metrics::histogram!("websocket_connected_duration_seconds", &labels).record(duration);
}

pub fn record_websocket_disconnect() {
    metrics::gauge!("websocket_connections_total").decrement(1);
}