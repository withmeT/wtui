use serde::{Deserialize, Serialize};

// 与前端 src/types/index.ts 中的 TemplateData 对齐
#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateData {
    pub 基础信息: BaseInfo,
    pub 被测单位信息: UnitInfo,
    pub 系统信息: SystemInfo,
    pub 资产情况: AssetInfo,
    pub 收费通用要求: ChargeCommon,
    pub ETC通用要求: EtcCommon,
    pub 漏洞扫描结果: VulnResult,
    pub 抽检系统范围: SampleScope,
    pub 报告结论: ReportConclusion,
    pub 主要问题: MainProblem,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseInfo {
    pub 编号: String,
    pub 报告时间: String,
    pub 检测时间: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitInfo {
    pub 单位名称: String,
    pub 单位地址: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub 系统名称: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetInfo {
    pub 机房: String,
    pub 网络设备: String,
    pub 安全设备: String,
    pub 服务器: String,
    pub 终端: String,
    pub 业务应用系统: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChargeCommon {
    pub 物理环境: String,
    pub 通信网络: String,
    pub 区域边界: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EtcCommon {
    pub 物理环境: String,
    pub 通信网络: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VulnResult {
    pub 结果: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SampleScope {
    pub 对象: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportConclusion {
    pub 结论: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainProblem {
    pub 问题: String,
}

// 与前端 src/types/index.ts 中的 CheckItem / CheckList 对齐
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckItem {
    pub id: i32,
    pub is_important: bool,
    pub requirement_item: String,
    pub requirement_subitem: Option<String>,
    pub requirement_detail: String,
    pub judge_condition: String,
    pub compensation_measure: String,
    pub detection_result: String,
    pub conclusion: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckList {
    pub title: String,
    pub version: Option<String>,
    pub items: Vec<CheckItem>,
}

// 提交整体 payload
#[derive(Debug, Serialize, Deserialize)]
pub struct GridCheckPayload {
    pub template_data: TemplateData,
    pub checklist: CheckList,
}

