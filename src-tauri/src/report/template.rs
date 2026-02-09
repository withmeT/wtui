use thiserror::Error;
use serde::{Deserialize, Serialize};


// ========== 保留你原有错误类型，新增 TOML 解析错误 ==========
#[derive(Error, Debug)]
pub enum TemplateError {
    #[error("文件IO错误: {0}")]
    IoError(#[from] std::io::Error), // IO错误

    #[error("模板文件不存在: {0}")]
    TemplateNotFound(String), // 自定义错误

    #[error("TOML 解析（反序列化）错误: {0}")]
    TomlDeError(#[from] toml::de::Error), // TOML解析错误

    #[error("TOML 序列化错误: {0}")]
    TomlSerError(#[from] toml::ser::Error), // 新增：TOML序列化错误

    #[error("JSON 序列化/反序列化错误: {0}")]
    JsonError(#[from] serde_json::Error), // 可选：新增JSON错误（兼容扩展字段）
}

pub type Result<T> = std::result::Result<T, TemplateError>;

// ========== 定义和 TOML 匹配的数据结构（兼容中文/动态字段） ==========
/// 单个检查项（结果 + 符合情况）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CheckDetail {
    pub 结果: String,
    pub 符合情况: String,
}

/// 设备通用结构（兼容扩展字段）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Device {
    pub 设备名称: String,
    #[serde(default)] // 缺省值为空字符串
    pub 品牌: String,
    #[serde(default)]
    pub 型号: String,
    #[serde(default)]
    pub 用途: String,
    #[serde(default)]
    pub 数量: u32,
    #[serde(flatten)] // 兼容扩展字段（如服务器的操作系统、终端的设备用途）
    pub extra: serde_json::Value,
}

/// 完整的配置文件数据结构（和你的 TOML 结构对齐）
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Config {
    #[serde(rename = "基础信息")]
    pub 基础信息: serde_json::Value,

    #[serde(rename = "被测单位信息")]
    pub 被测单位信息: serde_json::Value,

    #[serde(rename = "系统信息")]
    pub 系统信息: serde_json::Value,

    #[serde(rename = "报告结论")]
    pub 报告结论: serde_json::Value,

    #[serde(rename = "主要问题")]
    pub 主要问题: serde_json::Value,

    #[serde(rename = "资产情况")]
    pub 资产情况: AssetInfo,

    #[serde(rename = "收费通用要求")]
    pub 收费通用要求: CommonRequirements,

    #[serde(rename = "收费物联网扩展要求")]
    pub 收费物联网扩展要求: serde_json::Value,

    #[serde(rename = "ETC通用要求")]
    pub ETC通用要求: CommonRequirements,

    #[serde(rename = "ETC物联网扩展要求")]
    pub ETC物联网扩展要求: serde_json::Value,

    #[serde(rename = "路段中心通用要求")]
    pub 路段中心通用要求: serde_json::Value,

    #[serde(rename = "路段中心物联网扩展要求")]
    pub 路段中心物联网扩展要求: serde_json::Value,

    #[serde(rename = "漏洞扫描结果")]
    pub 漏洞扫描结果: serde_json::Value,

    #[serde(rename = "抽检系统范围")]
    pub 抽检系统范围: serde_json::Value,
}

/// 资产情况结构（支持动态设备列表）
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AssetInfo {
    #[serde(default)]
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
    pub 终端设备: Vec<Device>,

    #[serde(default)]
    pub 应用系统: Vec<Device>,

    #[serde(default)]
    pub 其他: Vec<Device>,

    #[serde(flatten)] // 兼容资产情况的扩展字段
    pub extra: serde_json::Value,
}

/// 通用要求结构（包含计算环境等动态检查项）
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CommonRequirements {
    #[serde(default)]
    pub 物理环境: serde_json::Value,

    #[serde(default)]
    pub 通信网络: serde_json::Value,

    #[serde(default)]
    pub 区域边界: serde_json::Value,

    #[serde(default)]
    pub 计算环境: std::collections::HashMap<String, std::collections::HashMap<String, CheckDetail>>,

    #[serde(flatten)] // 兼容通用要求的扩展字段
    pub extra: serde_json::Value,
}