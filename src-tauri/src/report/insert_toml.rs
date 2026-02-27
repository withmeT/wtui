use crate::report::template::{Config, Device, CheckDetail,Result,TemplateError};
use serde_json::json;
use std::path::{Path};
use std::fs;

pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    /// 创建新的配置构建器
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    /// 创建一个空的配置（只有基本框架）
    pub fn new_empty() -> Self {
        Self {
            config: Config::default(),
        }
    }

    /// 从现有配置创建构建器
    pub fn from_config(config: Config) -> Self {
        Self { config }
    }

    /// 从 TOML 文件加载配置
    pub fn from_toml_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(Self { config })
    }

    /// 从 JSON 文件加载配置
    pub fn from_json_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(Self { config })
    }

    /// 获取当前配置的不可变引用（用于查看）
    pub fn get_config(&self) -> &Config {
        &self.config
    }

    /// 获取当前配置的可变引用（用于直接修改）
    pub fn get_config_mut(&mut self) -> &mut Config {
        &mut self.config
    }

    // ========== 基础信息设置 ==========
    pub fn set_基础信息(mut self, 项目名称: &str, 测评日期: &str, 测评人员: &str) -> Self {
        self.config.基础信息 = json!({
            "项目名称": 项目名称,
            "测评日期": 测评日期,
            "测评人员": 测评人员,
        });
        self
    }

    /// 添加自定义基础信息字段
    pub fn add_基础信息_字段(mut self, key: &str, value: serde_json::Value) -> Self {
        if let Some(obj) = self.config.基础信息.as_object_mut() {
            obj.insert(key.to_string(), value);
        }
        self
    }

    // ========== 被测单位信息 ==========
    pub fn set_被测单位信息(mut self, 单位名称: &str, 联系人: &str, 电话: &str) -> Self {
        self.config.被测单位信息 = json!({
            "单位名称": 单位名称,
            "联系人": 联系人,
            "电话": 电话,
        });
        self
    }

    // ========== 系统信息 ==========
    pub fn set_系统信息(mut self, 系统名称: &str, 系统等级: &str) -> Self {
        self.config.系统信息 = json!({
            "系统名称": 系统名称,
            "系统等级": 系统等级,
        });
        self
    }

    // ========== 报告结论 ==========
    pub fn set_报告结论(mut self, 结论: &str) -> Self {
        self.config.报告结论 = json!({ "结论": 结论 });
        self
    }

    // ========== 主要问题 ==========
    pub fn add_主要问题(mut self, 问题: &str) -> Self {
        if let Some(arr) = self.config.主要问题.as_array_mut() {
            arr.push(json!(问题));
        } else {
            self.config.主要问题 = json!([问题]);
        }
        self
    }

    // ========== 资产情况 - 设备管理 ==========
    
    /// 添加网络设备
    pub fn add_网络设备(mut self, device: Device) -> Self {
        self.config.资产情况.网络设备.push(device);
        self
    }

    /// 快速添加网络设备
    pub fn add_网络设备_快速(mut self, 设备名称: &str, 品牌: &str, 型号: &str, 数量: u32) -> Self {
        self.config.资产情况.网络设备.push(Device {
            设备名称: 设备名称.to_string(),
            品牌: 品牌.to_string(),
            型号: 型号.to_string(),
            用途: String::new(),
            数量,
            extra: json!({}),
        });
        self
    }

    /// 添加安全设备
    pub fn add_安全设备(mut self, device: Device) -> Self {
        self.config.资产情况.安全设备.push(device);
        self
    }

    /// 快速添加安全设备
    pub fn add_安全设备_快速(mut self, 设备名称: &str, 品牌: &str, 型号: &str, 用途: &str, 数量: u32) -> Self {
        self.config.资产情况.安全设备.push(Device {
            设备名称: 设备名称.to_string(),
            品牌: 品牌.to_string(),
            型号: 型号.to_string(),
            用途: 用途.to_string(),
            数量,
            extra: json!({}),
        });
        self
    }

    /// 添加服务器
    pub fn add_服务器(mut self, device: Device) -> Self {
        self.config.资产情况.服务器.push(device);
        self
    }

    /// 快速添加服务器（带操作系统）
    pub fn add_服务器_快速(mut self, 设备名称: &str, 品牌: &str, 型号: &str, 操作系统: &str, 数量: u32) -> Self {
        self.config.资产情况.服务器.push(Device {
            设备名称: 设备名称.to_string(),
            品牌: 品牌.to_string(),
            型号: 型号.to_string(),
            用途: String::new(),
            数量,
            extra: json!({ "操作系统": 操作系统 }),
        });
        self
    }

    /// 添加数据库
    pub fn add_数据库(mut self, device: Device) -> Self {
        self.config.资产情况.数据库.push(device);
        self
    }

    /// 快速添加数据库
    pub fn add_数据库_快速(mut self, 设备名称: &str, 品牌: &str, 型号: &str, 数量: u32) -> Self {
        self.config.资产情况.数据库.push(Device {
            设备名称: 设备名称.to_string(),
            品牌: 品牌.to_string(),
            型号: 型号.to_string(),
            用途: String::new(),
            数量,
            extra: json!({}),
        });
        self
    }

    /// 添加终端
    pub fn add_终端(mut self, device: Device) -> Self {
        self.config.资产情况.终端.push(device);
        self
    }

    /// 快速添加终端
    pub fn add_终端_快速(mut self, 设备名称: &str, 品牌: &str, 型号: &str, 数量: u32) -> Self {
        self.config.资产情况.终端.push(Device {
            设备名称: 设备名称.to_string(),
            品牌: 品牌.to_string(),
            型号: 型号.to_string(),
            用途: String::new(),
            数量,
            extra: json!({}),
        });
        self
    }

    /// 添加其他设备
    pub fn add_其他设备(mut self, device: Device) -> Self {
        self.config.资产情况.其他.push(device);
        self
    }

    /// 快速添加其他设备
    pub fn add_其他设备_快速(mut self, 设备名称: &str, 品牌: &str, 型号: &str, 数量: u32) -> Self {
        self.config.资产情况.其他.push(Device {
            设备名称: 设备名称.to_string(),
            品牌: 品牌.to_string(),
            型号: 型号.to_string(),
            用途: String::new(),
            数量,
            extra: json!({}),
        });
        self
    }

    // ========== 检查项管理（只记录检查项名称 -> 结果 + 符合情况）==========
    
    /// 添加收费通用要求检查项
    pub fn add_收费通用要求_检查项(
        mut self,
        检查项: &str,  // 如 "a) 用户身份标识唯一性"
        结果: &str,     // 如 "符合"
        符合情况: &str, // 如 "已配置RBAC"
    ) -> Self {
        let v = json!({
            "结果": 结果,
            "符合情况": 符合情况,
        });
        if let Some(obj) = self.config.收费通用要求.计算环境.as_object_mut() {
            obj.insert(检查项.to_string(), v);
        } else {
            self.config.收费通用要求.计算环境 = json!({ 检查项: v });
        }
        self
    }

    /// 添加etc通用要求检查项
    pub fn add_etc通用要求_检查项(
        mut self,
        检查项: &str,
        结果: &str,
        符合情况: &str,
    ) -> Self {
        let v = json!({
            "结果": 结果,
            "符合情况": 符合情况,
        });
        if let Some(obj) = self.config.etc通用要求.计算环境.as_object_mut() {
            obj.insert(检查项.to_string(), v);
        } else {
            self.config.etc通用要求.计算环境 = json!({ 检查项: v });
        }
        self
    }

    // ========== 构建最终配置 ==========
    
    /// 构建并返回最终配置
    pub fn build(self) -> Config {
        self.config
    }

    /// 构建并保存为 TOML 文件
    pub fn save_to_toml(self, path: &str) -> Result<Config> {
        let config = self.config;
        let toml_string = toml::to_string_pretty(&config)?;
        std::fs::write(path, toml_string)?;
        Ok(config)
    }

    /// 构建并保存为 JSON 文件（调试用）
    pub fn save_to_json(self, path: &str) -> Result<Config> {
        let config = self.config;
        let json_string = serde_json::to_string_pretty(&config)?;
        std::fs::write(path, json_string)?;
        Ok(config)
    }
}

