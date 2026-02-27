//! 项目 TOML：创建、保存、加载（与 template.toml 同构 + checklist）

use crate::report::template::TemplateError;
use crate::report::template::{Checklist, Config, ProjectToml, TemplateConfig};
use std::fs;
use std::path::{Path, PathBuf};

fn projects_dir(base_dir: &Path) -> PathBuf {
    base_dir.join("projects")
}

fn sanitize_filename(input: &str) -> String {
    // Windows 保留字符：<>:"/\|?* 以及控制字符
    let mut s = input
        .chars()
        .map(|c| match c {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect::<String>();

    // 去掉首尾空白，并避免以 '.' 结尾（Windows 不允许）
    s = s.trim().trim_end_matches('.').to_string();
    if s.is_empty() {
        "untitled".to_string()
    } else {
        s
    }
}

pub fn project_path(base_dir: &Path, name: &str) -> PathBuf {
    let safe = sanitize_filename(name);
    projects_dir(base_dir).join(format!("{}.toml", safe))
}

/// 从 template.toml 内容解析 Config
fn parse_template_config(template_toml_content: &str) -> Result<Config, TemplateError> {
    let normalized = template_toml_content
        .replace("\r\n", "\n")
        .replace("\r", "\n");
    toml::from_str(&normalized).map_err(Into::into)
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
pub fn create_project_toml(
    base_dir: &Path,
    name: &str,
    template_toml_content: &str,
    checklist_toml_content: &str,
) -> Result<PathBuf, TemplateError> {
    let dir = projects_dir(base_dir);
    fs::create_dir_all(&dir)?;
    let path = project_path(base_dir, name);
    let config = parse_template_config(template_toml_content)?;
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
pub fn save_project_toml(
    base_dir: &Path,
    name: &str,
    config: Config,
    checklist: Checklist,
) -> Result<PathBuf, TemplateError> {
    let dir = projects_dir(base_dir);
    fs::create_dir_all(&dir)?;
    let path = project_path(base_dir, name);
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
    load_project_toml_at(&PathBuf::from("."), name)
}

pub fn load_project_toml_at(
    base_dir: &Path,
    name: &str,
) -> Result<(Config, Option<Checklist>), TemplateError> {
    let path = project_path(base_dir, name);
    if !path.exists() {
        return Err(TemplateError::TemplateNotFound(format!(
            "项目不存在: {}",
            path.display()
        )));
    }
    let mut content = fs::read_to_string(&path)?;
    content = content.replace("\r\n", "\n").replace("\r", "\n");
    let project: ProjectToml = toml::from_str(&content)?;
    Ok((project.config, project.checklist))
}

/// 列出已有项目（projects/*.toml 文件名，不含扩展名）
pub fn list_projects(base_dir: &Path) -> Result<Vec<String>, TemplateError> {
    let dir = projects_dir(base_dir);
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn create_and_load_project_roundtrip() {
        let base_dir = std::env::temp_dir().join(format!("wtui-test-{}", {
            use std::time::{SystemTime, UNIX_EPOCH};
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0)
        }));
        let _ = fs::remove_dir_all(&base_dir);
        fs::create_dir_all(&base_dir).expect("create temp base dir");

        let template = include_str!("../../resources/template.toml");
        let checklist = include_str!("../../resources/收费.toml");

        let name = "2026/02/27:测试项目";
        let path = create_project_toml(&base_dir, name, template, checklist).expect("create project");
        assert!(path.exists(), "project file should exist");

        let (config, checklist_opt) = load_project_toml_at(&base_dir, name).expect("load project");
        let c = checklist_opt.expect("checklist should exist");

        assert!(
            config.基础信息.is_object(),
            "基础信息 should be an object json"
        );
        assert!(!c.items.is_empty(), "checklist items should not be empty");
    }
}
