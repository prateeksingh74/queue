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

fn emit_profiles(app: &AppHandle, state: &AppState) {
    let _ = app.emit("profiles-updated", state.profiles.lock().unwrap().clone());
}

#[tauri::command]
pub fn get_profiles(state: State<AppState>) -> Vec<Profile> {
    state.profiles.lock().unwrap().clone()
}

#[tauri::command]
pub fn create_profile(app: AppHandle, state: State<AppState>, name: String) {
    {
        let mut profiles = state.profiles.lock().unwrap();
        let profile = Profile {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            items: Vec::new(),
            created_at: crate::state::now_ms(),
        };
        let db = state.db.lock().unwrap();
        let _ = db::save_profile(&db, &profile);
        profiles.push(profile);
    }
    emit_profiles(&app, &state);
}

#[tauri::command]
pub fn rename_profile(app: AppHandle, state: State<AppState>, id: String, name: String) {
    {
        let mut profiles = state.profiles.lock().unwrap();
        if let Some(profile) = profiles.iter_mut().find(|p| p.id == id) {
            profile.name = name;
            let db = state.db.lock().unwrap();
            let _ = db::save_profile(&db, profile);
        }
    }
    emit_profiles(&app, &state);
}

#[tauri::command]
pub fn delete_profile(app: AppHandle, state: State<AppState>, id: String) {
    {
        let mut profiles = state.profiles.lock().unwrap();
        profiles.retain(|p| p.id != id);
        let db = state.db.lock().unwrap();
        let _ = db::delete_profile(&db, &id);
    }
    emit_profiles(&app, &state);
}

#[tauri::command]
pub fn duplicate_profile(app: AppHandle, state: State<AppState>, id: String) {
    {
        let mut profiles = state.profiles.lock().unwrap();
        if let Some(source) = profiles.iter().find(|p| p.id == id).cloned() {
            let copy = Profile {
                id: uuid::Uuid::new_v4().to_string(),
                name: format!("{} Copy", source.name),
                items: source
                    .items
                    .into_iter()
                    .map(|mut item| {
                        item.id = uuid::Uuid::new_v4().to_string();
                        item
                    })
                    .collect(),
                created_at: crate::state::now_ms(),
            };
            let db = state.db.lock().unwrap();
            let _ = db::save_profile(&db, &copy);
            profiles.push(copy);
        }
    }
    emit_profiles(&app, &state);
}

#[tauri::command]
pub fn add_profile_item(
    app: AppHandle,
    state: State<AppState>,
    profile_id: String,
    label: String,
    content: String,
) {
    {
        let mut profiles = state.profiles.lock().unwrap();
        if let Some(profile) = profiles.iter_mut().find(|p| p.id == profile_id) {
            profile.items.push(crate::state::ProfileItem {
                id: uuid::Uuid::new_v4().to_string(),
                label,
                content_type: crate::intelligence::classify(&content),
                content,
            });
            let db = state.db.lock().unwrap();
            let _ = db::save_profile(&db, profile);
        }
    }
    emit_profiles(&app, &state);
}

#[tauri::command]
pub fn update_profile_item(
    app: AppHandle,
    state: State<AppState>,
    profile_id: String,
    item_id: String,
    label: String,
    content: String,
) {
    {
        let mut profiles = state.profiles.lock().unwrap();
        if let Some(profile) = profiles.iter_mut().find(|p| p.id == profile_id) {
            if let Some(item) = profile.items.iter_mut().find(|i| i.id == item_id) {
                item.label = label;
                item.content_type = crate::intelligence::classify(&content);
                item.content = content;
            }
            let db = state.db.lock().unwrap();
            let _ = db::save_profile(&db, profile);
        }
    }
    emit_profiles(&app, &state);
}

#[tauri::command]
pub fn remove_profile_item(
    app: AppHandle,
    state: State<AppState>,
    profile_id: String,
    item_id: String,
) {
    {
        let mut profiles = state.profiles.lock().unwrap();
        if let Some(profile) = profiles.iter_mut().find(|p| p.id == profile_id) {
            profile.items.retain(|i| i.id != item_id);
            let db = state.db.lock().unwrap();
            let _ = db::save_profile(&db, profile);
        }
    }
    emit_profiles(&app, &state);
}

