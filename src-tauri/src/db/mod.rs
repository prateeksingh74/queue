// SQLite persistence layer. One connection guarded by a Mutex in AppState;
// all access is synchronous, which matches the Mutex-based state model.
use rusqlite::{params, Connection};
use std::path::Path;

use crate::state::{ContentType, HistoryItem, Profile, ProfileItem, QueueItem, Slot};

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS queue_items (
    id TEXT PRIMARY KEY,
    content TEXT NOT NULL,
    content_type TEXT NOT NULL,
    added_at INTEGER NOT NULL,
    position INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS slots (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    content TEXT NOT NULL DEFAULT '',
    content_type TEXT NOT NULL DEFAULT 'text',
    updated_at INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS profiles (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    created_at INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS profile_items (
    id TEXT PRIMARY KEY,
    profile_id TEXT NOT NULL REFERENCES profiles(id) ON DELETE CASCADE,
    label TEXT NOT NULL,
    content TEXT NOT NULL,
    content_type TEXT NOT NULL DEFAULT 'text',
    position INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS history (
    id TEXT PRIMARY KEY,
    content TEXT NOT NULL,
    content_type TEXT NOT NULL,
    copied_at INTEGER NOT NULL,
    pinned INTEGER NOT NULL DEFAULT 0
);
CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
";

pub fn open(path: &Path) -> rusqlite::Result<Connection> {
    let conn = Connection::open(path)?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    conn.execute_batch(SCHEMA)?;
    Ok(conn)
}

fn type_to_str(t: &ContentType) -> &'static str {
    match t {
        ContentType::Text => "text",
        ContentType::Email => "email",
        ContentType::Phone => "phone",
        ContentType::Url => "url",
        ContentType::Address => "address",
        ContentType::Date => "date",
        ContentType::Unknown => "unknown",
    }
}

fn str_to_type(s: &str) -> ContentType {
    match s {
        "email" => ContentType::Email,
        "phone" => ContentType::Phone,
        "url" => ContentType::Url,
        "address" => ContentType::Address,
        "date" => ContentType::Date,
        "text" => ContentType::Text,
        _ => ContentType::Unknown,
    }
}

// --- queue ---

pub fn load_queue(conn: &Connection) -> rusqlite::Result<Vec<QueueItem>> {
    let mut stmt = conn.prepare(
        "SELECT id, content, content_type, added_at FROM queue_items ORDER BY position",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(QueueItem {
            id: row.get(0)?,
            content: row.get(1)?,
            content_type: str_to_type(&row.get::<_, String>(2)?),
            added_at: row.get(3)?,
        })
    })?;
    rows.collect()
}

pub fn save_queue(conn: &Connection, items: &[QueueItem]) -> rusqlite::Result<()> {
    conn.execute("DELETE FROM queue_items", [])?;
    let mut stmt = conn.prepare(
        "INSERT INTO queue_items (id, content, content_type, added_at, position) VALUES (?1, ?2, ?3, ?4, ?5)",
    )?;
    for (i, item) in items.iter().enumerate() {
        stmt.execute(params![
            item.id,
            item.content,
            type_to_str(&item.content_type),
            item.added_at,
            i as i64
        ])?;
    }
    Ok(())
}

// --- slots ---

pub fn load_slots(conn: &Connection) -> rusqlite::Result<Vec<Slot>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, content, content_type, updated_at FROM slots ORDER BY updated_at",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(Slot {
            id: row.get(0)?,
            name: row.get(1)?,
            content: row.get(2)?,
            content_type: str_to_type(&row.get::<_, String>(3)?),
            updated_at: row.get(4)?,
        })
    })?;
    rows.collect()
}

pub fn upsert_slot(conn: &Connection, slot: &Slot) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO slots (id, name, content, content_type, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)
         ON CONFLICT(id) DO UPDATE SET name = ?2, content = ?3, content_type = ?4, updated_at = ?5",
        params![
            slot.id,
            slot.name,
            slot.content,
            type_to_str(&slot.content_type),
            slot.updated_at
        ],
    )?;
    Ok(())
}

pub fn delete_slot(conn: &Connection, id: &str) -> rusqlite::Result<()> {
    conn.execute("DELETE FROM slots WHERE id = ?1", params![id])?;
    Ok(())
}

// --- profiles ---

