<template>
  <div class="module-content check-result">
    <h3 class="module-title">检测结果</h3>
    <div class="form-list">
      <div class="form-item">
        <label class="form-label">漏洞扫描结果：</label>
        <textarea
          v-model="templateData.漏洞扫描结果.结果"
          class="form-control textarea-lg"
          placeholder="请输入漏洞扫描详细结果"
        ></textarea>
      </div>
      <div class="form-item">
        <label class="form-label">抽检系统范围：</label>
        <textarea
          v-model="templateData.抽检系统范围.对象"
          class="form-control textarea-md"
          placeholder="格式：{名称编号:XXXXXX,是否抽检对象:是}"
        ></textarea>
      </div>
      <div class="form-item">
        <label class="form-label">报告结论：</label>
        <textarea
          v-model="templateData.报告结论.结论"
          class="form-control textarea-md"
          placeholder="请输入本次检测的最终结论"
        ></textarea>
      </div>
      <div class="form-item">
        <label class="form-label">主要问题：</label>
        <textarea
          v-model="templateData.主要问题.问题"
          class="form-control textarea-lg"
          placeholder="请列出本次检测发现的主要问题"
        ></textarea>
      </div>

      <!-- 新增测评项统计 -->
      <div class="check-result-stat">
        <h4 class="stat-title">测评项统计</h4>
        <div class="stat-grid">
          <div class="stat-item">
            <span class="stat-label">总测评项数：</span>
            <span class="stat-value">{{ checklist.items.length }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">重要项数：</span>
            <span class="stat-value">{{ importantItemCount }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">符合项数：</span>
            <span class="stat-value">{{ conformItemCount }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">部分符合项数：</span>
            <span class="stat-value">{{ partialConformItemCount }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">不符合项数：</span>
            <span class="stat-value">{{ nonConformItemCount }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">视同符合项数：</span>
            <span class="stat-value">{{ deemedConformItemCount }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">不适用项数：</span>
            <span class="stat-value">{{ notApplicableItemCount }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">未填写结论项数：</span>
            <span class="stat-value">{{ unFilledItemCount }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { TemplateData, CheckList } from '@/types';

interface Props {
  templateData: TemplateData;
  checklist: CheckList;
}

const props = defineProps<Props>();

// 计算统计数据
const importantItemCount = computed(() => {
  return props.checklist.items.filter(i => i.is_important).length;
});

const conformItemCount = computed(() => {
  return props.checklist.items.filter(i => i.conclusion === '符合').length;
});

const partialConformItemCount = computed(() => {
  return props.checklist.items.filter(i => i.conclusion === '部分符合').length;
});

const nonConformItemCount = computed(() => {
  return props.checklist.items.filter(i => i.conclusion === '不符合').length;
});

const deemedConformItemCount = computed(() => {
  return props.checklist.items.filter(i => i.conclusion === '视同符合').length;
});

const notApplicableItemCount = computed(() => {
  return props.checklist.items.filter(i => i.conclusion === '不适用').length;
});

const unFilledItemCount = computed(() => {
  return props.checklist.items.filter(i => !i.conclusion).length;
});
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

.textarea-md {
  min-height: 80px;
  resize: vertical;
}

.check-result-stat {
  margin-top: 20px;
  padding: 16px;
  background: #f9fafb;
  border-radius: 8px;
}

.stat-title {
  font-size: 14px;
  font-weight: 600;
  margin-bottom: 12px;
  color: #1f2937;
}

.stat-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 12px;
}

.stat-item {
  display: flex;
  justify-content: space-between;
  padding: 8px;
  background: #fff;
  border-radius: 4px;
  border: 1px solid #e5e7eb;
}

.stat-label {
  font-size: 13px;
  color: #4b5563;
}

.stat-value {
  font-size: 13px;
  font-weight: 600;
  color: #3b82f6;
}
</style>