// Replaces the queue with the profile's items, in order, and resets the cursor.
#[tauri::command]
pub fn load_profile(app: AppHandle, state: State<AppState>, id: String) -> Result<(), String> {
    {
        let profiles = state.profiles.lock().unwrap();
        let profile = profiles
            .iter()
            .find(|p| p.id == id)
            .ok_or_else(|| "profile not found".to_string())?;

        let now = crate::state::now_ms();
        let items: Vec<QueueItem> = profile
            .items
            .iter()
            .map(|item| QueueItem {
                id: uuid::Uuid::new_v4().to_string(),
                content: item.content.clone(),
                content_type: item.content_type.clone(),
                added_at: now,
            })
            .collect();

        let mut queue = state.queue.lock().unwrap();
        let mut cursor = state.cursor.lock().unwrap();
        *queue = items;
        *cursor = 0;
        let db = state.db.lock().unwrap();
        let _ = db::save_queue(&db, &queue);
        let _ = db::set_setting(&db, "queue_cursor", "0");
    }
    emit_queue(&app, &state);
    Ok(())
}

#[tauri::command]
pub fn search_history(
    state: State<AppState>,
    query: String,
) -> Result<Vec<crate::state::HistoryItem>, String> {
    let db = state.db.lock().unwrap();
    db::search_history(&db, &query).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_history_pinned(
    app: AppHandle,
    state: State<AppState>,
    id: String,
    pinned: bool,
) -> Result<(), String> {
    {
        let db = state.db.lock().unwrap();
        db::set_history_pinned(&db, &id, pinned).map_err(|e| e.to_string())?;
    }
    let _ = app.emit("history-updated", ());
    Ok(())
}

#[tauri::command]
pub fn delete_history_item(
    app: AppHandle,
    state: State<AppState>,
    id: String,
) -> Result<(), String> {
    {
        let db = state.db.lock().unwrap();
        db::delete_history_item(&db, &id).map_err(|e| e.to_string())?;
    }
    let _ = app.emit("history-updated", ());
    Ok(())
}

#[tauri::command]
pub fn clear_history(app: AppHandle, state: State<AppState>) -> Result<(), String> {
    {
        let db = state.db.lock().unwrap();
        db::clear_history(&db).map_err(|e| e.to_string())?;
    }
    let _ = app.emit("history-updated", ());
    Ok(())
}

// Re-adds a history item to the end of the queue.
#[tauri::command]
pub fn restore_history_item(
    app: AppHandle,
    state: State<AppState>,
    id: String,
) -> Result<(), String> {
    let item = {
        let db = state.db.lock().unwrap();
        db::get_history_item(&db, &id)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "history item not found".to_string())?
    };
    {
        let mut queue = state.queue.lock().unwrap();
        queue.push(QueueItem {
            id: uuid::Uuid::new_v4().to_string(),
            content: item.content,
            content_type: item.content_type,
            added_at: crate::state::now_ms(),
        });
        let db = state.db.lock().unwrap();
        let _ = db::save_queue(&db, &queue);
    }
    emit_queue(&app, &state);
    Ok(())
}

#[tauri::command]
pub fn set_max_history_items(state: State<AppState>, max_items: u32) {
    let db = state.db.lock().unwrap();
    let _ = db::set_setting(&db, "max_history_items", &max_items.to_string());
    let _ = db::trim_history(&db, max_items);
}

#[tauri::command]
pub fn get_hotkeys(state: State<AppState>) -> Vec<crate::hotkeys::HotkeyBinding> {
    let db = state.db.lock().unwrap();
    crate::hotkeys::load_bindings(&db)
}

// Rebinds one action. On registration failure the previous bindings are
// restored and the error (e.g. an OS-level conflict) is returned to the UI.
#[tauri::command]
pub fn set_hotkey(
    app: AppHandle,
    state: State<AppState>,
    action: String,
    keys: String,
) -> Result<Vec<crate::hotkeys::HotkeyBinding>, String> {
    let previous = {
        let db = state.db.lock().unwrap();
        crate::hotkeys::load_bindings(&db)
    };
    let mut updated = previous.clone();
    let Some(b) = updated.iter_mut().find(|b| b.action == action) else {
        return Err(format!("unknown action: {action}"));
    };
    b.keys = keys;

    if let Err(e) = crate::hotkeys::apply_bindings(&app, &updated) {
        let _ = crate::hotkeys::apply_bindings(&app, &previous);
        return Err(e);
    }
    let db = state.db.lock().unwrap();
    crate::hotkeys::save_bindings(&db, &updated).map_err(|e| e.to_string())?;
    Ok(updated)
}

#[tauri::command]
pub fn reset_hotkeys(
    app: AppHandle,
    state: State<AppState>,
) -> Result<Vec<crate::hotkeys::HotkeyBinding>, String> {
    let defaults = crate::hotkeys::default_bindings();
    crate::hotkeys::apply_bindings(&app, &defaults)?;
    let db = state.db.lock().unwrap();
    crate::hotkeys::save_bindings(&db, &defaults).map_err(|e| e.to_string())?;
    Ok(defaults)
}
