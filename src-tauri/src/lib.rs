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
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&data_dir)?;
            let conn = db::open(&data_dir.join("queue.db"))?;
            let bindings = hotkeys::load_bindings(&conn);
            app.manage(AppState::new(conn));
            clipboard::spawn_monitor(app.handle().clone());
            if let Err(e) = hotkeys::apply_bindings(app.handle(), &bindings) {
                eprintln!("hotkey registration failed: {e}");
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_queue_state,
            commands::remove_queue_item,
            commands::clear_queue,
            commands::reset_cursor,
            commands::select_queue_item,
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
            commands::create_profile,
            commands::rename_profile,
            commands::delete_profile,
            commands::duplicate_profile,
            commands::add_profile_item,
            commands::update_profile_item,
            commands::remove_profile_item,
            commands::load_profile,
            commands::search_history,
            commands::set_history_pinned,
            commands::delete_history_item,
            commands::clear_history,
            commands::restore_history_item,
            commands::set_max_history_items,
            commands::get_hotkeys,
            commands::set_hotkey,
            commands::reset_hotkeys,
        ])
        .run(tauri::generate_context!())
        .expect("error while running queue");
}
