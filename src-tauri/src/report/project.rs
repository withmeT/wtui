//! 项目 TOML：创建、保存、加载（与 template.toml 同构 + checklist）

use crate::report::template::{Checklist, Config, ProjectToml, TemplateConfig};
use crate::report::template::TemplateError;
use std::fs;
use std::path::PathBuf;

const PROJECTS_DIR: &str = "projects";
const TEMPLATE_TOML: &str = "template.toml";

fn projects_dir() -> PathBuf {
    PathBuf::from(PROJECTS_DIR)
}

pub fn project_path(name: &str) -> PathBuf {
    projects_dir().join(format!("{}.toml", name))
}

/// 读取 template.toml 得到 Config
fn load_template_config() -> Result<Config, TemplateError> {
    for path in &[TEMPLATE_TOML, "src-tauri/template.toml"] {
        let p = PathBuf::from(path);
        if p.exists() {
            let mut content = fs::read_to_string(&p)?;
            content = content.replace("\r\n", "\n").replace("\r", "\n");
            return toml::from_str(&content).map_err(Into::into);
        }
    }
    Err(TemplateError::TemplateNotFound(
        "template.toml 不存在（请放在项目根或 src-tauri/ 下）".into(),
    ))
}

/// 从 收费.toml 内容解析出第一个 checklist 并清空 detection_result/conclusion
fn parse_empty_checklist(toml_content: &str) -> Result<Checklist, TemplateError> {
    let config: TemplateConfig = toml::from_str(toml_content)?;
    let mut c = config
        .checklist
        .into_iter()
        .next()
        .ok_or_else(|| TemplateError::TemplateNotFound("收费.toml 中无 checklist".into()))?;
    for item in &mut c.items {
        item.detection_result.clear();
        item.conclusion.clear();
    }
    Ok(c)
}

/// 创建新项目：template.toml 结构 + 收费.toml 的测评项（结果为空）
pub fn create_project_toml(name: &str, checklist_toml_content: &str) -> Result<PathBuf, TemplateError> {
    let dir = projects_dir();
    fs::create_dir_all(&dir)?;
    let path = project_path(name);
    let config = load_template_config()?;
    let checklist = parse_empty_checklist(checklist_toml_content)?;
    let project = ProjectToml {
        config,
        checklist: Some(checklist),
    };
    let toml_str = toml::to_string_pretty(&project).map_err(TemplateError::TomlSerError)?;
    fs::write(&path, toml_str)?;
    Ok(path)
}

/// 保存项目：写入 Config + Checklist 到 projects/{name}.toml
pub fn save_project_toml(name: &str, config: Config, checklist: Checklist) -> Result<PathBuf, TemplateError> {
    let dir = projects_dir();
    fs::create_dir_all(&dir)?;
    let path = project_path(name);
    let project = ProjectToml {
        config,
        checklist: Some(checklist),
    };
    let toml_str = toml::to_string_pretty(&project).map_err(TemplateError::TomlSerError)?;
    fs::write(&path, toml_str)?;
    Ok(path)
}

/// 加载项目：返回 Config + Checklist
pub fn load_project_toml(name: &str) -> Result<(Config, Option<Checklist>), TemplateError> {
    let path = project_path(name);
    if !path.exists() {
        return Err(TemplateError::TemplateNotFound(
            format!("项目不存在: {}", path.display()),
        ));
    }
    let mut content = fs::read_to_string(&path)?;
    content = content.replace("\r\n", "\n").replace("\r", "\n");
    let project: ProjectToml = toml::from_str(&content)?;
    Ok((project.config, project.checklist))
}

/// 列出已有项目（projects/*.toml 文件名，不含扩展名）
pub fn list_projects() -> Result<Vec<String>, TemplateError> {
    let dir = projects_dir();
    if !dir.exists() {
        return Ok(vec![]);
    }
    let mut names: Vec<String> = fs::read_dir(&dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |x| x == "toml"))
        .filter_map(|e| {
            e.path()
                .file_stem()
                .and_then(|s| s.to_str())
                .map(String::from)
        })
        .collect();
    names.sort();
    Ok(names)
}
