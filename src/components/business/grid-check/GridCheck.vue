<template>
  <div class="grid-check">
    <div class="card-header">
      <div class="card-header-icon">📅</div>
      <h2 class="card-title">并网检测</h2>
    </div>
    <div class="card-content">
      <p class="card-main-desc">并网等保合规性检测，基于标准化模板自动生成检测报告</p>

      <!-- 项目 TOML：创建 / 加载 / 当前项目 -->
      <div class="project-bar">
        <div class="project-row">
          <span class="project-label">当前项目：</span>
          <select v-model="currentProjectName" class="project-select">
            <option value="">（未选择）</option>
            <option v-for="n in projectList" :key="n" :value="n">{{ n }}</option>
          </select>
          <button type="button" class="btn-secondary" @click="handleLoadProject" :disabled="!currentProjectName">加载</button>
        </div>
        <div class="project-row">
          <span class="project-label">新建项目：</span>
          <input v-model="newProjectName" type="text" class="project-input" placeholder="输入项目名称（如 2025-XX站）" />
          <button type="button" class="btn-primary" @click="handleCreateProject" :disabled="!newProjectName.trim()">创建项目</button>
        </div>
        <p v-if="projectMessage" class="project-message" :class="projectMessageType">{{ projectMessage }}</p>
      </div>

      <!-- 检测模板模块切换 -->
      <div class="module-tabs">
        <div
          class="tab-item"
          v-for="tab in moduleTabs"
          :key="tab.key"
          :class="{ active: activeModule === tab.key }"
          @click="activeModule = tab.key"
        >
          {{ tab.name }}
        </div>
      </div>

      <!-- 基础信息模块 -->
      <BaseInfo v-if="activeModule === 'baseInfo'" :templateData="templateData" />

      <!-- 资产情况模块 -->
      <AssetInfo v-if="activeModule === 'asset'" :templateData="templateData" />

      <!-- 检测要求模块 -->
      <div v-if="activeModule === 'check'">
        <!-- 系统类型选择 -->
        <div class="system-row">
          <span class="system-label">系统类型：</span>
          <select v-model="currentSystem" class="system-select">
            <option value="收费">收费系统</option>
            <option value="ETC">ETC系统</option>
            <option value="路段中心">路段中心系统</option>
          </select>
          <span class="system-tip">切换后将加载对应模板的测评项</span>
        </div>

        <p v-if="checklistLoading" class="checklist-loading">加载测评项中...</p>
        <p v-else-if="checklistError" class="checklist-error">{{ checklistError }}</p>
        <CheckRequirement v-else :checklist="checklist" :templateData="templateData" />
      </div>

      <!-- 检测结果模块 -->
      <CheckResult v-if="activeModule === 'result'" :templateData="templateData" :checklist="checklist" />

      <!-- 核查结果模块 -->
      <div v-if="activeModule === 'review'">
        <QuickReview :checklist="checklist" :templateData="templateData" />
      </div>

      <!-- 保存到项目 TOML / 提交 -->
      <div class="submit-btn-wrapper">
        <button
          class="submit-btn"
          @click="handleSaveToProject"
          :disabled="isSubmitting || !currentProjectName"
        >
          <span v-if="!isSubmitting">保存到项目 TOML</span>
          <span v-if="isSubmitting">保存中...</span>
        </button>
        <button
          class="submit-btn outline"
          @click="handleSubmit"
          :disabled="isSubmitting"
        >
          <span v-if="!isSubmitting">另存为 JSON</span>
          <span v-if="isSubmitting">提交中...</span>
        </button>
        <button
          class="submit-btn outline"
          @click="handleGenerateReport"
          :disabled="isSubmitting"
        >
          <span v-if="!isSubmitting">生成报告（HTML）</span>
          <span v-if="isSubmitting">生成中...</span>
        </button>
      </div>

      <!-- 提交结果提示 -->
      <div v-if="submitResult" class="submit-result" :class="submitResult.type">
        <span class="result-icon">{{ submitResult.type === 'success' ? '✅' : '❌' }}</span>
        <span class="result-text">{{ submitResult.message }}</span>
      </div>

      <p class="card-desc mt-20">
        支持ETC并网检测全流程管理，可直接编辑模板数据，提交后生成标准化等保检测报告
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { openPath } from '@tauri-apps/plugin-opener';
import BaseInfo from './modules/BaseInfo.vue';
import AssetInfo from './modules/AssetInfo.vue';
import CheckRequirement from './modules/CheckRequirement.vue';
import CheckResult from './modules/CheckResult.vue';
import QuickReview from './modules/QuickReview.vue';
import type { TabItem, TemplateData, CheckList, SubmitResult } from '@/types';

