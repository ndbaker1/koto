//! module for a basic session which holds clients and maintains active status

use std::{collections::HashMap, sync::Arc};

use futures::stream::SplitSink;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct SessionManager<Socket, Message, SessionExt> {
    pub clients: Arc<Mutex<HashMap<String, Client<Socket, Message>>>>,
    pub sessions: Arc<Mutex<HashMap<String, Session<SessionExt>>>>,
}

impl<T1, T2, T3> Default for SessionManager<T1, T2, T3> {
    fn default() -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[derive(Debug)]
pub struct Client<Socket, Message> {
    pub id: String,
    pub sender: Arc<Mutex<SplitSink<Socket, Message>>>,
}

#[derive(Debug)]
pub struct Session<Ext> {
    pub id: String,
    pub clients: HashMap<String, bool>,
    pub ext: Ext,
}

impl<Ext: Default> Session<Ext> {
    pub fn new(id: String) -> Self {
        Self {
            id,
            clients: HashMap::default(),
            ext: Ext::default(),
        }
    }

    pub fn set_client_status(&mut self, client_id: &str, active: bool) {
        if self.clients.contains_key(client_id) {
            self.clients.insert(client_id.to_string(), active);
        }
    }

    pub fn active_client_set(&self) -> Vec<&String> {
        self.clients
            .iter()
            .filter_map(|(client, status)| if *status { Some(client) } else { None })
            .collect()
    }
}

/// Generates a String of given length using characters that are valid for Session IDs
///
/// This should effectively resolve to Session uniqueness when the length is
/// greater than a value like 4 for a plausable number of concurrent sessions
pub(crate) fn generate_session_id<const ID_LENGTH: usize>() -> String {
    nanoid::nanoid!(
        ID_LENGTH,
        &[
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ]
    )
}
