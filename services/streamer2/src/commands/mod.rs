use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum Command {
    #[serde(rename = "translate")]
    Translate(String),
}

#[derive(Debug, Deserialize)]
pub struct StreamingRequest {
    pub command: Command
}

#[derive(Debug, Serialize)]
pub struct InstanceGeneratedTask {
    pub instance_id: String,

    #[serde(flatten)]
    pub inner: ClientGeneratedTask,
}

#[derive(Debug, Serialize)]
pub struct ClientGeneratedTask {
    pub client_id: String,
    pub command: Command,
}

#[derive(Debug)]
pub enum ClientMessage {
    SendPing,
    SendTextResponse(String)
}
