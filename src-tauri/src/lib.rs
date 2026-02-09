pub mod commands;
pub mod state;
pub mod service;
pub mod repository;
pub mod report;
pub mod config;

// 移动端入口标记保持
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::greet_cmd::greet,
            // commands::report_cmd::generate_report,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}