pub fn load_profiles(conn: &Connection) -> rusqlite::Result<Vec<Profile>> {
    let mut stmt =
        conn.prepare("SELECT id, name, created_at FROM profiles ORDER BY created_at")?;
    let profiles: Vec<(String, String, u64)> = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?
        .collect::<Result<_, _>>()?;

    let mut item_stmt = conn.prepare(
        "SELECT id, label, content, content_type FROM profile_items WHERE profile_id = ?1 ORDER BY position",
    )?;
    profiles
        .into_iter()
        .map(|(id, name, created_at)| {
            let items = item_stmt
                .query_map(params![id], |row| {
                    Ok(ProfileItem {
                        id: row.get(0)?,
                        label: row.get(1)?,
                        content: row.get(2)?,
                        content_type: str_to_type(&row.get::<_, String>(3)?),
                    })
                })?
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Profile { id, name, items, created_at })
        })
        .collect()
}

pub fn save_profile(conn: &Connection, profile: &Profile) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO profiles (id, name, created_at) VALUES (?1, ?2, ?3)
         ON CONFLICT(id) DO UPDATE SET name = ?2",
        params![profile.id, profile.name, profile.created_at],
    )?;
    conn.execute(
        "DELETE FROM profile_items WHERE profile_id = ?1",
        params![profile.id],
    )?;
    let mut stmt = conn.prepare(
        "INSERT INTO profile_items (id, profile_id, label, content, content_type, position) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
    )?;
    for (i, item) in profile.items.iter().enumerate() {
        stmt.execute(params![
            item.id,
            profile.id,
            item.label,
            item.content,
            type_to_str(&item.content_type),
            i as i64
        ])?;
    }
    Ok(())
}

pub fn delete_profile(conn: &Connection, id: &str) -> rusqlite::Result<()> {
    conn.execute("DELETE FROM profiles WHERE id = ?1", params![id])?;
    Ok(())
}

// --- history ---

pub fn insert_history(conn: &Connection, item: &HistoryItem) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO history (id, content, content_type, copied_at, pinned) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            item.id,
            item.content,
            type_to_str(&item.content_type),
            item.copied_at,
            item.pinned as i64
        ],
    )?;
    Ok(())
}

pub fn search_history(conn: &Connection, query: &str) -> rusqlite::Result<Vec<HistoryItem>> {
    let mut stmt = conn.prepare(
        "SELECT id, content, content_type, copied_at, pinned FROM history
         WHERE content LIKE ?1 ORDER BY pinned DESC, copied_at DESC LIMIT 200",
    )?;
    let pattern = format!("%{}%", query);
    let rows = stmt.query_map(params![pattern], |row| {
        Ok(HistoryItem {
            id: row.get(0)?,
            content: row.get(1)?,
            content_type: str_to_type(&row.get::<_, String>(2)?),
            copied_at: row.get(3)?,
            pinned: row.get::<_, i64>(4)? != 0,
        })
    })?;
    rows.collect()
}

pub fn set_history_pinned(conn: &Connection, id: &str, pinned: bool) -> rusqlite::Result<()> {
    conn.execute(
        "UPDATE history SET pinned = ?2 WHERE id = ?1",
        params![id, pinned as i64],
    )?;
    Ok(())
}

pub fn delete_history_item(conn: &Connection, id: &str) -> rusqlite::Result<()> {
    conn.execute("DELETE FROM history WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn clear_history(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute("DELETE FROM history WHERE pinned = 0", [])?;
    Ok(())
}

// Unpinned items beyond max_items are dropped, oldest first.
pub fn trim_history(conn: &Connection, max_items: u32) -> rusqlite::Result<()> {
    conn.execute(
        "DELETE FROM history WHERE pinned = 0 AND id NOT IN (
            SELECT id FROM history WHERE pinned = 0 ORDER BY copied_at DESC LIMIT ?1
        )",
        params![max_items],
    )?;
    Ok(())
}

// --- settings ---

pub fn get_setting(conn: &Connection, key: &str) -> rusqlite::Result<Option<String>> {
    let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = ?1")?;
    let mut rows = stmt.query(params![key])?;
    rows.next()?.map(|row| row.get(0)).transpose()
}

pub fn set_setting(conn: &Connection, key: &str, value: &str) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO settings (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = ?2",
        params![key, value],
    )?;
    Ok(())
}
