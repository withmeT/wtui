<template>
  <div class="quick-review">
    <h3 class="module-title">快速核查结果（收费系统 1~45 项）</h3>

    <div class="review-table-wrapper">
      <table class="review-table">
        <thead>
          <tr>
            <th style="width: 40px">ID</th>
            <th>测评项</th>
            <th style="width: 80px">一票否决</th>
            <th>检测要求</th>
            <th>判定条件</th>
            <th>补偿措施</th>
            <th style="width: 120px">符合情况</th>
            <th style="width: 220px">不符合涉及设备</th>
            <th style="width: 220px">备注</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="item in reviewItems" :key="item.id">
            <td>{{ item.id }}</td>
            <td>
              <div class="item-title">
                <span class="main">{{ item.requirement_item }}</span>
                <span v-if="item.requirement_subitem" class="sub">
                  - {{ item.requirement_subitem }}
                </span>
              </div>
            </td>
            <td>
              <span v-if="item.is_important" class="tag tag-danger">是</span>
              <span v-else class="tag">否</span>
            </td>
            <td><div class="text-cell">{{ item.requirement_detail }}</div></td>
            <td><div class="text-cell">{{ item.judge_condition }}</div></td>
            <td><div class="text-cell">{{ item.compensation_measure }}</div></td>
            <td>
              <select
                v-model="getRow(item.id).status"
                class="review-select"
              >
                <option value="">请选择</option>
                <option value="符合">符合</option>
                <option value="部分符合">部分符合</option>
                <option value="不符合">不符合</option>
                <option value="不适用">不适用</option>
              </select>
            </td>
            <td>
              <div
                class="devices-cell"
                v-if="availableDevices(item).length"
              >
                <label
                  v-for="dev in availableDevices(item)"
                  :key="dev"
                  class="device-checkbox"
                >
                  <input
                    type="checkbox"
                    :value="dev"
                    v-model="getRow(item.id).devices"
                  />
                  <span>{{ dev }}</span>
                </label>
              </div>
              <span v-else class="sub-nav-empty">当前类别暂无可选设备</span>
            </td>
            <td>
              <input
                v-model="getRow(item.id).remark"
                class="review-input"
                placeholder="可填写说明或处置建议"
              />
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import type { CheckList, TemplateData, CheckItem } from '@/types';

interface Props {
  checklist: CheckList;
  templateData: TemplateData;
}

const props = defineProps<Props>();

// 只取收费.toml 的前 45 项
const reviewItems = computed<CheckItem[]>(() =>
  (props.checklist.items || []).filter((i) => i.id >= 1 && i.id <= 45),
);

// 从资产情况中抽取设备名称，用于勾选“不符合涉及设备”
type DeviceLike = { 设备名称?: string };

function safeParseArray<T = any>(str: unknown): T[] {
  if (typeof str !== 'string' || !str.trim()) return [];
  try {
    const parsed = JSON.parse(str);
    return Array.isArray(parsed) ? (parsed as T[]) : [];
  } catch {
    return [];
  }
}

const roomLabel = computed<string | null>(() => {
  // 机房信息：{ "机房名称": "XX机房", ... }
  try {
    const raw = props.templateData.资产情况.机房;
    if (!raw) return null;
    const obj = JSON.parse(raw);
    if (obj && typeof obj === 'object' && obj.机房名称) {
      return String(obj.机房名称);
    }
  } catch {
    // ignore
  }
  return '机房';
});

const networkDevices = computed<string[]>(() => {
  const arr = safeParseArray<DeviceLike>(props.templateData.资产情况.网络设备);
  return Array.from(
    new Set(arr.map((d) => d.设备名称).filter((n): n is string => !!n)),
  );
});

const securityDevices = computed<string[]>(() => {
  const arr = safeParseArray<DeviceLike>(props.templateData.资产情况.安全设备);
  return Array.from(
    new Set(arr.map((d) => d.设备名称).filter((n): n is string => !!n)),
  );
});

