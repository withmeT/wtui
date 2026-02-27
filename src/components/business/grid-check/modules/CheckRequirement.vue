<template>
  <div class="module-content check-requirement">
    <h3 class="module-title">{{ checklist.title }}</h3>
    <!-- 测评项列表 -->
    <div class="checklist-container">
      <div
        class="check-item"
        v-for="item in checklist.items"
        :key="item.id"
        :class="{ 'important-item': item.is_important }"
      >
        <div class="check-item-header">
          <span class="item-id">【{{ item.id }}】</span>
          <span class="item-requirement-item">{{ item.requirement_item }}</span>
          <span v-if="item.requirement_subitem" class="item-requirement-subitem">- {{ item.requirement_subitem }}</span>
          <span class="important-tag" v-if="item.is_important">★ 一票否决项</span>
        </div>

        <div class="check-item-content">
          <div class="form-item">
            <label class="form-label">检测要求详情：</label>
            <div class="text-block">
              {{ item.requirement_detail }}
            </div>
          </div>

          <div class="form-item">
            <label class="form-label">判定条件：</label>
            <div class="text-block">
              {{ item.judge_condition }}
            </div>
          </div>

          <div class="form-item">
            <label class="form-label">补偿措施：</label>
            <div class="text-block">
              {{ item.compensation_measure }}
            </div>
          </div>

          <div class="form-item">
            <label class="form-label">检测结果：</label>
            <textarea
              v-model="item.detection_result"
              class="form-control textarea-md"
              placeholder="请输入该测评项的检测结果"
            ></textarea>
          </div>

          <div class="form-item">
            <label class="form-label">结论：</label>
            <select
              v-model="item.conclusion"
              class="form-control"
            >
              <option value="">请选择结论</option>
              <option value="视同符合">视同符合</option>
              <option value="符合">符合</option>
              <option value="部分符合">部分符合</option>
              <option value="不符合">不符合</option>
              <option value="不适用">不适用</option>
            </select>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { CheckList } from '@/types';

interface Props {
  checklist: CheckList;
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

.checklist-container {
  display: flex;
  flex-direction: column;
  gap: 20px;
  max-height: 600px;
  overflow-y: auto;
}

.check-item {
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  padding: 16px;
  background: #fff;
}

.check-item.important-item {
  border-left: 4px solid #f59e0b;
}

.check-item-header {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-bottom: 12px;
  align-items: center;
}

.item-id {
  font-weight: 600;
  color: #3b82f6;
}

.item-requirement-item {
  font-weight: 500;
}

.item-requirement-subitem {
  color: #6b7280;
}

.important-tag {
  margin-left: auto;
  font-size: 12px;
  color: #f59e0b;
  font-weight: 600;
}

.check-item-content {
  display: flex;
  flex-direction: column;
  gap: 12px;
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

.textarea-md {
  min-height: 80px;
  resize: vertical;
}

.text-block {
  padding: 8px 12px;
  border-radius: 4px;
  background-color: #f9fafb;
  border: 1px solid #e5e7eb;
  white-space: pre-wrap;
  font-size: 13px;
  line-height: 1.6;
  color: #374151;
}
</style>