use tauri::State;

use crate::state::{AppState, Profile, QueueItem, Slot};

#[tauri::command]
pub fn get_queue(state: State<AppState>) -> Vec<QueueItem> {
    state.queue.lock().unwrap().clone()
}

#[tauri::command]
pub fn clear_queue(state: State<AppState>) {
    let mut queue = state.queue.lock().unwrap();
    let mut cursor = state.cursor.lock().unwrap();
    queue.clear();
    *cursor = 0;
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