// ========== 设备构建器 ==========
pub struct DeviceBuilder {
    device: Device,
}

impl DeviceBuilder {
    pub fn new(设备名称: &str) -> Self {
        Self {
            device: Device {
                设备名称: 设备名称.to_string(),
                品牌: String::new(),
                型号: String::new(),
                用途: String::new(),
                数量: 1,
                extra: json!({}),
            },
        }
    }

    pub fn 品牌(mut self, 品牌: &str) -> Self {
        self.device.品牌 = 品牌.to_string();
        self
    }

    pub fn 型号(mut self, 型号: &str) -> Self {
        self.device.型号 = 型号.to_string();
        self
    }

    pub fn 用途(mut self, 用途: &str) -> Self {
        self.device.用途 = 用途.to_string();
        self
    }

    pub fn 数量(mut self, 数量: u32) -> Self {
        self.device.数量 = 数量;
        self
    }

    /// 添加额外字段（如操作系统、IP地址等）
    pub fn add_extra(mut self, key: &str, value: serde_json::Value) -> Self {
        if let Some(obj) = self.device.extra.as_object_mut() {
            obj.insert(key.to_string(), value);
        } else {
            self.device.extra = json!({ key: value });
        }
        self
    }

    pub fn build(self) -> Device {
        self.device
    }
}

// ========== 默认实现 ==========
impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}


pub fn create_template(name: &str) -> Result<String> {
    let target_path = format!("{}.toml", name);
    // 创建空配置并保存
    ConfigBuilder::new_empty()
        .save_to_toml(&target_path)?;               
    
    println!("✅ 空配置创建成功！{}",target_path);
    Ok(target_path)
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

