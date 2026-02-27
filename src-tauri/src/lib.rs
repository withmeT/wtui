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
            commands::report_cmd::create_temp,
            commands::report_cmd::load_conf,
            commands::report_cmd::load_template_toml,
            commands::report_cmd::save_dynamic_config,
            commands::report_cmd::load_dynamic_config,
            commands::report_cmd::submit_grid_check,
            commands::report_cmd::load_checklist,
            commands::report_cmd::create_project,
            commands::report_cmd::save_project,
            commands::report_cmd::load_project,
            commands::report_cmd::list_project_names,
            commands::report_cmd::generate_grid_check_report,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}