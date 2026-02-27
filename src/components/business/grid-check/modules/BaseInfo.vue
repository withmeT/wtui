<template>
  <div class="module-content base-info">
    <h3 class="module-title">基础信息</h3>
    <div class="form-grid">
      <div class="form-item">
        <label class="form-label">报告编号：</label>
        <input
          v-model="templateData.基础信息.编号"
          type="text"
          class="form-control"
          placeholder="请输入报告编号"
        />
      </div>
      <div class="form-item">
        <label class="form-label">报告时间：</label>
        <input
          v-model="templateData.基础信息.报告时间"
          type="date"
          class="form-control"
        />
      </div>
      <div class="form-item">
        <label class="form-label">检测时间：</label>
        <input
          v-model="templateData.基础信息.检测时间"
          type="date"
          class="form-control"
        />
      </div>
      <div class="form-item">
        <label class="form-label">被测单位：</label>
        <input
          v-model="templateData.被测单位信息.单位名称"
          type="text"
          class="form-control"
          placeholder="请输入被测单位全称"
        />
      </div>
      <div class="form-item">
        <label class="form-label">系统名称：</label>
        <input
          v-model="templateData.系统信息.系统名称"
          type="text"
          class="form-control"
          placeholder="请输入系统名称"
        />
      </div>
      <div class="form-item">
        <label class="form-label">单位地址：</label>
        <input
          v-model="templateData.被测单位信息.单位地址"
          type="text"
          class="form-control"
          placeholder="请输入单位地址"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { TemplateData } from '@/types';

interface Props {
  templateData: TemplateData;
}

defineProps<Props>();
</script>

<style scoped>
.module-title {
  font-size: 16px;
  font-weight: 600;
  margin-bottom: 20px;
  color: #1f2937;
}

.form-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 16px;
}

.form-item {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.form-label {
  font-size: 14px;
  color: #4b5563;
  font-weight: 500;
}

.form-control {
  height: 36px;
  padding: 0 12px;
  border: 1px solid #d1d5db;
  border-radius: 4px;
  outline: none;
  transition: border-color 0.2s;
}

.form-control:focus {
  border-color: #3b82f6;
}
</style>