// 模块标签
const moduleTabs = ref<TabItem[]>([
  { key: 'baseInfo', name: '基础信息' },
  { key: 'asset', name: '资产情况' },
  { key: 'check', name: '检测要求' },
  { key: 'result', name: '检测结果' },
  { key: 'review', name: '核查结果' }
]);

const activeModule = ref('baseInfo');
const currentSystem = ref<'收费' | 'ETC' | '路段中心'>('收费');
const isSubmitting = ref(false);
const submitResult = ref<SubmitResult | null>(null);

// 模板数据初始化
const templateData = ref<TemplateData>({
  基础信息: {
    编号: '',
    报告时间: '',
    检测时间: ''
  },
  被测单位信息: {
    单位名称: '',
    单位地址: ''
  },
  系统信息: {
    系统名称: ''
  },
  资产情况: {
    机房: '',
    网络设备: '',
    安全设备: '',
    服务器: '',
    终端: '',
    业务应用系统: ''
  },
  收费通用要求: {
    物理环境: '',
    通信网络: '',
    区域边界: ''
  },
  ETC通用要求: {
    物理环境: '',
    通信网络: ''
  },
  漏洞扫描结果: {
    结果: ''
  },
  抽检系统范围: {
    对象: ''
  },
  报告结论: {
    结论: ''
  },
  主要问题: {
    问题: ''
  }
});

// 测评项数据：从后端 收费.toml 加载，初始为空
const checklist = ref<CheckList>({ title: '', version: '', items: [] });
const checklistLoading = ref(false);
const checklistError = ref('');

// 项目 TOML：当前项目名、项目列表、新建项目名、提示信息
const currentProjectName = ref('');
const projectList = ref<string[]>([]);
const newProjectName = ref('');
const projectMessage = ref('');
const projectMessageType = ref<'success' | 'error'>('success');

// 从后端加载测评项（读取 resources/收费.toml）
async function fetchChecklist() {
  checklistLoading.value = true;
  checklistError.value = '';
  try {
    const data = await invoke<CheckList>('load_checklist', { name: currentSystem.value });
    checklist.value = data;
  } catch (e: unknown) {
    const msg = e instanceof Error ? e.message : String(e);
    checklistError.value = `加载测评项失败：${msg}`;
    checklist.value = { title: '', version: '', items: [] };
  } finally {
    checklistLoading.value = false;
  }
}

// 列出已有项目
async function refreshProjectList() {
  try {
    const list = await invoke<string[]>('list_project_names');
    projectList.value = list ?? [];
  } catch {
    projectList.value = [];
  }
}

// 创建项目：生成 projects/{name}.toml，并加载到当前表单
async function handleCreateProject() {
  const name = newProjectName.value.trim();
  if (!name) return;
  projectMessage.value = '';
  try {
    await invoke('create_project', { name });
    await refreshProjectList();
    currentProjectName.value = name;
    await handleLoadProject();
    projectMessage.value = `已创建并加载项目：${name}`;
    projectMessageType.value = 'success';
    newProjectName.value = '';
  } catch (e: unknown) {
    const msg = e instanceof Error ? e.message : String(e);
    projectMessage.value = `创建失败：${msg}`;
    projectMessageType.value = 'error';
  }
  setTimeout(() => { projectMessage.value = ''; }, 3000);
}

