// temp.rs

use thiserror::Error;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Error, Debug)]
pub enum TemplateError {
    #[error("文件IO错误: {0}")]
    IoError(#[from] std::io::Error),

    #[error("模板文件不存在: {0}")]
    TemplateNotFound(String),

    #[error("TOML 解析（反序列化）错误: {0}")]
    TomlDeError(#[from] toml::de::Error),

    #[error("TOML 序列化错误: {0}")]
    TomlSerError(#[from] toml::ser::Error),

    #[error("JSON 序列化/反序列化错误: {0}")]
    JsonError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, TemplateError>;

/// 单个检查项（只记录结果 + 符合情况）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CheckDetail {
    pub 结果: String,
    pub 符合情况: String,
}

/// 设备通用结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct  Device {
    pub 设备名称: String,
    #[serde(default)]
    pub 品牌: String,
    #[serde(default)]
    pub 型号: String,
    #[serde(default)]
    pub 用途: String,
    #[serde(default)]
    pub 数量: u32,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// 完整的配置文件数据结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(rename = "基础信息", default = "default_object")]
    pub 基础信息: serde_json::Value,

    #[serde(rename = "被测单位信息", default = "default_object")]
    pub 被测单位信息: serde_json::Value,

    #[serde(rename = "系统信息", default = "default_object")]
    pub 系统信息: serde_json::Value,

    #[serde(rename = "报告结论", default = "default_object")]
    pub 报告结论: serde_json::Value,

    #[serde(rename = "主要问题", default = "default_array")]
    pub 主要问题: serde_json::Value,

    #[serde(rename = "资产情况")]
    pub 资产情况: AssetInfo,

    #[serde(rename = "收费通用要求")]
    pub 收费通用要求: CommonRequirements,

    #[serde(rename = "收费物联网扩展要求", default = "default_object")]
    pub 收费物联网扩展要求: serde_json::Value,

    #[serde(rename = "ETC通用要求")]
    pub etc通用要求: CommonRequirements,

    #[serde(rename = "ETC物联网扩展要求", default = "default_object")]
    pub etc物联网扩展要求: serde_json::Value,

    #[serde(rename = "路段中心通用要求", default = "default_object")]
    pub 路段中心通用要求: serde_json::Value,

    #[serde(rename = "路段中心物联网扩展要求", default = "default_object")]
    pub 路段中心物联网扩展要求: serde_json::Value,

    #[serde(rename = "漏洞扫描结果", default = "default_object")]
    pub 漏洞扫描结果: serde_json::Value,

    #[serde(rename = "抽检系统范围", default = "default_object")]
    pub 抽检系统范围: serde_json::Value,
}

// 默认值辅助函数
fn default_object() -> serde_json::Value {
    json!({})
}

fn default_array() -> serde_json::Value {
    json!([])
}

impl Default for Config {
    fn default() -> Self {
        Self {
            基础信息: json!({}),
            被测单位信息: json!({}),
            系统信息: json!({}),
            报告结论: json!({}),
            主要问题: json!({}),
            资产情况: AssetInfo::default(),
            收费通用要求: CommonRequirements::default(),
            收费物联网扩展要求: json!({}),
            etc通用要求: CommonRequirements::default(),
            etc物联网扩展要求: json!({}),
            路段中心通用要求: json!({}),
            路段中心物联网扩展要求: json!({}),
            漏洞扫描结果: json!({}),
            抽检系统范围: json!({}),
        }
    }
}

/// 资产情况结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssetInfo {
    #[serde(default = "default_object")]
    pub 机房: serde_json::Value,

    #[serde(default)]
    pub 网络设备: Vec<Device>,

    #[serde(default)]
    pub 安全设备: Vec<Device>,

    #[serde(default)]
    pub 服务器: Vec<Device>,

    #[serde(default)]
    pub 数据库: Vec<Device>,

    #[serde(default)]
    pub 终端: Vec<Device>,

    #[serde(default = "default_object")]
    pub 业务应用系统: serde_json::Value,

    #[serde(default = "default_object")]
    pub 安全相关人员: serde_json::Value,
    
    #[serde(default)]
    pub 其他: Vec<Device>,

    #[serde(flatten)]
    pub extra: serde_json::Value,
}

impl Default for AssetInfo {
    fn default() -> Self {
        Self {
            机房: json!({}),
            网络设备: Vec::new(),
            安全设备: Vec::new(),
            服务器: Vec::new(),
            数据库: Vec::new(),
            终端: Vec::new(),
            业务应用系统: json!({}),
            安全相关人员: json!({}),
            其他: Vec::new(),
            extra: json!({}),
        }
    }
}

/// 通用要求结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommonRequirements {
    #[serde(default = "default_object")]
    pub 物理环境: serde_json::Value,

    #[serde(default = "default_object")]
    pub 通信网络: serde_json::Value,

    #[serde(default = "default_object")]
    pub 区域边界: serde_json::Value,

    #[serde(default)]
    pub 计算环境: std::collections::HashMap<String, CheckDetail>,

    #[serde(flatten)]
    pub extra: serde_json::Value,
}

impl Default for CommonRequirements {
    fn default() -> Self {
        Self {
            物理环境: json!({}),
            通信网络: json!({}),
            区域边界: json!({}),
            计算环境: std::collections::HashMap::new(),
            extra: json!({}),
        }
    }
}


#[derive(Debug, Deserialize, Serialize)]
pub struct ChecklistItem {
    pub id: i32,
    pub is_important: bool,
    pub requirement_item: String,
    pub requirement_subitem: String,
    pub requirement_detail: String,
    pub judge_condition: String,
    pub compensation_measure: String,
    pub detection_result: String,
    pub conclusion: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Checklist {
    pub title: String,
    pub version: String,
    pub items: Vec<ChecklistItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TemplateConfig {
    pub checklist: Vec<Checklist>,
}