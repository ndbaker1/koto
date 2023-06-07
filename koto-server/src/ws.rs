use std::sync::Arc;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        Query, WebSocketUpgrade,
    },
    response::IntoResponse,
    Extension,
};
use axum_macros::debug_handler;
use futures::{
    stream::{SplitSink, SplitStream},
    StreamExt,
};
use koto_core::Game;
use serde::Deserialize;
use tokio::sync::Mutex;
use tracing::info;

use crate::{
    messages::ClientMessage,
    session::{generate_session_id, Client, Session},
    SharedServerState,
};

#[derive(Deserialize)]
pub struct ClientConnectionParams {
    id: String,
}

#[debug_handler]
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    Query(ClientConnectionParams { id: connection_id }): Query<ClientConnectionParams>,
    Extension(state): Extension<SharedServerState>,
) -> impl IntoResponse {
    if state.clients.lock().await.get(&connection_id).is_none() {
        ws.on_upgrade(move |socket| {
            // By splitting we can send and receive at the same time.
            let (sender, receiver) = socket.split();

            ClientRunner::new(state, Arc::new(Mutex::new(sender)), receiver, connection_id).run()
        })
    } else {
        format!("User [{}] Already Connected", connection_id).into_response()
    }
}

/// Abstraction around a client, which gets run in its own async task.
struct ClientRunner<State> {
    state: State,
    sender: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    receiver: SplitStream<WebSocket>,
    connection_id: String,
    cached_session: Option<String>,
}

impl<State> ClientRunner<State> {
    pub fn new(
        state: State,
        sender: Arc<Mutex<SplitSink<WebSocket, Message>>>,
        receiver: SplitStream<WebSocket>,
        connection_id: String,
    ) -> Self {
        Self {
            connection_id,
            sender,
            receiver,
            state,
            cached_session: None,
        }
    }
}

impl ClientRunner<SharedServerState> {
    async fn run(mut self) {
        self.connect_client().await;

        // Loop until a text message is found.
        while let Some(Ok(message)) = self.receiver.next().await {
            match message {
                Message::Text(text) => {
                    if let Ok(event) = serde_json::from_str::<ClientMessage>(&text) {
                        self.process_event(event).await;
                    }
                }
                // unhandled other cases
                _ => info!("{:?}", message),
            }
        }

        self.disconnect_client().await;
    }

    async fn connect_client(&self) {
        self.state.clients.lock().await.insert(
            self.connection_id.clone(),
            Client {
                id: self.connection_id.clone(),
                sender: self.sender.clone(),
            },
        );
    }

    async fn disconnect_client(&self) {
        self.state.clients.lock().await.remove(&self.connection_id);

        if let Some(session_id) = &self.cached_session {
            self.leave_session(session_id).await;
        };
    }

    async fn leave_session(&self, session_id: &str) {
        // remove the client from the session and check if the session become empty
        let empty = if let Some(session) = self.state.sessions.lock().await.get_mut(session_id) {
            session.set_client_status(&self.connection_id, false);
            session.active_client_set().is_empty()
        } else {
            false
        };

        if empty {
            self.state.sessions.lock().await.remove(session_id);
        }
    }

    async fn process_event(&mut self, event: ClientMessage) {
        match event {
            ClientMessage::CreateSession => {
                let new_session = self.create_session(None).await;

                self.state
                    .sessions
                    .lock()
                    .await
                    .insert(new_session.id.clone(), new_session);
            }
            ClientMessage::JoinSession(session_id) => {
                let mut lock = self.state.sessions.lock().await;

                if let Some(session) = lock.get_mut(&session_id) {
                    session.clients.insert(self.connection_id.clone(), true);
                    self.cached_session = Some(session.id.clone());
                } else {
                    drop(lock);

                    let mut session = self.create_session(Some(session_id)).await;

                    session.clients.insert(self.connection_id.clone(), true);
                    self.cached_session = Some(session.id.clone());

                    self.state
                        .sessions
                        .lock()
                        .await
                        .insert(session.id.clone(), session);
                }
            }
            ClientMessage::LeaveSession => {
                if let Some(session_id) = &self.cached_session {
                    self.leave_session(session_id).await;
                }
            }
        }
    }

    async fn create_session(&mut self, reserved_id: Option<String>) -> Session<Game> {
        let session_id = reserved_id.unwrap_or_else(|| generate_session_id::<5>());
        let mut session = Session::<Game>::new(session_id.clone());

        session.clients.insert(self.connection_id.clone(), true);
        info!("new session [{}] created", session_id);
        self.cached_session = Some(session_id);

        session
    }
}
