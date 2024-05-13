use std::collections::HashMap;

use redis::{aio::MultiplexedConnection, AsyncCommands};
use tokio::sync::mpsc::Receiver;

use opentelemetry::global as otel;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::commands::{ClientGeneratedTask, InstanceGeneratedTask};

const SK_TASK_QUEUE: &'static str = "streaming_tasks_queue:requests";

fn span_context_json() -> String {
    let ctx = Span::current().context();

    let mut injected_context = HashMap::new();

    otel::get_text_map_propagator(|prop| prop.inject_context(&ctx, &mut injected_context));

    serde_json::to_string(&injected_context).unwrap()
}

#[tracing::instrument]
async fn enqueue_task(
    con: &mut MultiplexedConnection,
    task: &InstanceGeneratedTask,
) -> Result<(), redis::RedisError> {
    let span_ctx = span_context_json();
    let task_json = serde_json::to_string(&task).unwrap();

    con.xadd(
        SK_TASK_QUEUE,
        "*",
        &[("task", task_json), ("ctx", span_ctx)],
    )
    .await
}

async fn app_taskgen(
    mut task_receiver: Receiver<ClientGeneratedTask>,
    instance_id: &str,
    redis_url: &str,
    mut shutdown: Receiver<()>,
) {
    let client = redis::Client::open(redis_url).unwrap();

    let mut con = client.get_multiplexed_async_connection().await.unwrap();

    tracing::info!("🟢 start processing tasks");

    'main: loop {
        tokio::select! {
            Some(task) = task_receiver.recv() => {
                tracing::debug!(?task, "enqueue task");

                let instance_generated_task = InstanceGeneratedTask {
                    instance_id: instance_id.to_string(),
                    inner: task,
                };

                if let Err(err) = enqueue_task(&mut con, &instance_generated_task).await {
                    tracing::error!(?err, "could not add task to queue");
                    break 'main;
                }
            }
            None = shutdown.recv() => break 'main,
        }
    }

    tracing::info!("🛑 stopped");
}

pub async fn run_app(
    task_receiver: Receiver<ClientGeneratedTask>,
    instance_id: String,
    redis_url: String,
    shutdown: Receiver<()>,
) {
    app_taskgen(task_receiver, &instance_id, &redis_url, shutdown).await
}
