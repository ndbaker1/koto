//! Messages

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub enum ClientMessage {
    LeaveSession,
    CreateSession,
    JoinSession(String),
}

#[derive(Serialize)]
pub enum ServerMessage {}
