use clap::Parser;

mod app;
mod commands;
mod metrics;
mod middleware;
mod services;
mod shutdown;
mod trace_utils;

use app::{app_metrics, app_responder, app_streaming, app_taskgen, state::AppState};
use tokio::sync::mpsc::channel;

#[derive(Debug, Parser)]
struct Configuration {
    #[clap(env)]
    redis_url: String,
}

#[tokio::main]
async fn main() {
    let config = Configuration::parse();

    trace_utils::setup_tracing("streamer2");

    let (task_app_streaming, task_app_metrics, task_app_responder, task_app_taskgen) =
        run_applications(&config.redis_url).await;

    // NOTE: Wait for the applications to gracefully exit

    _ = tokio::join!(
        task_app_streaming,
        task_app_metrics,
        task_app_responder,
        task_app_taskgen
    )
}

async fn run_applications(
    redis_url: &str,
) -> (
    tokio::task::JoinHandle<()>,
    tokio::task::JoinHandle<()>,
    tokio::task::JoinHandle<()>,
    tokio::task::JoinHandle<()>,
) {
    let (task_sender, task_receiver) = channel(32);

    let state = AppState::new(task_sender);

    let (app_streaming_send, app_streaming_recv) = channel::<()>(1);
    let task_app_streaming =
        tokio::spawn(app_streaming::run_app(state.clone(), app_streaming_recv));

    let (app_metrics_send, app_metrics_recv) = channel::<()>(1);
    let task_app_metrics = tokio::spawn(app_metrics::run_app(app_metrics_recv));

    let (app_responder_send, app_responder_recv) = channel::<()>(1);
    let task_app_responder = tokio::spawn(app_responder::run_app(
        state.clone(),
        redis_url.to_string(),
        app_responder_recv,
    ));

    let (app_taskgen_send, app_taskgen_recv) = channel::<()>(1);
    let task_app_taskgen = tokio::spawn(app_taskgen::run_app(
        task_receiver,
        state.instance_id().to_string(),
        redis_url.to_string(),
        app_taskgen_recv,
    ));

    tokio::select! {
        _ = app_streaming_send.closed() => { tracing::warn!("⚠️  app_streaming stopped unexpectedly") },
        _ = app_metrics_send.closed() => { tracing::warn!("⚠️  app_metrics stopped unexpectedly") },
        _ = app_responder_send.closed() => { tracing::warn!("⚠️  app_responder stopped unexpectedly") },
        _ = app_taskgen_send.closed() => { tracing::warn!("⚠️  app_taskgen stopped unexpectedly") },
        _ = shutdown::shutdown_signal() => { }
    };

    (
        task_app_streaming,
        task_app_metrics,
        task_app_responder,
        task_app_taskgen,
    )
}
