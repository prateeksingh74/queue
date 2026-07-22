// Clipboard monitoring service. Polls the system clipboard for text changes,
// classifies new content, appends it to the queue and history, and notifies
// the frontend via "queue-updated" / "history-updated" events.
use std::sync::atomic::Ordering;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};
use uuid::Uuid;

use crate::db;
use crate::intelligence;
use crate::state::{now_ms, AppState, HistoryItem, QueueItem};

const POLL_INTERVAL: Duration = Duration::from_millis(300);

pub fn spawn_monitor(app: AppHandle) {
    std::thread::spawn(move || {
        let mut clipboard = match arboard::Clipboard::new() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("clipboard monitor failed to start: {e}");
                return;
            }
        };
        let mut last = clipboard.get_text().ok();

        loop {
            std::thread::sleep(POLL_INTERVAL);

            let Ok(text) = clipboard.get_text() else {
                continue; // clipboard holds non-text content (image, files)
            };
            if last.as_deref() == Some(text.as_str()) {
                continue;
            }
            last = Some(text.clone());

            if text.trim().is_empty() {
                continue;
            }

            let state = app.state::<AppState>();

            // Skip content we wrote ourselves during a paste.
            {
                let mut suppress = state.suppress.lock().unwrap();
                if suppress.as_deref() == Some(text.as_str()) {
                    *suppress = None;
                    continue;
                }
            }

            if !state.monitoring.load(Ordering::Relaxed) {
                continue;
            }

            capture(&app, &state, text);
        }
    });
}

fn capture(app: &AppHandle, state: &AppState, text: String) {
    let content_type = intelligence::classify(&text);
    let now = now_ms();

    let item = QueueItem {
        id: Uuid::new_v4().to_string(),
        content: text.clone(),
        content_type: content_type.clone(),
        added_at: now,
    };
    let history_item = HistoryItem {
        id: Uuid::new_v4().to_string(),
        content: text,
        content_type,
        copied_at: now,
        pinned: false,
    };

    {
        let mut queue = state.queue.lock().unwrap();
        queue.push(item);
        let db = state.db.lock().unwrap();
        let _ = db::save_queue(&db, &queue);
        let _ = db::insert_history(&db, &history_item);
        let max_items = db::get_setting(&db, "max_history_items")
            .ok()
            .flatten()
            .and_then(|v| v.parse().ok())
            .unwrap_or(500);
        let _ = db::trim_history(&db, max_items);
    }

    let _ = app.emit("queue-updated", state.snapshot());
    let _ = app.emit("history-updated", ());
}
