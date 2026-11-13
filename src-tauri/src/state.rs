use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::sync::atomic::AtomicBool;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

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
#[serde(rename_all = "camelCase")]
pub struct QueueItem {
    pub id: String,
    pub content: String,
    #[serde(rename = "type")]
    pub content_type: ContentType,
    pub added_at: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Slot {
    pub id: String,
    pub name: String,
    pub content: String,
    #[serde(rename = "type")]
    pub content_type: ContentType,
    pub updated_at: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProfileItem {
    pub id: String,
    pub label: String,
    pub content: String,
    #[serde(rename = "type")]
    pub content_type: ContentType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub items: Vec<ProfileItem>,
    pub created_at: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HistoryItem {
    pub id: String,
    pub content: String,
    #[serde(rename = "type")]
    pub content_type: ContentType,
    pub copied_at: u64,
    pub pinned: bool,
}

// Snapshot sent to the frontend on every queue mutation ("queue-updated" event).
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueueSnapshot {
    pub items: Vec<QueueItem>,
    pub cursor: usize,
    pub monitoring: bool,
}

pub struct AppState {
    pub db: Mutex<Connection>,
    pub queue: Mutex<Vec<QueueItem>>,
    pub cursor: Mutex<usize>,
    pub slots: Mutex<Vec<Slot>>,
    pub profiles: Mutex<Vec<Profile>>,
    pub monitoring: AtomicBool,
    // Content we wrote to the clipboard ourselves (during paste); the monitor
    // must skip it so pasting never re-enqueues the pasted item.
    pub suppress: Mutex<Option<String>>,
    pub paste_lock: Mutex<()>,
}

impl AppState {
    pub fn new(conn: Connection) -> Self {
        let queue = crate::db::load_queue(&conn).unwrap_or_default();
        let slots = crate::db::load_slots(&conn).unwrap_or_default();
        let profiles = crate::db::load_profiles(&conn).unwrap_or_default();
        let cursor: usize = crate::db::get_setting(&conn, "queue_cursor")
            .ok()
            .flatten()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);
        let cursor = cursor.min(queue.len().saturating_sub(1));
        Self {
            db: Mutex::new(conn),
            queue: Mutex::new(queue),
            cursor: Mutex::new(cursor),
            slots: Mutex::new(slots),
            profiles: Mutex::new(profiles),
            monitoring: AtomicBool::new(true),
            suppress: Mutex::new(None),
            paste_lock: Mutex::new(()),
        }
    }

    pub fn snapshot(&self) -> QueueSnapshot {
        QueueSnapshot {
            items: self.queue.lock().unwrap().clone(),
            cursor: *self.cursor.lock().unwrap(),
            monitoring: self.monitoring.load(std::sync::atomic::Ordering::Relaxed),
        }
    }
}

pub fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn restores_legacy_exhausted_cursor_to_last_saved_item() {
        let conn = crate::db::open(std::path::Path::new(":memory:")).unwrap();
        let items = vec![QueueItem {
            id: "saved".into(), content: "reuse me".into(),
            content_type: ContentType::Text, added_at: 0,
        }];
        crate::db::save_queue(&conn, &items).unwrap();
        crate::db::set_setting(&conn, "queue_cursor", "1").unwrap();
        let state = AppState::new(conn);
        assert_eq!(state.snapshot().cursor, 0);
        assert_eq!(state.snapshot().items[0].content, "reuse me");
    }
}