// 加载项目：从 projects/{name}.toml 读入并回填表单
async function handleLoadProject() {
  const name = currentProjectName.value;
  if (!name) return;
  projectMessage.value = '';
  try {
    const payload = await invoke<{ template_data: TemplateData; checklist: CheckList }>('load_project', { name });
    templateData.value = payload.template_data;
    checklist.value = payload.checklist;
    projectMessage.value = `已加载项目：${name}`;
    projectMessageType.value = 'success';
  } catch (e: unknown) {
    const msg = e instanceof Error ? e.message : String(e);
    projectMessage.value = `加载失败：${msg}`;
    projectMessageType.value = 'error';
  }
  setTimeout(() => { projectMessage.value = ''; }, 3000);
}

// 保存到项目 TOML：将当前表单写入 projects/{name}.toml
async function handleSaveToProject() {
  const name = currentProjectName.value;
  if (!name) {
    submitResult.value = { type: 'error', message: '请先选择或创建项目' };
    setTimeout(() => { submitResult.value = null; }, 3000);
    return;
  }
  try {
    isSubmitting.value = true;
    submitResult.value = null;
    const payload = {
      template_data: templateData.value,
      checklist: checklist.value,
    };
    const path = await invoke<string>('save_project', { name, payload });
    submitResult.value = { type: 'success', message: `已保存到：${path}` };
  } catch (error: unknown) {
    const msg = error instanceof Error ? error.message : String(error);
    submitResult.value = { type: 'error', message: `保存失败：${msg}` };
  } finally {
    isSubmitting.value = false;
    setTimeout(() => { submitResult.value = null; }, 3000);
  }
}

// 生成报告（HTML）：将当前表单数据渲染为离线报告文件
async function handleGenerateReport() {
  try {
    isSubmitting.value = true;
    submitResult.value = null;

    const name = currentProjectName.value || templateData.value.基础信息.编号 || 'grid_check_report';
    const payload = {
      template_data: templateData.value,
      checklist: checklist.value,
    };

    const path = await invoke<string>('generate_grid_check_report', { name, payload });

    submitResult.value = { type: 'success', message: `报告已生成：${path}` };
    try {
      await openPath(path);
    } catch {
      // ignore
    }
  } catch (error: unknown) {
    const msg = error instanceof Error ? error.message : String(error);
    submitResult.value = { type: 'error', message: `生成报告失败：${msg}` };
  } finally {
    isSubmitting.value = false;
    setTimeout(() => { submitResult.value = null; }, 3000);
  }
}

onMounted(() => {
  fetchChecklist();
  refreshProjectList();
});

watch(currentSystem, () => {
  fetchChecklist();
});

// 处理提交
const handleSubmit = async () => {
  try {
    isSubmitting.value = true;
    submitResult.value = null;

    // 这里可以后续改成“报告编号”或“项目名称”等唯一标识
    const name = templateData.value.基础信息.编号 || 'grid_check';

    const payload = {
      template_data: templateData.value,
      checklist: checklist.value,
    };

    const path = await invoke<string>('submit_grid_check', {
      name,
      payload,
    });

    submitResult.value = {
      type: 'success',
      message: `检测模板提交成功，数据已保存到：${path}`,
    };
  } catch (error: any) {
    submitResult.value = {
      type: 'error',
      message: `检测模板提交失败：${error?.message ?? String(error)}`,
    };
  } finally {
    isSubmitting.value = false;

    setTimeout(() => {
      submitResult.value = null;
    }, 3000);
  }
};
</script>

<style scoped>
.grid-check {
  width: 100%;
  height: 100%;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 20px;
}

.card-header-icon {
  font-size: 20px;
}

