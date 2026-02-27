<template>
  <div class="module-content check-requirement">
    <h3 class="module-title">{{ checklist.title }}</h3>
    <!-- 测评项列表 -->
    <div class="checklist-container">
      <div
        class="check-item"
        v-for="item in checklist.items"
        :key="item.id"
        :class="{ 'important-item': item.is_important, 'collapsed-item': !isExpanded(item.id) }"
      >
        <div class="check-item-header" @click="toggleItem(item.id)">
          <span class="item-id">【{{ item.id }}】</span>
          <span class="item-requirement-item">{{ item.requirement_item }}</span>
          <span v-if="item.requirement_subitem" class="item-requirement-subitem">- {{ item.requirement_subitem }}</span>
          <span class="important-tag" v-if="item.is_important">★ 一票否决项</span>
          <button
            type="button"
            class="toggle-btn"
            @click.stop="toggleItem(item.id)"
          >
            {{ isExpanded(item.id) ? '收起' : '展开' }}
          </button>
        </div>

        <div v-if="isExpanded(item.id)" class="check-item-content">
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
            <label class="form-label">检测结果（汇总文本）：</label>
            <textarea
              v-model="item.detection_result"
              class="form-control textarea-md"
              placeholder="请输入该测评项的检测结果（可自动从安全计算环境生成）"
            ></textarea>
          </div>

          <!-- 安全计算环境：按设备建立监测项（仅“安全计算环境”相关条款展示） -->
          <div v-if="isSafetyEnvironmentItem(item)" class="safety-block">
            <div class="safety-header">
              <span class="form-label">安全计算环境（按设备）</span>
              <button
                type="button"
                class="btn-mini"
                @click="addSafetyRow(item.id)"
              >
                新增监测项
              </button>
            </div>
            <div class="table-wrapper">
              <table class="safety-table">
                <thead>
                  <tr>
                    <th style="width: 26px">#</th>
                    <th>设备名称</th>
                    <th>监测项</th>
                    <th>结果</th>
                    <th>符合情况</th>
                    <th style="width: 70px">操作</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-if="getSafetyRows(item.id).length === 0">
                    <td colspan="6" class="empty-cell">暂无监测项，点击“新增监测项”添加</td>
                  </tr>
                  <tr
                    v-for="(row, idx) in getSafetyRows(item.id)"
                    :key="row._id"
                  >
                    <td>{{ idx + 1 }}</td>
                    <td>
                      <input
                        v-model="row.设备名称"
                        class="table-input"
                        placeholder="如 站级交换机"
                        @input="onSafetyChange(item.id)"
                      />
                    </td>
                    <td>
                      <input
                        v-model="row.监测项"
                        class="table-input"
                        placeholder="如 口令复杂度、访问控制等"
                        @input="onSafetyChange(item.id)"
                      />
                    </td>
                    <td>
                      <input
                        v-model="row.结果"
                        class="table-input"
                        placeholder="如 符合/不符合/部分符合"
                        @input="onSafetyChange(item.id)"
                      />
                    </td>
                    <td>
                      <input
                        v-model="row.符合情况"
                        class="table-input"
                        placeholder="简要说明符合情况"
                        @input="onSafetyChange(item.id)"
                      />
                    </td>
                    <td>
                      <button
                        type="button"
                        class="btn-link"
                        @click="removeSafetyRow(item.id, idx)"
                      >
                        删除
                      </button>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
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
import { ref } from 'vue';
import type { CheckList } from '@/types';

interface Props {
  checklist: CheckList;
}

const props = defineProps<Props>();

const expandedIds = ref<Set<number>>(new Set());

function isExpanded(id: number) {
  return expandedIds.value.has(id);
}

function toggleItem(id: number) {
  const next = new Set(expandedIds.value);
  if (next.has(id)) {
    next.delete(id);
  } else {
    next.add(id);
  }
  expandedIds.value = next;
}

type SafetyRow = {
  _id: number;
  设备名称: string;
  监测项: string;
  结果: string;
  符合情况: string;
};

const safetyMap = ref<Record<number, SafetyRow[]>>({});
let safetyUid = 1;
const nextSafetyId = () => safetyUid++;

// 仅对“安全计算环境”相关条款开启安全计算环境表格，
// 通过条款名称里是否包含“安全计算环境”或“计算环境”来判断。
function isSafetyEnvironmentItem(item: CheckList['items'][number]): boolean {
  const name = item.requirement_item || '';
  return name.includes('安全计算环境') || name.includes('计算环境');
}

function getSafetyRows(itemId: number): SafetyRow[] {
  if (!safetyMap.value[itemId]) {
    safetyMap.value[itemId] = [];
  }
  return safetyMap.value[itemId];
}

function addSafetyRow(itemId: number) {
  const rows = [...getSafetyRows(itemId), {
    _id: nextSafetyId(),
    设备名称: '',
    监测项: '',
    结果: '',
    符合情况: '',
  }];
  safetyMap.value = {
    ...safetyMap.value,
    [itemId]: rows,
  };
  syncSafetyToDetectionResult(itemId);
}

function removeSafetyRow(itemId: number, index: number) {
  const rows = [...getSafetyRows(itemId)];
  rows.splice(index, 1);
  safetyMap.value = {
    ...safetyMap.value,
    [itemId]: rows,
  };
  syncSafetyToDetectionResult(itemId);
}

function onSafetyChange(itemId: number) {
  syncSafetyToDetectionResult(itemId);
}

function syncSafetyToDetectionResult(itemId: number) {
  const item = props.checklist.items.find(i => i.id === itemId);
  if (!item) return;
  const rows = getSafetyRows(itemId);
  if (!rows.length) {
    return;
  }
  const lines = rows.map(r => {
    return `设备：${r.设备名称 || '-'}｜监测项：${r.监测项 || '-'}｜结果：${r.结果 || '-'}｜符合情况：${r.符合情况 || '-'}`;
  });
  item.detection_result = lines.join('\n');
}
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

.collapsed-item {
  padding-bottom: 8px;
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

.toggle-btn {
  margin-left: auto;
  border: none;
  background: none;
  font-size: 12px;
  color: #2563eb;
  cursor: pointer;
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

.safety-block {
  margin-top: 8px;
  padding: 10px;
  border-radius: 6px;
  border: 1px dashed #d1d5db;
  background: #f9fafb;
}

.safety-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 8px;
}

.btn-mini {
  border: 1px solid #3b82f6;
  background: #eff6ff;
  color: #1d4ed8;
  border-radius: 9999px;
  font-size: 12px;
  padding: 2px 8px;
  cursor: pointer;
  white-space: nowrap;
}

.btn-mini:hover {
  background: #dbeafe;
}

.table-wrapper {
  overflow-x: auto;
}

.safety-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 12px;
  background: #fff;
}

.safety-table th,
.safety-table td {
  border: 1px solid #e5e7eb;
  padding: 4px 6px;
  text-align: left;
}

.safety-table th {
  background: #f3f4f6;
  font-weight: 500;
  color: #4b5563;
}

.table-input {
  width: 100%;
  border: none;
  outline: none;
  font-size: 12px;
  padding: 1px 2px;
  background: transparent;
}

.table-input:focus {
  outline: 1px solid #3b82f6;
  border-radius: 2px;
  background: #eff6ff;
}

.btn-link {
  border: none;
  background: none;
  color: #ef4444;
  font-size: 12px;
  cursor: pointer;
  padding: 0;
}

.btn-link:hover {
  text-decoration: underline;
}

.empty-cell {
  text-align: center;
  color: #9ca3af;
  padding: 6px;
  font-size: 12px;
}
</style>