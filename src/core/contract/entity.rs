use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreadType {
    Group = 1,
    User = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gender {
    Male = 0,
    Female = 1,
    Unknown = 2,
}

impl Gender {
    pub fn display_string(&self) -> &'static str {
        match self {
            Gender::Male => "Nam",
            Gender::Female => "Nữ",
            Gender::Unknown => "Không công khai",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: i64,
    pub display_name: String,
    pub name: String,
    pub alias: Option<String>,
    pub username: Option<String>,
    pub gender: Gender,
    pub phone: Option<String>,
    pub bio: Option<String>,
    pub cover: Option<String>,
    pub avatar: Option<String>,
    pub is_friend: bool,
    pub is_blocked: bool,
    pub is_active: bool,
    pub raw_data: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Friend {
    pub user_id: i64,
    pub name: String,
    pub alias: String,
    pub avatar: String,
    pub is_blocked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMember {
    pub user_id: i64,
    pub name: String,
    pub display_name: String,
    pub avatar: Option<String>,
    pub role: i32,
    pub joined_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupInfo {
    pub group_id: i64,
    pub name: String,
    pub desc: String,
    pub avatar: String,
    pub creator_id: i64,
    pub total_members: usize,
    pub members: Vec<GroupMember>,
    pub admins: Vec<i64>,
    pub link: Option<String>,
    pub setting: Option<HashMap<String, serde_json::Value>>,
    pub raw: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub owner_id: i64,
    pub message_id: i64,
    pub content: String,
    pub ttl: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub message_id: i64,
    pub cli_msg_id: i64,
    pub group_id: Option<i64>,
    pub user_id: Option<i64>,
    pub sender_id: i64,
    pub content: String,
    pub msg_type: i32,
    pub timestamp: i64,
    pub quote: Option<Quote>,
    pub extra: Option<HashMap<String, serde_json::Value>>,
}
