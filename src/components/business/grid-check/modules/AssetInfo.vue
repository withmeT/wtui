<template>
  <div class="module-content asset-info">
    <h3 class="module-title">资产情况</h3>
    <div class="form-list">
      <div class="form-item">
        <label class="form-label">机房信息：</label>
        <textarea
          v-model="templateData.资产情况.机房"
          class="form-control textarea-lg"
          placeholder="格式：{机房名称:XX机房,机房位置:XX位置}"
        ></textarea>
      </div>
      <div class="form-item">
        <label class="form-label">网络设备：</label>
        <textarea
          v-model="templateData.资产情况.网络设备"
          class="form-control textarea-lg"
          placeholder="格式：{设备名称:XX,品牌:XX,型号:XX,用途:XX,数量:XX}"
        ></textarea>
      </div>
      <div class="form-item">
        <label class="form-label">安全设备：</label>
        <textarea
          v-model="templateData.资产情况.安全设备"
          class="form-control textarea-lg"
          placeholder="格式：{设备名称:XX,品牌:XX,型号:XX,用途:XX,数量:XX}"
        ></textarea>
      </div>
      <div class="form-item">
        <label class="form-label">服务器：</label>
        <textarea
          v-model="templateData.资产情况.服务器"
          class="form-control textarea-lg"
          placeholder="格式：{设备名称:XX,操作系统及版本:XX,数据库版本:XX,数量:XX}"
        ></textarea>
      </div>
      <div class="form-item">
        <label class="form-label">终端设备：</label>
        <textarea
          v-model="templateData.资产情况.终端"
          class="form-control textarea-lg"
          placeholder="格式：{设备名称:XX,操作系统及版本:XX,用途:XX,数量:XX}"
        ></textarea>
      </div>
      <div class="form-item">
        <label class="form-label">业务应用系统：</label>
        <textarea
          v-model="templateData.资产情况.业务应用系统"
          class="form-control textarea-lg"
          placeholder="格式：{系统名称:XX,主要功能:XX,开发厂商:XX}"
        ></textarea>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { TemplateData } from '@/types';

interface Props {
  templateData: TemplateData;
}

const props = defineProps<Props>();
</script>

<style scoped>
.module-title {
  font-size: 16px;
  font-weight: 600;
  margin-bottom: 20px;
  color: #1f2937;
}

.form-list {
  display: flex;
  flex-direction: column;
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
  border: 1px solid #d1d5db;
  border-radius: 4px;
  outline: none;
  transition: border-color 0.2s;
  padding: 8px 12px;
}

.form-control:focus {
  border-color: #3b82f6;
}

.textarea-lg {
  min-height: 100px;
  resize: vertical;
}
</style>