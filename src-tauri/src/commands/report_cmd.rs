use crate::commands::grid_check_dto::{
    AssetInfo as DtoAssetInfo, BaseInfo, ChargeCommon, CheckItem as DtoCheckItem,
    CheckList as DtoCheckList, EtcCommon, GridCheckPayload, MainProblem, ReportConclusion,
    SampleScope, SystemInfo, TemplateData as DtoTemplateData, UnitInfo, VulnResult,
};
use crate::commands::report_dto::DynamicConfig;
use crate::report::insert_toml::{create_template, load_config};
use crate::report::project::{
    create_project_toml, load_project_toml_at, save_project_toml, list_projects,
};
use crate::report::template::TemplateError;
use crate::report::template::{
    AssetInfo, Checklist, ChecklistItem, CommonRequirements, Config, Device, TemplateConfig,
};
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

fn parse_json_value(s: &str) -> serde_json::Value {
    serde_json::from_str(s).unwrap_or_else(|_| json!({}))
}
fn parse_json_array_device(s: &str) -> Vec<Device> {
    serde_json::from_str(s).unwrap_or_else(|_| vec![])
}

fn payload_to_config_and_checklist(payload: &GridCheckPayload) -> (Config, Checklist) {
    let td = &payload.template_data;
    let config = Config {
        基础信息: json!({
            "编号": td.基础信息.编号,
            "报告时间": td.基础信息.报告时间,
            "检测时间": td.基础信息.检测时间,
        }),
        被测单位信息: json!({
            "单位名称": td.被测单位信息.单位名称,
            "单位地址": td.被测单位信息.单位地址,
        }),
        系统信息: json!({ "系统名称": td.系统信息.系统名称 }),
        报告结论: json!({ "结论": td.报告结论.结论 }),
        主要问题: if td.主要问题.问题.is_empty() {
            json!([])
        } else {
            json!([td.主要问题.问题])
        },
        漏洞扫描结果: json!({ "结果": td.漏洞扫描结果.结果 }),
        抽检系统范围: parse_json_value(td.抽检系统范围.对象.as_str()),
        资产情况: AssetInfo {
            机房: parse_json_value(td.资产情况.机房.as_str()),
            网络设备: parse_json_array_device(td.资产情况.网络设备.as_str()),
            安全设备: parse_json_array_device(td.资产情况.安全设备.as_str()),
            服务器: parse_json_array_device(td.资产情况.服务器.as_str()),
            终端: parse_json_array_device(td.资产情况.终端.as_str()),
            业务应用系统: parse_json_value(td.资产情况.业务应用系统.as_str()),
            安全相关人员: json!({}),
            数据库: vec![],
            其他: vec![],
            extra: json!({}),
        },
        收费通用要求: CommonRequirements {
            物理环境: json!({ "描述": td.收费通用要求.物理环境 }),
            通信网络: json!({ "描述": td.收费通用要求.通信网络 }),
            区域边界: json!({ "描述": td.收费通用要求.区域边界 }),
            计算环境: json!({}),
            extra: json!({}),
        },
        etc通用要求: CommonRequirements {
            物理环境: json!({ "描述": td.ETC通用要求.物理环境 }),
            通信网络: json!({ "描述": td.ETC通用要求.通信网络 }),
            区域边界: json!({}),
            计算环境: json!({}),
            extra: json!({}),
        },
        收费物联网扩展要求: json!({}),
        etc物联网扩展要求: json!({}),
        路段中心通用要求: json!({}),
        路段中心物联网扩展要求: json!({}),
    };
    let checklist = Checklist {
        title: payload.checklist.title.clone(),
        version: payload
            .checklist
            .version
            .as_deref()
            .unwrap_or("")
            .to_string(),
        items: payload
            .checklist
            .items
            .iter()
            .map(|i| ChecklistItem {
                id: i.id,
                is_important: i.is_important,
                requirement_item: i.requirement_item.clone(),
                requirement_subitem: i.requirement_subitem.as_deref().unwrap_or("").to_string(),
                requirement_detail: i.requirement_detail.clone(),
                judge_condition: i.judge_condition.clone(),
                compensation_measure: i.compensation_measure.clone(),
                detection_result: i.detection_result.clone(),
                conclusion: i.conclusion.clone(),
            })
            .collect(),
    };
    (config, checklist)
}

