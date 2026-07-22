use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ContentType {
    Text,
    Email,
    Phone,
    Url,
    Address,
    Date,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueueItem {
    pub id: String,
    pub content: String,
    pub content_type: ContentType,
    pub added_at: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Slot {
    pub id: String,
    pub name: String,
    pub content: String,
    pub content_type: ContentType,
    pub updated_at: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProfileItem {
    pub id: String,
    pub label: String,
    pub content: String,
    pub content_type: ContentType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub items: Vec<ProfileItem>,
    pub created_at: u64,
}

pub struct AppState {
    pub queue: Mutex<Vec<QueueItem>>,
    pub cursor: Mutex<usize>,
    pub slots: Mutex<Vec<Slot>>,
    pub profiles: Mutex<Vec<Profile>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(Vec::new()),
            cursor: Mutex::new(0),
            slots: Mutex::new(Vec::new()),
            profiles: Mutex::new(Vec::new()),
        }
    }
}
