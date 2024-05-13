use std::{net::SocketAddr, time::Duration};

use axum::{
    extract::{
        ws::WebSocket,
        ConnectInfo, State, WebSocketUpgrade,
    },
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_extra::{headers, TypedHeader};
use tokio::{
    sync::mpsc::Receiver,
    time::sleep,
};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

use crate::
    middleware::metrics::middleware_metrics
;

use super::state::AppState;

fn app_streaming(state: AppState) -> Router {
    Router::new()
        .route("/streamer2/v1/status", get(status))
        .route("/streamer2/v1/translate", get(translate_handler))
        .route("/streamer2/v1/faulty", get(faulty_handler))
        .route("/streamer2/v1/slow", get(slow_handler))
        .route_layer(middleware::from_fn(middleware_metrics))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .with_state(state)
}

#[tracing::instrument]
async fn status() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

#[tracing::instrument]
async fn faulty_handler() -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "")
}

#[tracing::instrument]
async fn slow_handler() -> impl IntoResponse {
    sleep(Duration::from_millis(800)).await;

    (StatusCode::OK, "SLOW RESPONSE")
}

#[tracing::instrument(skip(state))]
async fn translate_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown")
    };

    tracing::info!(?addr, user_agent, "⚡ client connection request");

    ws.on_upgrade(move |socket| handle_connection(socket, addr, state))
}

async fn handle_connection(socket: WebSocket, addr: SocketAddr, state: AppState) {
    state.clients().handle_connection(socket, addr).await;
}

pub async fn run_app(state: AppState, shutdown: Receiver<()>) {
    let app = app_streaming(state).into_make_service_with_connect_info::<SocketAddr>();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await;

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