const serverDevices = computed<string[]>(() => {
  const arr = safeParseArray<DeviceLike>(props.templateData.资产情况.服务器);
  return Array.from(
    new Set(arr.map((d) => d.设备名称).filter((n): n is string => !!n)),
  );
});

const terminalDevices = computed<string[]>(() => {
  const arr = safeParseArray<DeviceLike>(props.templateData.资产情况.终端);
  return Array.from(
    new Set(arr.map((d) => d.设备名称).filter((n): n is string => !!n)),
  );
});

function classify(item: CheckItem): 'physical' | 'network' | 'boundary' | 'safe' | 'iot' | 'other' {
  const name = item.requirement_item || '';
  if (name.includes('物联网扩展')) return 'iot';
  if (name.includes('安全物理环境')) return 'physical';
  if (name.includes('安全通信网络')) return 'network';
  if (name.includes('安全区域边界')) return 'boundary';
  if (name.includes('安全计算环境')) return 'safe';
  return 'other';
}

function availableDevices(item: CheckItem): string[] {
  const cat = classify(item);
  switch (cat) {
    case 'physical':
      return roomLabel.value ? [roomLabel.value] : [];
    case 'network':
      return networkDevices.value;
    case 'boundary':
      return securityDevices.value;
    case 'safe':
      // 计算环境：除机房外全部关联
      return Array.from(
        new Set([
          ...networkDevices.value,
          ...securityDevices.value,
          ...serverDevices.value,
          ...terminalDevices.value,
        ]),
      );
    case 'iot':
      // 物联网扩展：固定设备名 RSU
      return ['RSU'];
    default:
      return [];
  }
}

// 每条的核查结果结构
type ReviewRow = {
  status: '' | '符合' | '部分符合' | '不符合' | '不适用';
  devices: string[];
  remark: string;
};

const rows = ref<Record<number, ReviewRow>>({});

function getRow(id: number): ReviewRow {
  if (!rows.value[id]) {
    rows.value[id] = {
      status: '',
      devices: [],
      remark: '',
    };
  }
  return rows.value[id];
}
</script>

<style scoped>
.quick-review {
  width: 100%;
}

.module-title {
  font-size: 16px;
  font-weight: 600;
  margin-bottom: 12px;
  color: #1f2937;
}

.review-table-wrapper {
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  overflow: hidden;
  background: #fff;
}

.review-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}

.review-table th,
.review-table td {
  border-bottom: 1px solid #e5e7eb;
  padding: 6px 8px;
  text-align: left;
  vertical-align: top;
}

.review-table thead th {
  background: #f9fafb;
  font-weight: 500;
  color: #4b5563;
}

.item-title {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.item-title .main {
  font-weight: 500;
  color: #111827;
}

.item-title .sub {
  font-size: 12px;
  color: #6b7280;
}

.text-cell {
  max-height: 72px;
  overflow: auto;
  white-space: pre-wrap;
}

.tag {
  display: inline-block;
  padding: 2px 8px;
  border-radius: 999px;
  font-size: 12px;
  background: #e5e7eb;
  color: #374151;
}

.tag-danger {
  background: #fee2e2;
  color: #b91c1c;
}

.review-select {
  width: 100%;
  padding: 4px 6px;
  border-radius: 4px;
  border: 1px solid #d1d5db;
  font-size: 13px;
}

.review-input {
  width: 100%;
  padding: 4px 6px;
  border-radius: 4px;
  border: 1px solid #d1d5db;
  font-size: 13px;
}

.devices-cell {
  display: flex;
  flex-wrap: wrap;
  gap: 4px 8px;
}

.device-checkbox {
  display: inline-flex;
  align-items: center;
  gap: 2px;
  font-size: 12px;
  color: #4b5563;
}

.sub-nav-empty {
  font-size: 12px;
  color: #9ca3af;
}
</style>
