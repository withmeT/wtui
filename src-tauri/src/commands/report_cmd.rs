use crate::report::insert_toml::{create_template, load_config};
use crate::report::template::TemplateConfig;
use crate::commands::report_dto::DynamicConfig;
use crate::commands::grid_check_dto::GridCheckPayload;
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

// 保存前端传来的测评配置（DynamicConfig），以 JSON 形式持久化到本地
#[tauri::command]
pub fn save_dynamic_config(name: &str, data: DynamicConfig) -> Result<String, String> {
    // 确保数据目录存在
    let base_dir = PathBuf::from("data");
    if let Err(e) = fs::create_dir_all(&base_dir) {
        // 目录已存在时忽略错误，其它错误返回前端
        if !base_dir.exists() {
            return Err(format!("创建数据目录失败: {}", e));
        }
    }

    let path = base_dir.join(format!("{}.json", name));

    // 序列化为 JSON
    let json =
        serde_json::to_string_pretty(&data).map_err(|e| format!("序列化 JSON 失败: {}", e))?;

    // 写入文件
    fs::write(&path, json).map_err(|e| format!("写入文件失败: {}", e))?;

    Ok(format!("{}", path.display()))
}

// 加载已保存的测评配置，返回给前端编辑
#[tauri::command]
pub fn load_dynamic_config(name: &str) -> Result<DynamicConfig, String> {
    let base_dir = PathBuf::from("data");
    let path = base_dir.join(format!("{}.json", name));

    let content =
        fs::read_to_string(&path).map_err(|e| format!("读取配置失败: {}", e))?;

    let config: DynamicConfig =
        serde_json::from_str(&content).map_err(|e| format!("解析 JSON 失败: {}", e))?;

    Ok(config)
}

// 并网检测整体提交：TemplateData + CheckList，一次性保存为 JSON
#[tauri::command]
pub fn submit_grid_check(name: &str, payload: GridCheckPayload) -> Result<String, String> {
    let base_dir = PathBuf::from("grid-check");
    if let Err(e) = fs::create_dir_all(&base_dir) {
        if !base_dir.exists() {
            return Err(format!("创建并网检测目录失败: {}", e));
        }
    }

    let path = base_dir.join(format!("{}.json", name));

    let json =
        serde_json::to_string_pretty(&payload).map_err(|e| format!("序列化 JSON 失败: {}", e))?;

    fs::write(&path, json).map_err(|e| format!("写入文件失败: {}", e))?;

    Ok(format!("{}", path.display()))
}