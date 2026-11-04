use std::sync::Arc;

use crate::core::base::BaseContext;
use crate::core::contract::event::MessageEvent;
use crate::core::session::credential::SessionProfile;
use crate::core::transport::connection::SocketConnection;
use crate::endpoints::auth::AuthEndpoint;
use crate::endpoints::channel::ChannelEndpoint;
use crate::endpoints::identity::IdentityEndpoint;
use crate::endpoints::storage::StorageEndpoint;
use crate::endpoints::timeline::TimelineEndpoint;
use crate::endpoints::transmission::TransmissionEndpoint;

#[derive(Clone)]
pub struct RApiClient {
    pub connection: Arc<SocketConnection>,
    pub channel: ChannelEndpoint,
    pub identity: IdentityEndpoint,
    pub transmission: TransmissionEndpoint,
    pub timeline: TimelineEndpoint,
    pub storage: StorageEndpoint,
    pub auth: AuthEndpoint,
}

impl RApiClient {
    pub fn new(session_path: &str) -> Result<Self, String> {
        let profile = SessionProfile::load_from_file(session_path)?;
        Self::from_profile(&profile)
    }

    pub fn from_profile(profile: &SessionProfile) -> Result<Self, String> {
        let conn = SocketConnection::new(profile)?;
        let ctx = Arc::new(BaseContext::new(Some(conn.clone())));

        Ok(Self {
            connection: conn,
            channel: ChannelEndpoint::new(ctx.clone()),
            identity: IdentityEndpoint::new(ctx.clone()),
            transmission: TransmissionEndpoint::new(ctx.clone()),
            timeline: TimelineEndpoint::new(ctx.clone()),
            storage: StorageEndpoint::new(ctx.clone()),
            auth: AuthEndpoint::new(ctx),
        })
    }

    pub fn connect(&self) -> bool {
        self.connection.connect()
    }

    pub fn is_connected(&self) -> bool {
        self.connection.is_connected()
    }

    pub fn close(&self) {
        self.connection.close();
    }

    pub fn on_message<F>(&self, handler: F)
    where
        F: Fn(MessageEvent) + Send + Sync + 'static,
    {
        self.connection.on_message(handler);
    }

    pub fn on_event<F>(&self, event: &str, handler: F)
    where
        F: Fn(serde_json::Value) + Send + Sync + 'static,
    {
        self.connection.dispatcher.add_event_handler(event, handler);
    }
}
