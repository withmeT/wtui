// 导航项类型
export interface NavItem {
  id: number;
  name: string;
  icon: string;
}

// 标签页类型
export interface TabItem {
  key: string;
  name: string;
}

// 提交结果类型
export interface SubmitResult {
  type: 'success' | 'error';
  message: string;
}

// 测评项类型
export interface CheckItem {
  id: number;
  is_important: boolean;
  requirement_item: string;
  requirement_subitem?: string;
  requirement_detail: string;
  judge_condition: string;
  compensation_measure: string;
  detection_result: string;
  conclusion: '' | '视同符合' | '符合' | '部分符合' | '不符合' | '不适用';
}

// 测评清单类型
export interface CheckList {
  title: string;
  version?: string;
  items: CheckItem[];
}

// 模板数据类型
export interface TemplateData {
  基础信息: {
    编号: string;
    报告时间: string;
    检测时间: string;
  };
  被测单位信息: {
    单位名称: string;
    单位地址: string;
  };
  系统信息: {
    系统名称: string;
  };
  资产情况: {
    机房: string;
    网络设备: string;
    安全设备: string;
    服务器: string;
    终端: string;
    业务应用系统: string;
  };
  收费通用要求: {
    物理环境: string;
    通信网络: string;
    区域边界: string;
  };
  ETC通用要求: {
    物理环境: string;
    通信网络: string;
  };
  漏洞扫描结果: {
    结果: string;
  };
  抽检系统范围: {
    对象: string;
  };
  报告结论: {
    结论: string;
  };
  主要问题: {
    问题: string;
  };
}