fn config_and_checklist_to_payload(
    config: &Config,
    checklist: Option<&Checklist>,
) -> (DtoTemplateData, DtoCheckList) {
    use crate::commands::grid_check_dto::*;
    let get_str = |v: &serde_json::Value, key: &str| {
        v.get(key)
            .and_then(|x| x.as_str())
            .unwrap_or("")
            .to_string()
    };
    let arr_first = |v: &serde_json::Value| {
        v.as_array()
            .and_then(|a| a.first())
            .and_then(|x| x.as_str())
            .unwrap_or("")
            .to_string()
    };
    let template_data = DtoTemplateData {
        基础信息: BaseInfo {
            编号: get_str(&config.基础信息, "编号"),
            报告时间: get_str(&config.基础信息, "报告时间"),
            检测时间: get_str(&config.基础信息, "检测时间"),
        },
        被测单位信息: UnitInfo {
            单位名称: get_str(&config.被测单位信息, "单位名称"),
            单位地址: get_str(&config.被测单位信息, "单位地址"),
        },
        系统信息: SystemInfo {
            系统名称: get_str(&config.系统信息, "系统名称"),
        },
        资产情况: DtoAssetInfo {
            机房: serde_json::to_string(&config.资产情况.机房).unwrap_or_else(|_| "{}".into()),
            网络设备: serde_json::to_string(&config.资产情况.网络设备)
                .unwrap_or_else(|_| "[]".into()),
            安全设备: serde_json::to_string(&config.资产情况.安全设备)
                .unwrap_or_else(|_| "[]".into()),
            服务器: serde_json::to_string(&config.资产情况.服务器)
                .unwrap_or_else(|_| "[]".into()),
            终端: serde_json::to_string(&config.资产情况.终端).unwrap_or_else(|_| "[]".into()),
            业务应用系统: serde_json::to_string(&config.资产情况.业务应用系统)
                .unwrap_or_else(|_| "[]".into()),
        },
        收费通用要求: ChargeCommon {
            物理环境: get_str(&config.收费通用要求.物理环境, "描述"),
            通信网络: get_str(&config.收费通用要求.通信网络, "描述"),
            区域边界: get_str(&config.收费通用要求.区域边界, "描述"),
        },
        ETC通用要求: EtcCommon {
            物理环境: get_str(&config.etc通用要求.物理环境, "描述"),
            通信网络: get_str(&config.etc通用要求.通信网络, "描述"),
        },
        漏洞扫描结果: VulnResult {
            结果: get_str(&config.漏洞扫描结果, "结果"),
        },
        抽检系统范围: SampleScope {
            对象: serde_json::to_string(&config.抽检系统范围).unwrap_or_else(|_| "{}".into()),
        },
        报告结论: ReportConclusion {
            结论: get_str(&config.报告结论, "结论"),
        },
        主要问题: MainProblem {
            问题: arr_first(&config.主要问题),
        },
    };
    let checklist_dto = checklist
        .map(|c| DtoCheckList {
            title: c.title.clone(),
            version: Some(c.version.clone()),
            items: c
                .items
                .iter()
                .map(|i| DtoCheckItem {
                    id: i.id,
                    is_important: i.is_important,
                    requirement_item: i.requirement_item.clone(),
                    requirement_subitem: Some(i.requirement_subitem.clone()),
                    requirement_detail: i.requirement_detail.clone(),
                    judge_condition: i.judge_condition.clone(),
                    compensation_measure: i.compensation_measure.clone(),
                    detection_result: i.detection_result.clone(),
                    conclusion: i.conclusion.clone(),
                })
                .collect(),
        })
        .unwrap_or_else(|| DtoCheckList {
            title: String::new(),
            version: None,
            items: vec![],
        });
    (template_data, checklist_dto)
}

// 创建配置文件
#[tauri::command]
pub fn create_temp(name: &str) -> String {
    match create_template(name) {
        Ok(file_path) => format!("✅ 创建成功！配置文件已生成{}", file_path),
        Err(e) => format!("❌ 创建失败：{}", e),
    }
}

