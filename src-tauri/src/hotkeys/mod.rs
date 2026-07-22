// Global hotkey manager. Registers system-wide shortcuts via
// tauri-plugin-global-shortcut and maps them to queue actions.
// Bindings persist in the settings table as JSON.
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::db;
use crate::state::AppState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HotkeyBinding {
    pub action: String,
    pub label: String,
    pub keys: String,
}

pub fn default_bindings() -> Vec<HotkeyBinding> {
    vec![
        binding("queue-paste", "Queue Paste", "CommandOrControl+Shift+V"),
        binding("queue-add", "Add Clipboard to Queue", "CommandOrControl+Shift+C"),
        binding("open-queue", "Open Queue", "CommandOrControl+Shift+Q"),
        binding("clear-queue", "Clear Queue", ""),
    ]
}

fn binding(action: &str, label: &str, keys: &str) -> HotkeyBinding {
    HotkeyBinding {
        action: action.into(),
        label: label.into(),
        keys: keys.into(),
    }
}

pub fn load_bindings(conn: &rusqlite::Connection) -> Vec<HotkeyBinding> {
    db::get_setting(conn, "hotkeys")
        .ok()
        .flatten()
        .and_then(|json| serde_json::from_str(&json).ok())
        .unwrap_or_else(default_bindings)
}

pub fn save_bindings(
    conn: &rusqlite::Connection,
    bindings: &[HotkeyBinding],
) -> rusqlite::Result<()> {
    let json = serde_json::to_string(bindings).unwrap_or_default();
    db::set_setting(conn, "hotkeys", &json)
}

// Replaces all registered shortcuts with the given bindings.
// Fails with the offending accelerator if one can't be registered.
pub fn apply_bindings(app: &AppHandle, bindings: &[HotkeyBinding]) -> Result<(), String> {
    let shortcuts = app.global_shortcut();
    shortcuts.unregister_all().map_err(|e| e.to_string())?;

    for b in bindings.iter().filter(|b| !b.keys.is_empty()) {
        let action = b.action.clone();
        shortcuts
            .on_shortcut(b.keys.as_str(), move |app, _shortcut, event| {
                if event.state() == ShortcutState::Pressed {
                    handle_action(app, &action);
                }
            })
            .map_err(|e| format!("failed to register {}: {}", b.keys, e))?;
    }
    Ok(())
}

fn handle_action(app: &AppHandle, action: &str) {
    match action {
        "queue-paste" => {
            let _ = crate::paste::paste_next(app);
        }
        "queue-add" => {
            // Works even while monitoring is paused — that's the point.
            if let Ok(text) = arboard::Clipboard::new().and_then(|mut c| c.get_text()) {
                if !text.trim().is_empty() {
                    let state = app.state::<AppState>();
                    crate::clipboard::capture(app, &state, text);
                }
            }
        }
        "open-queue" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        "clear-queue" => {
            let state = app.state::<AppState>();
            {
                let mut queue = state.queue.lock().unwrap();
                let mut cursor = state.cursor.lock().unwrap();
                queue.clear();
                *cursor = 0;
                let db = state.db.lock().unwrap();
                let _ = db::save_queue(&db, &queue);
                let _ = db::set_setting(&db, "queue_cursor", "0");
            }
            let _ = app.emit("queue-updated", state.snapshot());
        }
        _ => {}
    }
}
