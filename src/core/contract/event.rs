use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::entity::{Message, ThreadType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEvent {
    pub message_id: i64,
    pub cli_msg_id: i64,
    pub sender_id: i64,
    pub sender_display: Option<String>,
    pub user_id: Option<i64>,
    pub group_id: Option<i64>,
    pub is_group: bool,
    pub is_self: bool,
    pub text: String,
    pub content: String,
    pub msg_type: String,
    pub ttl: i32,
    pub timestamp: i64,
    pub quote: Option<HashMap<String, serde_json::Value>>,
    pub raw: Option<HashMap<String, serde_json::Value>>,
    pub message: Option<Message>,
}

impl MessageEvent {
    pub fn thread_id(&self) -> i64 {
        if self.is_group {
            self.group_id.unwrap_or(0)
        } else {
            self.user_id.unwrap_or(self.sender_id)
        }
    }

    pub fn thread_type(&self) -> ThreadType {
        if self.is_group {
            ThreadType::Group
        } else {
            ThreadType::User
        }
    }
}
