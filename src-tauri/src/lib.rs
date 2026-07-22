mod clipboard;
mod commands;
mod db;
mod hotkeys;
mod input;
mod intelligence;
mod paste;
mod state;

use state::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&data_dir)?;
            let conn = db::open(&data_dir.join("queue.db"))?;
            app.manage(AppState::new(conn));
            clipboard::spawn_monitor(app.handle().clone());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_queue_state,
            commands::remove_queue_item,
            commands::clear_queue,
            commands::reset_cursor,
            commands::set_monitoring,
            commands::queue_paste,
            commands::get_slots,
            commands::create_slot,
            commands::set_slot_content,
            commands::rename_slot,
            commands::delete_slot,
            commands::capture_slot,
            commands::paste_slot,
            commands::get_profiles,
            commands::search_history,
        ])
        .run(tauri::generate_context!())
        .expect("error while running queue");
}
