use crate::report::insert_toml::{create_template, load_config};
use crate::report::template::TemplateConfig;
use std::fs;
use tauri::Manager;
use std::path::PathBuf;

// 创建配置文件
#[tauri::command]
pub fn create_temp(name: &str) -> String {
    match create_template(name) {
        Ok(file_path) => format!(
            "✅ 创建成功！配置文件已生成{}",
            file_path
        ),
        Err(e) => format!("❌ 创建失败：{}", e),
    }
}

// 加载配置文件
#[tauri::command]
pub fn load_conf(name: &str) -> String {
    match load_config(name) {
        Ok(_toml_config) => format!(
            "✅ 配置文件加载成功"
        ),
        Err(e) => format!("❌ 配置文件失败：{}", e),
    }
}

// 加载收费、etc、路段中心检测项
#[tauri::command]
pub fn load_template_toml(app: tauri::AppHandle, name: &str) -> Result<String, String> {
    // 先尝试生产环境路径（打包后）
    let toml_content = if let Ok(resource_path) = app.path().resource_dir() {
        let prod_path = resource_path.join(format!("{}.toml", name));
        if prod_path.exists() {
            fs::read_to_string(&prod_path)
                .map_err(|e| format!("读取 toml 失败: {}", e))?
        } else {
            // 开发环境路径
            let dev_path = PathBuf::from("resources").join(format!("{}.toml", name));
            fs::read_to_string(&dev_path)
                .map_err(|e| format!("读取 toml 失败: {}", e))?
        }
    } else {
        // 开发环境路径
        let dev_path = PathBuf::from("resources").join(format!("{}.toml", name));
        fs::read_to_string(&dev_path)
            .map_err(|e| format!("读取 toml 失败: {}", e))?
    };
    
    // 解析 TOML
    let config: TemplateConfig = toml::from_str(&toml_content)
        .map_err(|e| format!("解析 TOML 失败: {}", e))?;
    
    // 转换为 JSON 返回
    serde_json::to_string(&config)
        .map_err(|e| format!("转换 JSON 失败: {}", e))
}