// 加载配置文件
#[tauri::command]
pub fn load_conf(name: &str) -> String {
    match load_config(name) {
        Ok(_toml_config) => format!("✅ 配置文件加载成功"),
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
            fs::read_to_string(&prod_path).map_err(|e| format!("读取 toml 失败: {}", e))?
        } else {
            // 开发环境路径
            let dev_path = PathBuf::from("resources").join(format!("{}.toml", name));
            fs::read_to_string(&dev_path).map_err(|e| format!("读取 toml 失败: {}", e))?
        }
    } else {
        // 开发环境路径
        let dev_path = PathBuf::from("resources").join(format!("{}.toml", name));
        fs::read_to_string(&dev_path).map_err(|e| format!("读取 toml 失败: {}", e))?
    };

    // 解析 TOML
    let config: TemplateConfig =
        toml::from_str(&toml_content).map_err(|e| format!("解析 TOML 失败: {}", e))?;

    // 转换为 JSON 返回
    serde_json::to_string(&config).map_err(|e| format!("转换 JSON 失败: {}", e))
}

// 加载测评项清单（如 收费.toml），返回第一个 checklist 供前端展示
#[tauri::command]
pub fn load_checklist(app: tauri::AppHandle, name: &str) -> Result<Checklist, String> {
    let toml_content = if let Ok(resource_path) = app.path().resource_dir() {
        let prod_path = resource_path.join(format!("{}.toml", name));
        if prod_path.exists() {
            fs::read_to_string(&prod_path).map_err(|e| format!("读取 toml 失败: {}", e))?
        } else {
            let dev_path = PathBuf::from("resources").join(format!("{}.toml", name));
            fs::read_to_string(&dev_path).map_err(|e| format!("读取 toml 失败: {}", e))?
        }
    } else {
        let dev_path = PathBuf::from("resources").join(format!("{}.toml", name));
        fs::read_to_string(&dev_path).map_err(|e| format!("读取 toml 失败: {}", e))?
    };

    let config: TemplateConfig =
        toml::from_str(&toml_content).map_err(|e| format!("解析 TOML 失败: {}", e))?;

    config
        .checklist
        .into_iter()
        .next()
        .ok_or_else(|| format!("模板 {} 中未包含测评清单(checklist)", name))
}

// 保存前端传来的测评配置（DynamicConfig），以 JSON 形式持久化到本地
#[tauri::command]
pub fn save_dynamic_config(
    app: tauri::AppHandle,
    name: &str,
    data: DynamicConfig,
) -> Result<String, String> {
    let base_dir = ensure_app_data_dir(&app)?.join("data");
    fs::create_dir_all(&base_dir).map_err(|e| format!("创建数据目录失败: {}", e))?;
    let path = base_dir.join(format!("{}.json", sanitize_filename(name)));

    // 序列化为 JSON
    let json =
        serde_json::to_string_pretty(&data).map_err(|e| format!("序列化 JSON 失败: {}", e))?;

    // 写入文件
    fs::write(&path, json).map_err(|e| format!("写入文件失败: {}", e))?;

    Ok(format!("{}", path.display()))
}

// 加载已保存的测评配置，返回给前端编辑
#[tauri::command]
pub fn load_dynamic_config(app: tauri::AppHandle, name: &str) -> Result<DynamicConfig, String> {
    let base_dir = ensure_app_data_dir(&app)?.join("data");
    let path = base_dir.join(format!("{}.json", sanitize_filename(name)));

    let content = fs::read_to_string(&path).map_err(|e| format!("读取配置失败: {}", e))?;

    let config: DynamicConfig =
        serde_json::from_str(&content).map_err(|e| format!("解析 JSON 失败: {}", e))?;

    Ok(config)
}

// 并网检测整体提交：TemplateData + CheckList，一次性保存为 JSON
#[tauri::command]
pub fn submit_grid_check(
    app: tauri::AppHandle,
    name: &str,
    payload: GridCheckPayload,
) -> Result<String, String> {
    let base_dir = ensure_app_data_dir(&app)?.join("grid-check");
    fs::create_dir_all(&base_dir).map_err(|e| format!("创建并网检测目录失败: {}", e))?;
    let path = base_dir.join(format!("{}.json", sanitize_filename(name)));

    let json =
        serde_json::to_string_pretty(&payload).map_err(|e| format!("序列化 JSON 失败: {}", e))?;

    fs::write(&path, json).map_err(|e| format!("写入文件失败: {}", e))?;

    Ok(format!("{}", path.display()))
}

// ---------- 项目 TOML（与 template.toml 同构 + checklist）----------

