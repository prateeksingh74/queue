mod clipboard;
mod commands;
mod db;
mod hotkeys;
mod input;
mod intelligence;
mod paste;
mod state;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::new())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_queue,
            commands::clear_queue,
            commands::get_slots,
            commands::get_profiles,
            commands::search_history,
        ])
        .run(tauri::generate_context!())
        .expect("error while running queue");
}
