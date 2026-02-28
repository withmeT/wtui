<template>
  <div class="module-content check-requirement">
    <h3 class="module-title">{{ checklist.title }}</h3>

    <div class="layout">
      <!-- 左侧导航栏 -->
      <aside class="nav">
        <div
          v-for="cat in categories"
          :key="cat.key"
          class="nav-item"
          :class="{ active: activeCategory === cat.key }"
          @click="activeCategory = cat.key"
        >
          {{ cat.label }}
        </div>

        <!-- 安全计算环境的二级导航：按资产里的设备 -->
        <div v-if="activeCategory === 'safe-env'" class="sub-nav">
          <div class="sub-nav-title">安全计算环境设备列表</div>
          <div
            v-for="dev in deviceNavList"
            :key="dev"
            class="sub-nav-item"
            :class="{ active: activeDevice === dev }"
            @click="activeDevice = dev"
          >
            {{ dev }}
          </div>
          <p v-if="!deviceNavList.length" class="sub-nav-empty">
            请先在“资产情况”中维护网络设备 / 安全设备 / 服务器 / 终端，再回来配置安全计算环境。
          </p>
        </div>
      </aside>

      <!-- 右侧内容 -->
      <section class="content">
        <!-- 普通大类（物理环境 / 通信网络 / 区域边界 / 物联网扩展等） -->
        <template v-if="activeCategory !== 'safe-env'">
          <div
            v-for="item in visibleItems"
            :key="item.id"
            class="check-item"
            :class="{ 'important-item': item.is_important }"
          >
            <div class="check-item-header" @click="toggleItem(item.id)">
              <span class="item-id">【{{ item.id }}】</span>
              <span class="item-requirement-item">{{ displayRequirement(item.requirement_item) }}</span>
              <span v-if="item.requirement_subitem" class="item-requirement-subitem">
                - {{ item.requirement_subitem }}
              </span>
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

          <p v-if="!visibleItems.length" class="empty-tip">
            当前大类下暂无测评项。
          </p>
        </template>

        <!-- 安全计算环境：左侧选设备，右侧该设备的全部安全计算环境检测项 -->
        <template v-else>
          <div v-if="!deviceNavList.length" class="empty-tip">
            请先在“资产情况”模块维护设备列表。
          </div>
          <div v-else-if="!activeDevice" class="empty-tip">
            请先在左侧选择一个设备，再配置其安全计算环境检测项。
          </div>
          <div v-else>
            <h4 class="device-title">设备：{{ activeDevice }}</h4>
            <div
              v-for="item in safeEnvItems"
              :key="item.id"
              class="check-item"
              :class="{ 'important-item': item.is_important }"
            >
              <div class="check-item-header" @click="toggleItem(item.id)">
                <span class="item-id">【{{ item.id }}】</span>
                <span class="item-requirement-item">{{ displayRequirement(item.requirement_item) }}</span>
                <span v-if="item.requirement_subitem" class="item-requirement-subitem">
                  - {{ item.requirement_subitem }}
                </span>
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
                  <label class="form-label">检测结果（针对该设备）：</label>
                  <textarea
                    v-model="getDeviceEntry(activeDevice, item.id).结果"
                    class="form-control textarea-md"
                    placeholder="请输入该设备在本测评项下的检测结果"
                    @input="syncDeviceSafety(item.id)"
                  ></textarea>
                </div>

                <div class="form-item">
                  <label class="form-label">符合情况：</label>
                  <input
                    v-model="getDeviceEntry(activeDevice, item.id).符合情况"
                    class="form-control"
                    placeholder="简要说明符合情况"
                    @input="syncDeviceSafety(item.id)"
                  />
                </div>
              </div>
            </div>
          </div>
        </template>
      </section>
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

type Category = {
  key: string;
  label: string;
};

const categories: Category[] = [
  { key: 'physical', label: '安全物理环境' },
  { key: 'network', label: '安全通信网络' },
  { key: 'boundary', label: '安全区域边界' },
  { key: 'safe-env', label: '安全计算环境' },
  { key: 'iot', label: '物联网扩展要求' },
];

const activeCategory = ref<string>('physical');
const activeDevice = ref<string>('');

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

// 各类条目过滤
function inCategory(item: CheckItem, key: string): boolean {
  const name = item.requirement_item || '';
  switch (key) {
    case 'physical':
      // 物联网扩展的条目不要出现在“安全物理环境”导航下
      return name.includes('安全物理环境') && !name.includes('物联网扩展');
    case 'network':
      // 预留：如果后续有 “安全通信网络-物联网扩展”，同样只归到物联网扩展
      return name.includes('安全通信网络') && !name.includes('物联网扩展');
    case 'boundary':
      return name.includes('安全区域边界') && !name.includes('物联网扩展');
    case 'safe-env':
      // “安全计算环境-物联网扩展” 这类要归到物联网扩展，不算在安全计算环境里
      if (name.includes('物联网扩展')) return false;
      return name.includes('安全计算环境') || name.includes('计算环境');
    case 'iot':
      // 物联网扩展相关条款（例如：安全计算环境-物联网扩展）归到“物联网扩展要求”
      return name.includes('物联网') || name.includes('物联网扩展') || name.includes('扩展要求');
    default:
      return false;
  }
}

