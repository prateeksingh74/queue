use std::sync::atomic::Ordering;
use tauri::{AppHandle, Emitter, State};

use crate::db;
use crate::paste;
use crate::state::{AppState, Profile, QueueItem, QueueSnapshot, Slot};

fn emit_queue(app: &AppHandle, state: &AppState) {
    let _ = app.emit("queue-updated", state.snapshot());
}

#[tauri::command]
pub fn get_queue_state(state: State<AppState>) -> QueueSnapshot {
    state.snapshot()
}

#[tauri::command]
pub fn remove_queue_item(app: AppHandle, state: State<AppState>, id: String) {
    {
        let mut queue = state.queue.lock().unwrap();
        let mut cursor = state.cursor.lock().unwrap();
        if let Some(index) = queue.iter().position(|item| item.id == id) {
            queue.remove(index);
            if index < *cursor {
                *cursor -= 1;
            }
        }
        let db = state.db.lock().unwrap();
        let _ = db::save_queue(&db, &queue);
        let _ = db::set_setting(&db, "queue_cursor", &cursor.to_string());
    }
    emit_queue(&app, &state);
}

#[tauri::command]
pub fn clear_queue(app: AppHandle, state: State<AppState>) {
    {
        let mut queue = state.queue.lock().unwrap();
        let mut cursor = state.cursor.lock().unwrap();
        queue.clear();
        *cursor = 0;
        let db = state.db.lock().unwrap();
        let _ = db::save_queue(&db, &queue);
        let _ = db::set_setting(&db, "queue_cursor", "0");
    }
    emit_queue(&app, &state);
}

#[tauri::command]
pub fn reset_cursor(app: AppHandle, state: State<AppState>) {
    {
        let mut cursor = state.cursor.lock().unwrap();
        *cursor = 0;
        let db = state.db.lock().unwrap();
        let _ = db::set_setting(&db, "queue_cursor", "0");
    }
    emit_queue(&app, &state);
}

#[tauri::command]
pub fn set_monitoring(app: AppHandle, state: State<AppState>, enabled: bool) {
    state.monitoring.store(enabled, Ordering::Relaxed);
    emit_queue(&app, &state);
}

#[tauri::command]
pub fn queue_paste(app: AppHandle) -> Result<(), String> {
    paste::paste_next(&app)
}

fn emit_slots(app: &AppHandle, state: &AppState) {
    let _ = app.emit("slots-updated", state.slots.lock().unwrap().clone());
}

#[tauri::command]
pub fn get_slots(state: State<AppState>) -> Vec<Slot> {
    state.slots.lock().unwrap().clone()
}

#[tauri::command]
pub fn create_slot(app: AppHandle, state: State<AppState>, name: String) {
    {
        let mut slots = state.slots.lock().unwrap();
        let slot = Slot {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            content: String::new(),
            content_type: crate::state::ContentType::Text,
            updated_at: crate::state::now_ms(),
        };
        let db = state.db.lock().unwrap();
        let _ = db::upsert_slot(&db, &slot);
        slots.push(slot);
    }
    emit_slots(&app, &state);
}

#[tauri::command]
pub fn set_slot_content(app: AppHandle, state: State<AppState>, id: String, content: String) {
    {
        let mut slots = state.slots.lock().unwrap();
        if let Some(slot) = slots.iter_mut().find(|s| s.id == id) {
            slot.content_type = crate::intelligence::classify(&content);
            slot.content = content;
            slot.updated_at = crate::state::now_ms();
            let db = state.db.lock().unwrap();
            let _ = db::upsert_slot(&db, slot);
        }
    }
    emit_slots(&app, &state);
}

#[tauri::command]
pub fn rename_slot(app: AppHandle, state: State<AppState>, id: String, name: String) {
    {
        let mut slots = state.slots.lock().unwrap();
        if let Some(slot) = slots.iter_mut().find(|s| s.id == id) {
            slot.name = name;
            let db = state.db.lock().unwrap();
            let _ = db::upsert_slot(&db, slot);
        }
    }
    emit_slots(&app, &state);
}

#[tauri::command]
pub fn delete_slot(app: AppHandle, state: State<AppState>, id: String) {
    {
        let mut slots = state.slots.lock().unwrap();
        slots.retain(|s| s.id != id);
        let db = state.db.lock().unwrap();
        let _ = db::delete_slot(&db, &id);
    }
    emit_slots(&app, &state);
}

// Copies the current clipboard text into the slot ("Store to Slot").
#[tauri::command]
pub fn capture_slot(app: AppHandle, state: State<AppState>, id: String) -> Result<(), String> {
    let text = arboard::Clipboard::new()
        .and_then(|mut c| c.get_text())
        .map_err(|e| e.to_string())?;
    set_slot_content(app, state, id, text);
    Ok(())
}

// Pastes a slot's content into the frontmost app.
#[tauri::command]
pub fn paste_slot(state: State<AppState>, id: String) -> Result<(), String> {
    let content = {
        let slots = state.slots.lock().unwrap();
        slots
            .iter()
            .find(|s| s.id == id)
            .map(|s| s.content.clone())
            .ok_or_else(|| "slot not found".to_string())?
    };
    if content.is_empty() {
        return Err("slot is empty".to_string());
    }
    paste::paste_text(&state, &content)
}

#[tauri::command]
pub fn get_profiles(state: State<AppState>) -> Vec<Profile> {
    state.profiles.lock().unwrap().clone()
}

// Implemented in Phase 6 — returns empty results for now
#[tauri::command]
pub fn search_history(_query: String) -> Vec<QueueItem> {
    vec![]
}
