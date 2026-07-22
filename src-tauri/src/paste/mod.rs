// Keyboard simulation engine. Writes content to the OS clipboard, then
// simulates Cmd+V in the frontmost app. Requires the Accessibility
// permission on macOS (System Settings > Privacy & Security > Accessibility).
use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};

use crate::db;
use crate::state::AppState;

// Small pause between clipboard write and keystroke so the target app
// sees the new clipboard contents.
const CLIPBOARD_SETTLE: Duration = Duration::from_millis(80);

pub fn paste_text(state: &AppState, content: &str) -> Result<(), String> {
    // Mark our own clipboard write so the monitor doesn't re-enqueue it.
    *state.suppress.lock().unwrap() = Some(content.to_string());

    let mut clipboard = arboard::Clipboard::new().map_err(|e| e.to_string())?;
    clipboard.set_text(content).map_err(|e| e.to_string())?;
    std::thread::sleep(CLIPBOARD_SETTLE);

    let mut enigo = Enigo::new(&Settings::default()).map_err(|e| e.to_string())?;
    enigo.key(Key::Meta, Direction::Press).map_err(|e| e.to_string())?;
    enigo.key(Key::Unicode('v'), Direction::Click).map_err(|e| e.to_string())?;
    enigo.key(Key::Meta, Direction::Release).map_err(|e| e.to_string())?;
    Ok(())
}

// Paste the item at the cursor and advance. Errors if the queue is exhausted.
pub fn paste_next(app: &AppHandle) -> Result<(), String> {
    let state = app.state::<AppState>();

    let content = {
        let queue = state.queue.lock().unwrap();
        let cursor = state.cursor.lock().unwrap();
        queue
            .get(*cursor)
            .map(|item| item.content.clone())
            .ok_or_else(|| "queue exhausted".to_string())?
    };

    paste_text(&state, &content)?;

    {
        let queue = state.queue.lock().unwrap();
        let mut cursor = state.cursor.lock().unwrap();
        if *cursor < queue.len() {
            *cursor += 1;
        }
        let db = state.db.lock().unwrap();
        let _ = db::set_setting(&db, "queue_cursor", &cursor.to_string());
    }

    let _ = app.emit("queue-updated", state.snapshot());
    Ok(())
}
