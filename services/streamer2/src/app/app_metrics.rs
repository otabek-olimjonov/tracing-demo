use std::{future::ready, time::Duration};

use axum::{routing::get, Router};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use tokio::sync::mpsc::Receiver;
use tower_http::timeout::TimeoutLayer;

fn app_metrics() -> Router {
    let metrics_handler = setup_prometheus();

    Router::new()
        .route("/metrics", get(move || ready(metrics_handler.render())))
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
}

fn setup_prometheus() -> PrometheusHandle {
    const EXPONENTIAL_SECONDS: &[f64] = &[
        0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
    ];

    PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full("http_requests_duration_seconds".to_string()),
            EXPONENTIAL_SECONDS,
        )
        .unwrap()
        .set_buckets_for_metric(
            Matcher::Full("websocket_connected_duration_seconds".to_string()),
            EXPONENTIAL_SECONDS,
        )
        .unwrap()
        .install_recorder()
        .unwrap()
}

pub async fn run_app(shutdown: Receiver<()>) {
    let app = app_metrics();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await;

    if let Err(err) = listener {
        tracing::error!(?err, "❌ could not create listener");
        return;
    }

    let listener = listener.unwrap();

    tracing::info!("🟢 server listening on {}", listener.local_addr().unwrap());

    if let Err(err) = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(shutdown))
        .await
    {
        tracing::error!(?err, "❌ server error");
    }

    tracing::info!("🛑 stopped");
}

async fn shutdown_signal(mut ch: Receiver<()>) {
    while let Some(_) = ch.recv().await {}
}