.card-title {
  font-size: 18px;
  font-weight: 600;
  color: #1f2937;
  margin: 0;
}

.card-content {
  padding: 20px;
  background: #fff;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
}

.card-main-desc {
  margin-bottom: 20px;
  color: #4b5563;
  line-height: 1.6;
}

.module-tabs {
  display: flex;
  gap: 4px;
  margin-bottom: 20px;
  border-bottom: 1px solid #e5e7eb;
  padding-bottom: 8px;
}

.tab-item {
  padding: 8px 16px;
  cursor: pointer;
  border-radius: 4px 4px 0 0;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s;
}

.tab-item.active {
  background: #3b82f6;
  color: #fff;
}

.tab-item:hover:not(.active) {
  background: #eff6ff;
  color: #3b82f6;
}

.project-bar {
  margin-bottom: 20px;
  padding: 16px;
  background: #f9fafb;
  border-radius: 8px;
  border: 1px solid #e5e7eb;
}
.project-row {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 10px;
}
.project-row:last-of-type {
  margin-bottom: 0;
}
.project-label {
  font-size: 14px;
  color: #4b5563;
  min-width: 80px;
}
.project-select {
  padding: 6px 10px;
  border: 1px solid #d1d5db;
  border-radius: 4px;
  font-size: 14px;
  min-width: 180px;
}
.project-input {
  padding: 6px 10px;
  border: 1px solid #d1d5db;
  border-radius: 4px;
  font-size: 14px;
  flex: 1;
  max-width: 280px;
}
.btn-primary {
  padding: 6px 16px;
  background: #3b82f6;
  color: #fff;
  border: none;
  border-radius: 4px;
  font-size: 14px;
  cursor: pointer;
}
.btn-primary:disabled {
  background: #93c5fd;
  cursor: not-allowed;
}
.btn-secondary {
  padding: 6px 16px;
  background: #fff;
  color: #3b82f6;
  border: 1px solid #3b82f6;
  border-radius: 4px;
  font-size: 14px;
  cursor: pointer;
}
.btn-secondary:disabled {
  color: #93c5fd;
  border-color: #93c5fd;
  cursor: not-allowed;
}
.project-message {
  margin-top: 10px;
  font-size: 13px;
}
.project-message.success { color: #16a34a; }
.project-message.error { color: #dc2626; }

.submit-btn-wrapper {
  margin-top: 20px;
  display: flex;
  justify-content: center;
  gap: 12px;
}

.submit-btn {
  padding: 10px 32px;
  background: #3b82f6;
  color: #fff;
  border: none;
  border-radius: 4px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s;
}
.submit-btn.outline {
  background: #fff;
  color: #3b82f6;
  border: 1px solid #3b82f6;
}

.submit-btn:disabled {
  background: #93c5fd;
  cursor: not-allowed;
}

.submit-btn:hover:not(:disabled) {
  background: #2563eb;
}

.submit-result {
  margin-top: 16px;
  padding: 12px;
  border-radius: 4px;
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
}

.submit-result.success {
  background: #dcfce7;
  color: #16a34a;
}

.submit-result.error {
  background: #fee2e2;
  color: #dc2626;
}

.result-icon {
  font-size: 16px;
}

.card-desc {
  margin-top: 20px;
  color: #6b7280;
  font-size: 14px;
  line-height: 1.6;
}

.mt-20 {
  margin-top: 20px;
}

.checklist-loading {
  color: #6b7280;
  padding: 20px;
}
.checklist-error {
  color: #dc2626;
  padding: 20px;
  background: #fee2e2;
  border-radius: 4px;
}

.system-row {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 12px;
}

.system-label {
  font-size: 14px;
  color: #4b5563;
}

.system-select {
  padding: 4px 10px;
  border-radius: 4px;
  border: 1px solid #d1d5db;
  font-size: 14px;
}

.system-tip {
  font-size: 12px;
  color: #9ca3af;
}
</style>