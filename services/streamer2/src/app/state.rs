use std::sync::Arc;

use tokio::sync::mpsc::Sender;
use uuid::Uuid;

use crate::{commands::ClientGeneratedTask, services::client_manager::ClientManager};

#[derive(Clone)]
pub struct AppState {
    instance_id: String,
    clients: Arc<ClientManager>,
}

impl AppState {
    pub fn new(task_sender: Sender<ClientGeneratedTask>) -> Self {
        AppState { 
            instance_id: Uuid::new_v4().to_string(),
            clients: Arc::new(ClientManager::new(task_sender)),
         }
    }

    pub fn instance_id(&self) -> &str {
        &self.instance_id
    }

    pub fn clients(&self) -> &ClientManager {
        &self.clients
    }
}
