use std::{borrow::Cow, ops::ControlFlow, time::Duration};

use axum::extract::ws::{close_code::RESTART, CloseFrame, Message, WebSocket};
use futures::{stream::SplitSink, SinkExt, StreamExt};
use tokio::{
    sync::mpsc::{Receiver, Sender},
    time::{interval, Instant},
};

use crate::{commands::{ClientGeneratedTask, ClientMessage, Command, StreamingRequest}, metrics::{record_websocket_connect, record_websocket_disconnect, record_websocket_duration}};

pub async fn client_socket_io(
    socket: WebSocket,
    client_id: String,
    task_sender: Sender<ClientGeneratedTask>,
    client_sender: Sender<ClientMessage>,
    mut client_receiver: Receiver<ClientMessage>,
) {
    tracing::debug!(?client_id, "📨 start processing socket io");

    let mut last_recorded_time = Instant::now();

    record_websocket_connect();
    record_websocket_duration(&client_id, 0.0);

    let (mut sender, mut receiver) = socket.split();

    // NOTE: Consumes `ClientMessage` messages from the channel and send them across the
    // socket. If the channel or socket is closed this task is finished. The task will return
    // any errors as a task result so it can be handled or logged appropriately.
    let mut send_message_task = tokio::spawn(async move {
        while let Some(message) = client_receiver.recv().await {
            if let Err(err) = process_client_message(message, &mut sender).await {
                return Err(err);
            }
        }

        tracing::debug!("client message channel closed, server shutting down");

        if let Err(err) = sender
            .send(Message::Close(Some(CloseFrame {
                code: RESTART,
                reason: Cow::from("Adios, amigo!"),
            })))
            .await
        {
            return Err(err);
        }

        Ok(())
    });

    let cid = client_id.clone();

    let mut recv_message_task = tokio::spawn(async move {
        while let Some(Ok(message)) = receiver.next().await {
            if process_socket_message(message, &cid, &task_sender)
                .await
                .is_break()
            {
                break;
            }
        }
    });

    let cid = client_id.clone();
    let mut ping_socket_task = tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(5));

        while let Ok(_) = client_sender.send(ClientMessage::SendPing).await {
            record_websocket_duration(&cid, last_recorded_time.elapsed().as_secs_f64());
            last_recorded_time = Instant::now();

            interval.tick().await;
        }
    });

    tokio::select! {
        recv = (&mut recv_message_task) => {
            match recv {
                Err(err) => tracing::error!(?client_id, ?err, "error receiving messages"),
                _ => {}
            }

            send_message_task.abort();
            ping_socket_task.abort();
        }
        send = (&mut send_message_task) => {
            match send {
                Err(err) => tracing::error!(?client_id, ?err, "error sending messages"),
                _ => {}
            }

            recv_message_task.abort();
            ping_socket_task.abort();
        },
        ping = (&mut ping_socket_task) => {
            match ping {
                Err(err) => tracing::error!(?client_id, ?err, "error sending ping"),
                _ => {}
            }

            send_message_task.abort();
            recv_message_task.abort();
        }
    }

    record_websocket_disconnect();

    tracing::debug!(?client_id, "📨 stop processing socket io");

}

#[tracing::instrument]
async fn process_client_message(
    message: ClientMessage,
    sender: &mut SplitSink<WebSocket, Message>,
) -> Result<(), axum::Error> {
    match message {
        ClientMessage::SendTextResponse(text) => sender.send(Message::Text(text)).await,
        ClientMessage::SendPing => sender.send(Message::Ping(vec![])).await,
    }
}

#[tracing::instrument]
async fn process_socket_message(
    message: Message,
    client_id: &String,
    task_sender: &Sender<ClientGeneratedTask>,
) -> ControlFlow<(), ()> {
    match message {
        Message::Text(text) => {
            tracing::trace!(client_id, "📨 received: {}", text);

            match serde_json::from_str::<StreamingRequest>(&text) {
                Ok(request) => process_command(request.command, client_id, task_sender).await,
                Err(err) => tracing::error!(?err, client_id, "invalid message from client"),
            }
        }
        Message::Binary(blob) => {
            tracing::trace!(client_id, "📨 received: {} bytes: {:?}", blob.len(), blob)
        }
        Message::Ping(payload) => tracing::trace!(client_id, "➡️ received ping: {:?}", payload),
        Message::Pong(payload) => tracing::trace!(client_id, "⬅️ received pong: {:?}", payload),
        Message::Close(close_frame) => {
            if let Some(close_frame) = close_frame {
                tracing::trace!(client_id, close_frame.code, ?close_frame.reason, "⛔ socket closing");
            } else {
                tracing::trace!(client_id, "⛔ socket closing");
            }

            return ControlFlow::Break(());
        }
    }

    ControlFlow::Continue(())
}

#[tracing::instrument]
async fn process_command(
    command: Command,
    client_id: &str,
    task_sender: &Sender<ClientGeneratedTask>,
) {
    tracing::debug!(?command, client_id, "process command");

    let task = ClientGeneratedTask {
        client_id: client_id.to_string(),
        command,
    };

    if let Err(err) = task_sender.send(task).await {
        tracing::error!(?err, "couldn't send task");
    }
}