fn read_resource_toml(app: &tauri::AppHandle, name: &str) -> Result<String, String> {
    let mut tried: Vec<PathBuf> = vec![];
    let filename = format!("{}.toml", name);

    if let Ok(resource_path) = app.path().resource_dir() {
        tried.push(resource_path.join(&filename));
    }
    tried.push(PathBuf::from("resources").join(&filename));
    tried.push(PathBuf::from("src-tauri/resources").join(&filename));
    tried.push(PathBuf::from("src-tauri").join(&filename));

    for p in &tried {
        if p.exists() {
            return fs::read_to_string(p).map_err(|e| format!("读取失败: {} ({})", e, p.display()));
        }
    }

    Err(format!(
        "未找到资源文件：{}（已尝试：{}）",
        filename,
        tried
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>()
            .join("；")
    ))
}

/// 创建项目：生成 projects/{name}.toml（基于 template.toml + 收费.toml 测评项，结果为空）
#[tauri::command]
pub fn create_project(app: tauri::AppHandle, name: &str) -> Result<String, String> {
    let base_dir = ensure_app_data_dir(&app)?;
    let template = read_resource_toml(&app, "template")?;
    let checklist = read_resource_toml(&app, "收费")?;
    let path = create_project_toml(&base_dir, name, &template, &checklist)
        .map_err(|e: TemplateError| e.to_string())?;
    Ok(path.display().to_string())
}

/// 保存项目：将当前表单数据写入 projects/{name}.toml
#[tauri::command]
pub fn save_project(
    app: tauri::AppHandle,
    name: &str,
    payload: GridCheckPayload,
) -> Result<String, String> {
    let base_dir = ensure_app_data_dir(&app)?;
    let (config, checklist) = payload_to_config_and_checklist(&payload);
    let path = save_project_toml(&base_dir, name, config, checklist)
        .map_err(|e: TemplateError| e.to_string())?;
    Ok(path.display().to_string())
}

/// 加载项目：读取 projects/{name}.toml，返回 { templateData, checklist } 供前端回填
#[tauri::command]
pub fn load_project(app: tauri::AppHandle, name: &str) -> Result<GridCheckPayload, String> {
    let base_dir = ensure_app_data_dir(&app)?;
    let (config, checklist_opt) =
        load_project_toml_at(&base_dir, name).map_err(|e: TemplateError| e.to_string())?;
    let (template_data, checklist_dto) =
        config_and_checklist_to_payload(&config, checklist_opt.as_ref());
    Ok(GridCheckPayload {
        template_data,
        checklist: checklist_dto,
    })
}

/// 列出已有项目名称（projects/*.toml）
#[tauri::command]
pub fn list_project_names(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    let base_dir = ensure_app_data_dir(&app)?;
    list_projects(&base_dir).map_err(|e: TemplateError| e.to_string())
}

fn ensure_app_data_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("获取应用数据目录失败: {}", e))?;
    fs::create_dir_all(&dir).map_err(|e| format!("创建应用数据目录失败: {}", e))?;
    Ok(dir)
}

