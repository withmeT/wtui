use crate::commands::report_dto::DynamicConfig;


#[tauri::command]
pub fn report(_input: DynamicConfig) -> Result<(), String> {
    Ok(())
}