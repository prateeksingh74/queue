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

#[tauri::command]
pub fn get_slots(state: State<AppState>) -> Vec<Slot> {
    state.slots.lock().unwrap().clone()
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
