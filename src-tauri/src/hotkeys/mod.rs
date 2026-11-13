// Global hotkey manager. Registers system-wide shortcuts via
// tauri-plugin-global-shortcut and maps them to queue actions.
// Bindings persist in the settings table as JSON.
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
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
        binding("queue-paste", "Paste Selected Item", "CommandOrControl+Shift+V"),
        binding("queue-previous", "Previous Item", "CommandOrControl+Shift+Up"),
        binding("queue-next", "Next Item", "CommandOrControl+Shift+Down"),
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
    let saved: Vec<HotkeyBinding> = db::get_setting(conn, "hotkeys")
        .ok()
        .flatten()
        .and_then(|json| serde_json::from_str(&json).ok())
        .unwrap_or_else(default_bindings);
    let mut bindings = default_bindings();
    for binding in &mut bindings {
        if let Some(previous) = saved.iter().find(|b| b.action == binding.action) {
            binding.keys = previous.keys.clone();
        }
    }
    bindings
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
        let paste_held = AtomicBool::new(false);
        shortcuts
            .on_shortcut(b.keys.as_str(), move |app, _shortcut, event| {
                if action == "queue-paste" {
                    match event.state() {
                        ShortcutState::Pressed => {
                            if !paste_held.swap(true, Ordering::Relaxed) {
                                handle_action(app, &action);
                            }
                        }
                        ShortcutState::Released => paste_held.store(false, Ordering::Relaxed),
                    }
                } else if event.state() == ShortcutState::Pressed {
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
            eprintln!("Paste shortcut received; waiting for modifiers to be released.");
            let app = app.clone();
            std::thread::spawn(move || {
                if let Err(error) = crate::paste::paste_next(&app) {
                    eprintln!("Paste failed: {error}");
                    let _ = app.emit("queue-error", &error);
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            });
        }
        "queue-previous" | "queue-next" => {
            let state = app.state::<AppState>();
            crate::commands::move_selection(app, &state, action == "queue-next");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_navigation_to_saved_bindings_without_overwriting_custom_keys() {
        let conn = db::open(std::path::Path::new(":memory:")).unwrap();
        let mut old = default_bindings();
        old.retain(|b| b.action != "queue-previous" && b.action != "queue-next");
        old[0].keys = "Control+Alt+P".into();
        old[1].keys = "".into();
        save_bindings(&conn, &old).unwrap();
        let loaded = load_bindings(&conn);
        assert_eq!(loaded.iter().find(|b| b.action == "queue-paste").unwrap().keys, "Control+Alt+P");
        assert_eq!(loaded.iter().find(|b| b.action == "queue-add").unwrap().keys, "");
        assert!(loaded.iter().any(|b| b.action == "queue-previous"));
        assert!(loaded.iter().any(|b| b.action == "queue-next"));
    }
}
