use serde::{Deserialize, Serialize};
use thiserror::Error;

// 自定义错误类型
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("文件IO错误: {0}")]
    IoError(#[from] std::io::Error),
    #[error("TOML解析/序列化错误: {0}")]
    TomlError(#[from] toml::de::Error),
    #[error("JSON反序列化错误: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("模板文件不存在: {0}")]
    TemplateNotFound(String),
}
pub type Result<T> = std::result::Result<T, ConfigError>;

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckDetail {
    pub 结果: String,        // 原有的结果字段
    pub 符合情况: String,    // 新增的符合情况字段
}

// ===== 核心：动态数据结构体（与前端传参对齐）=====
// 设备通用结构（资产情况中的单个设备）
#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub 设备名称: String,
    pub 品牌: Option<String>,       // 可选字段（部分设备无品牌）
    pub 型号: Option<String>,
    pub 用途: Option<String>,
    pub 数量: Option<u32>,
    #[serde(flatten)] // 兼容扩展字段（如服务器的操作系统、终端的设备用途等）
    pub extra: serde_json::Value,
}

// 计算环境检查项（支持动态增删check）
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckItem {
    #[serde(flatten)] // 动态key（check1/check2/...）
    pub checks: std::collections::HashMap<String, String>,
}

// 前端传递的动态数据整体结构
#[derive(Debug, Serialize, Deserialize)]
pub struct DynamicConfig {
    // 资产情况：各设备列表（空数组则删除对应字段）
    pub 机房: Option<serde_json::Value>, // 单对象
    pub 网络设备: Option<Vec<Device>>,    // 多设备（数组）
    pub 安全设备: Option<Vec<Device>>,
    pub 服务器: Option<Vec<Device>>,
    pub 终端: Option<Vec<Device>>,
    pub 业务应用系统: Option<Vec<Device>>,
    pub 安全相关人员: Option<Vec<Device>>,

    // 计算环境：动态检查项（空则删除）
    pub 收费通用要求_计算环境: Option<serde_json::Value>, // { "XX交换机": { "check1": "结果", ... } }
    pub etc通用要求_计算环境: Option<serde_json::Value>,
}

