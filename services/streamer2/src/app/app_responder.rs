use redis::{
    aio::MultiplexedConnection,
    streams::{StreamReadOptions, StreamReadReply},
    AsyncCommands, FromRedisValue, RedisResult,
};
use serde::Deserialize;
use tokio::sync::mpsc::Receiver;

use crate::commands::ClientMessage;

use super::state::AppState;

#[derive(Debug, Deserialize)]
pub struct StreamingResponse {
    pub client_id: String,
    pub translated: String,
}

async fn app_streaming_response(state: AppState, redis_url: &str, mut shutdown: Receiver<()>) {
    let client = redis::Client::open(redis_url).unwrap();

    let mut con = client.get_multiplexed_async_connection().await.unwrap();
    let opts = StreamReadOptions::default().block(500).count(20);

    let stream_id = format!("streaming_tasks_queue:responses:{}", state.instance_id());
    let mut read_cursor = String::from("$");

    tracing::info!("🟢 start processing response queue");

    'main: loop {
        tokio::select! {
            read_result = read_response_stream(&mut con, &stream_id, &read_cursor, &opts) => {
                match read_result {
                    Ok(stream_read_reply) => {
                        if let Some(last_read_id) = process_stream_read_reply(&state, stream_read_reply).await {
                            read_cursor = last_read_id;
                        }
                    },
                    Err(err) => {
                        tracing::error!(?err, stream_id, read_cursor, "🛑 stopping because of error");
                        break 'main;
                    },
                }
            }
            None = shutdown.recv() => break 'main,
        }
    }

    tracing::info!("🛑 stopped");
}

async fn read_response_stream(
    con: &mut MultiplexedConnection,
    sid: &String,
    rid: &String,
    opts: &StreamReadOptions,
) -> RedisResult<StreamReadReply> {
    // TODO: is it possible to avoid cloning the stream_id and read_id here?
    con.xread_options(&[sid.clone()], &[rid.clone()], &opts)
        .await
}

async fn process_stream_read_reply(
    state: &AppState,
    stream_read_reply: StreamReadReply,
) -> Option<String> {
    for stream_id in stream_read_reply
        .keys
        .iter()
        .flat_map(|stream_key| &stream_key.ids)
    {
        tracing::trace!(?stream_id.id, ?stream_id.map, "process message from redis stream");

        let response = stream_id
            .map
            .get("json")
            .map(|json| String::from_redis_value(json).ok())
            .flatten()
            .and_then(|response_json| {
                serde_json::from_str::<StreamingResponse>(&response_json).ok()
            });

        if let Some(response) = response {
            state
                .clients()
                .send_message_to_client(
                    &response.client_id,
                    ClientMessage::SendTextResponse(response.translated),
                )
                .await
        } else {
            tracing::error!(?stream_id.id, ?stream_id.map, "couldn't parse response")
        }
    }

    get_last_message_id(stream_read_reply)
}

fn get_last_message_id(reply: StreamReadReply) -> Option<String> {
    reply
        .keys
        .first()
        .map(|key| key.ids.last().map(|msg| msg.id.clone()))
        .flatten()
}

pub async fn run_app(state: AppState, redis_url: String, shutdown: Receiver<()>) {
    app_streaming_response(state, &redis_url, shutdown).await
}
