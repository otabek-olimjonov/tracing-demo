use axum::extract::ws::WebSocket;
use std::{collections::HashMap, net::SocketAddr};
use tokio::sync::{
    mpsc::{channel, Sender},
    Mutex,
};
use uuid::Uuid;

use crate::{
    commands::{ClientGeneratedTask, ClientMessage},
    services::client_socket_io::client_socket_io,
};

#[derive(Debug)]
pub struct ClientManager {
    client_message_channels: Mutex<HashMap<String, Sender<ClientMessage>>>,
    task_sender: Sender<ClientGeneratedTask>,
}

impl ClientManager {
    pub fn new(task_sender: Sender<ClientGeneratedTask>) -> Self {
        ClientManager {
            client_message_channels: Mutex::new(HashMap::new()),
            task_sender,
        }
    }

    #[tracing::instrument(skip(self, socket))]
    pub async fn handle_connection(&self, socket: WebSocket, addr: SocketAddr) {
        let client_id = Uuid::new_v4().to_string();
        tracing::info!(?addr, ?client_id, "➕ add client");

        let (client_sender, client_receiver) = channel::<ClientMessage>(64);

        self.add_message_channel(&client_id, client_sender.clone())
            .await;

        client_socket_io(
            socket,
            client_id.clone(),
            self.task_sender.clone(),
            client_sender,
            client_receiver,
        )
        .await;

        tracing::info!(?addr, ?client_id, "➖ remove client");
        self.remove_message_channel(&client_id).await;
    }

    // TODO: lockless repository to avoid potential deadlocks

    #[tracing::instrument]
    async fn add_message_channel(&self, id: &str, sender: Sender<ClientMessage>) {
        self.client_message_channels
            .lock()
            .await
            .insert(id.to_string(), sender);
    }

    #[tracing::instrument]
    async fn remove_message_channel(&self, client_id: &str) {
        self.client_message_channels.lock().await.remove(client_id);
    }

    #[tracing::instrument]
    pub async fn send_message_to_client(&self, client_id: &str, message: ClientMessage) {
        if let Some(client) = self.client_message_channels.lock().await.get(client_id) {
            tracing::debug!(client_id, ?message, "send message to client");
            if let Err(err) = client.send(message).await {
                tracing::error!(?err, client_id, "client is disconnected");
            }
        } else {
            tracing::error!(client_id, ?message, "couldn't find client");
        }
    }
}
