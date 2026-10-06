//! RSpeek client state and connection core.
//!
//! NO UI dependencies (no egui, no ratatui) allowed here.
//! Must compile cleanly on both native targets and `wasm32-unknown-unknown`.

use rspeek_shared::feature::Feature;
use rspeek_shared::id::{ChannelId, ServerId, UserId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting { endpoint: String },
    Connected { endpoint: String },
    Reconnecting { attempt: u32 },
}

#[derive(Debug, Clone)]
pub struct UserSession {
    pub user_id: UserId,
    pub username: String,
    pub active_features: Vec<Feature>,
}

#[derive(Debug, Clone)]
pub struct ServerSummary {
    pub id: ServerId,
    pub name: String,
    pub channels: Vec<ChannelSummary>,
}

#[derive(Debug, Clone)]
pub struct ChannelSummary {
    pub id: ChannelId,
    pub name: String,
    pub is_voice: bool,
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub id: u64,
    pub author_name: String,
    pub content: String,
    pub timestamp_ms: u64,
}

#[derive(Debug, Clone)]
pub struct ClientState {
    pub connection_status: ConnectionStatus,
    pub session: Option<UserSession>,
    pub servers: Vec<ServerSummary>,
    pub selected_server: Option<ServerId>,
    pub selected_channel: Option<ChannelId>,
    pub messages: Vec<ChatMessage>,
}

impl Default for ClientState {
    fn default() -> Self {
        Self {
            connection_status: ConnectionStatus::Disconnected,
            session: None,
            servers: Vec::new(),
            selected_server: None,
            selected_channel: None,
            messages: Vec::new(),
        }
    }
}

impl ClientState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_connected(&self) -> bool {
        matches!(self.connection_status, ConnectionStatus::Connected { .. })
    }

    pub fn has_feature(&self, feature: Feature) -> bool {
        self.session
            .as_ref()
            .map(|s| s.active_features.contains(&feature))
            .unwrap_or(false)
    }
}
