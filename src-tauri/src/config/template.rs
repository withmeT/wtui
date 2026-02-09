use std::fs;
use std::path::{Path, PathBuf};
use crate::report::template::{Config,TemplateError};


// 定义 Result 别名
pub type Result<T> = std::result::Result<T, TemplateError>;

/// 从项目目录下的 temp.toml 模板复制创建新的配置文件
/// 
/// # 参数
/// - `name`: 新配置文件名（不含 .toml 扩展名）
/// 
/// # 返回值
/// - Ok(PathBuf): 成功创建的文件路径
/// - Err(TemplateError): 创建失败的错误信息
pub fn create_template(name: &str) -> Result<PathBuf> {
    // 1. 定义模板文件路径（项目根目录下的 temp.toml）
    let template_path = Path::new("template.toml");
    
    // 检查模板文件是否存在
    if !template_path.exists() {
        return Err(TemplateError::TemplateNotFound(
            format!("模板文件 {} 不存在", template_path.display())
        ));
    }

    // 2. 定义目标文件路径（当前目录 + 传入的名称 + .toml 扩展名）
    let target_dir = Path::new("."); // 当前工作目录
    let target_path = target_dir.join(format!("{}.toml", name));

    // 3. 读取模板文件内容
    let template_content = fs::read_to_string(template_path)?;

    // 4. 将模板内容写入新文件（存在则覆盖）
    fs::write(&target_path, template_content)?;

    println!("成功从模板创建配置文件：{}", target_path.display());
    Ok(target_path)
}


pub fn save_config(config: &Config, file_name: &str) -> Result<()> {
    // 1. 序列化为格式化的 TOML 字符串
    let toml_content = toml::to_string_pretty(config)?;

    // 2. 写入文件（覆盖原有内容）
    let config_path = Path::new(file_name);
    fs::write(config_path, toml_content)?;

    println!("✅ 配置文件保存成功：{}", config_path.display());
    Ok(())
}

pub fn load_config(file_name: &str) -> Result<Config> {
    // 1. 定义配置文件路径
    let config_path = Path::new(file_name);
    
    // 检查文件是否存在
    if !config_path.exists() {
        return Err(TemplateError::TemplateNotFound(
            format!("配置文件 {} 不存在", config_path.display())
        ));
    }

    // 2. 读取文件内容并清理换行符（兼容 Windows \r\n）
    let mut config_content = fs::read_to_string(config_path)?;
    config_content = config_content.replace("\r\n", "\n").replace("\r", "\n");

    // 3. 解析 TOML 为 Config 结构体（兼容中文键/嵌套结构）
    let config: Config = toml::from_str(&config_content)?;

    println!("✅ 配置文件加载成功：{}", config_path.display());
    Ok(config)
}