const allItems = computed(() => props.checklist.items || []);

const safeEnvItems = computed(() => allItems.value.filter(i => inCategory(i, 'safe-env')));

const visibleItems = computed(() =>
  allItems.value.filter(i => inCategory(i, activeCategory.value)),
);

// 展示用标题：物联网扩展类条目，去掉后缀“-物联网扩展”
function displayRequirement(raw: string): string {
  if (!raw) return '';
  return raw.replace(/-物联网扩展$/u, '');
}

// 从资产情况中抽取设备名称，用于安全计算环境导航
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

const deviceNavList = computed<string[]>(() => {
  const asset = props.templateData.资产情况;
  const names: string[] = [];

  const pushFrom = (raw: string) => {
    const arr = safeParseArray<DeviceLike>(raw);
    for (const d of arr) {
      if (d && d.设备名称) names.push(d.设备名称);
    }
  };

  pushFrom(asset.网络设备);
  pushFrom(asset.安全设备);
  pushFrom(asset.服务器);
  pushFrom(asset.终端);

  const list = Array.from(new Set(names)).filter(Boolean);
  if (!activeDevice.value && list.length) {
    activeDevice.value = list[0];
  }
  return list;
});

// 安全计算环境：按 设备 + 条目ID 记录结果
type DeviceEntry = {
  结果: string;
  符合情况: string;
};

const deviceSafety = ref<Record<string, Record<number, DeviceEntry>>>({});

function getDeviceEntry(device: string, itemId: number): DeviceEntry {
  if (!deviceSafety.value[device]) {
    deviceSafety.value[device] = {};
  }
  if (!deviceSafety.value[device][itemId]) {
    deviceSafety.value[device][itemId] = { 结果: '', 符合情况: '' };
  }
  return deviceSafety.value[device][itemId];
}

// 将所有设备在某条安全计算环境测评项下的结果，汇总回原来的 detection_result 字段
function syncDeviceSafety(itemId: number) {
  const item = allItems.value.find(i => i.id === itemId);
  if (!item) return;

  const lines: string[] = [];
  for (const dev of deviceNavList.value) {
    const entry = deviceSafety.value[dev]?.[itemId];
    if (!entry || (!entry.结果 && !entry.符合情况)) continue;
    lines.push(
      `设备：${dev}｜结果：${entry.结果 || '-'}｜符合情况：${entry.符合情况 || '-'}`,
    );
  }

  if (lines.length) {
    item.detection_result = lines.join('\n');
  }
}
</script>

<style scoped>
.module-title {
  font-size: 16px;
  font-weight: 600;
  margin-bottom: 16px;
  color: #1f2937;
}

.layout {
  display: flex;
  gap: 16px;
}

.nav {
  width: 200px;
  border-right: 1px solid #e5e7eb;
  padding-right: 12px;
}

.nav-item {
  padding: 8px 10px;
  margin-bottom: 4px;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  color: #374151;
}

.nav-item.active {
  background: #eff6ff;
  color: #1d4ed8;
  font-weight: 600;
}

.sub-nav {
  margin-top: 12px;
  padding-top: 8px;
  border-top: 1px solid #e5e7eb;
}

.sub-nav-title {
  font-size: 13px;
  color: #4b5563;
  margin-bottom: 6px;
}

.sub-nav-item {
  padding: 6px 8px;
  margin-bottom: 2px;
  border-radius: 4px;
  cursor: pointer;
  font-size: 13px;
  color: #4b5563;
}

.sub-nav-item.active {
  background: #e5f3ff;
  color: #1d4ed8;
  font-weight: 500;
}

.sub-nav-empty {
  margin-top: 6px;
  font-size: 12px;
  color: #9ca3af;
}

.content {
  flex: 1;
  max-height: 600px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 14px;
}

.check-item {
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  padding: 12px 14px;
  background: #fff;
}

.check-item.important-item {
  border-left: 4px solid #f59e0b;
}

.check-item-header {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-bottom: 10px;
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
  gap: 10px;
}

.form-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
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
  padding: 6px 10px;
  font-size: 13px;
}

.form-control:focus {
  border-color: #3b82f6;
}

.textarea-md {
  min-height: 72px;
  resize: vertical;
}

.text-block {
  padding: 8px 10px;
  border-radius: 4px;
  background-color: #f9fafb;
  border: 1px solid #e5e7eb;
  white-space: pre-wrap;
  font-size: 13px;
  line-height: 1.6;
  color: #374151;
}

.device-title {
  font-size: 14px;
  font-weight: 600;
  color: #111827;
}

.empty-tip {
  font-size: 13px;
  color: #9ca3af;
  padding: 12px 0;
}
</style>
