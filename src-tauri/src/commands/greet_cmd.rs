use crate::config::template::create_template;
use crate::config::template::load_config;
#[tauri::command]
pub fn greet(name: &str) -> String {
    match create_template(name) {
        Ok(file_path) => format!(
            "✅ 创建成功！配置文件已生成：{}",
            file_path.display()
        ),
        Err(e) => format!("❌ 创建失败：{}", e),
    }
}