fn sanitize_filename(input: &str) -> String {
    let mut s = input
        .chars()
        .map(|c| match c {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect::<String>();
    s = s.trim().trim_end_matches('.').to_string();
    if s.is_empty() {
        "untitled".to_string()
    } else {
        s
    }
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn json_pretty(v: &serde_json::Value) -> String {
    serde_json::to_string_pretty(v).unwrap_or_else(|_| v.to_string())
}

fn now_unix_secs() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[tauri::command]
pub fn generate_grid_check_report(
    app: tauri::AppHandle,
    name: &str,
    payload: GridCheckPayload,
) -> Result<String, String> {
    let base_dir = ensure_app_data_dir(&app)?;
    let reports_dir = base_dir.join("reports");
    fs::create_dir_all(&reports_dir).map_err(|e| format!("创建 reports 目录失败: {}", e))?;

    let (config, checklist) = payload_to_config_and_checklist(&payload);

    let report_name = sanitize_filename(name);
    let ts = now_unix_secs();
    let path = reports_dir.join(format!("{}-{}.html", report_name, ts));

    let mut html = String::new();
    html.push_str("<!doctype html><html><head><meta charset=\"utf-8\">");
    html.push_str(&format!(
        "<title>并网检测报告 - {}</title>",
        escape_html(&report_name)
    ));
    html.push_str(
        "<style>
body{font-family:system-ui,Segoe UI,Arial,Helvetica,sans-serif;margin:24px;color:#111}
h1{margin:0 0 8px 0;font-size:22px}
h2{margin:22px 0 10px 0;font-size:16px;border-left:4px solid #3b82f6;padding-left:10px}
.meta{color:#555;margin-bottom:18px}
table{border-collapse:collapse;width:100%;margin:10px 0}
th,td{border:1px solid #e5e7eb;padding:8px;vertical-align:top;font-size:13px}
th{background:#f9fafb;text-align:left}
pre{white-space:pre-wrap;word-break:break-word;margin:0}
.tag{display:inline-block;padding:2px 8px;border-radius:999px;font-size:12px;background:#eff6ff;color:#1d4ed8}
        </style></head><body>",
    );

    html.push_str("<h1>并网检测报告</h1>");
    html.push_str(&format!(
        "<div class=\"meta\">项目：<span class=\"tag\">{}</span></div>",
        escape_html(&report_name)
    ));

    // 基础信息 / 单位信息 / 系统信息 / 结论 / 漏洞扫描 / 抽检范围
    let sections: Vec<(&str, &serde_json::Value)> = vec![
        ("基础信息", &config.基础信息),
        ("被测单位信息", &config.被测单位信息),
        ("系统信息", &config.系统信息),
        ("报告结论", &config.报告结论),
        ("主要问题", &config.主要问题),
        ("漏洞扫描结果", &config.漏洞扫描结果),
        ("抽检系统范围", &config.抽检系统范围),
    ];
    for (title, v) in sections {
        html.push_str(&format!("<h2>{}</h2>", escape_html(title)));
        html.push_str("<pre>");
        html.push_str(&escape_html(&json_pretty(v)));
        html.push_str("</pre>");
    }

    // 资产情况
    html.push_str("<h2>资产情况</h2>");
    html.push_str("<table><thead><tr><th>类别</th><th>内容</th></tr></thead><tbody>");
    html.push_str(&format!(
        "<tr><td>机房</td><td><pre>{}</pre></td></tr>",
        escape_html(&json_pretty(&config.资产情况.机房))
    ));
    let devices = vec![
        ("网络设备", &config.资产情况.网络设备),
        ("安全设备", &config.资产情况.安全设备),
        ("服务器", &config.资产情况.服务器),
        ("终端", &config.资产情况.终端),
        ("数据库", &config.资产情况.数据库),
        ("其他", &config.资产情况.其他),
    ];
    for (k, list) in devices {
        let v = serde_json::to_value(list).unwrap_or_else(|_| json!([]));
        html.push_str(&format!(
            "<tr><td>{}</td><td><pre>{}</pre></td></tr>",
            escape_html(k),
            escape_html(&json_pretty(&v))
        ));
    }
    html.push_str(&format!(
        "<tr><td>业务应用系统</td><td><pre>{}</pre></td></tr>",
        escape_html(&json_pretty(&config.资产情况.业务应用系统))
    ));
    html.push_str(&format!(
        "<tr><td>安全相关人员</td><td><pre>{}</pre></td></tr>",
        escape_html(&json_pretty(&config.资产情况.安全相关人员))
    ));
    html.push_str("</tbody></table>");

    // 测评项
    html.push_str("<h2>测评项清单（收费）</h2>");
    html.push_str(&format!(
        "<div class=\"meta\">标题：{}；版本：{}</div>",
        escape_html(&checklist.title),
        escape_html(&checklist.version)
    ));
    html.push_str("<table><thead><tr><th>ID</th><th>重要</th><th>条款</th><th>子项</th><th>要求</th><th>检测结果</th><th>结论</th></tr></thead><tbody>");
    for ChecklistItem {
        id,
        is_important,
        requirement_item,
        requirement_subitem,
        requirement_detail,
        detection_result,
        conclusion,
        ..
    } in checklist.items
    {
        html.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td><pre>{}</pre></td><td><pre>{}</pre></td></tr>",
            id,
            if is_important { "是" } else { "否" },
            escape_html(&requirement_item),
            escape_html(&requirement_subitem),
            escape_html(&requirement_detail),
            escape_html(&detection_result),
            escape_html(&conclusion),
        ));
    }
    html.push_str("</tbody></table>");

    html.push_str("</body></html>");
    fs::write(&path, html).map_err(|e| format!("写入报告失败: {}", e))?;

    Ok(path.display().to_string())
}
