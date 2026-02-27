<template>
  <div class="module-content asset-info">
    <h3 class="module-title">资产情况</h3>

    <!-- 机房信息（单行对象） -->
    <div class="block">
      <div class="block-header">
        <span class="block-title">机房信息</span>
        <span class="block-tip">（如：机房名称、机房位置等，自由填写）</span>
      </div>
      <div class="room-grid">
        <div class="room-item">
          <label class="form-label">机房名称</label>
          <input v-model="roomInfo.机房名称" class="form-control" placeholder="如 XX 机房" />
        </div>
        <div class="room-item">
          <label class="form-label">机房位置</label>
          <input v-model="roomInfo.机房位置" class="form-control" placeholder="如 XX 楼 XX 机房" />
        </div>
      </div>
    </div>

    <!-- 网络设备表 -->
    <div class="block">
      <div class="block-header">
        <span class="block-title">网络设备</span>
        <button type="button" class="btn-mini" @click="addNetworkDevice">新增设备</button>
      </div>
      <div class="table-wrapper">
        <table class="asset-table">
          <thead>
            <tr>
              <th style="width: 24px">#</th>
              <th>设备名称</th>
              <th>品牌</th>
              <th>型号</th>
              <th>用途</th>
              <th style="width: 80px">数量</th>
              <th style="width: 70px">操作</th>
            </tr>
          </thead>
          <tbody>
            <tr v-if="networkDevices.length === 0">
              <td colspan="7" class="empty-cell">暂无数据，点击右上角“新增设备”添加</td>
            </tr>
            <tr v-for="(row, index) in networkDevices" :key="row._id">
              <td>{{ index + 1 }}</td>
              <td><input v-model="row.设备名称" class="table-input" /></td>
              <td><input v-model="row.品牌" class="table-input" /></td>
              <td><input v-model="row.型号" class="table-input" /></td>
              <td><input v-model="row.用途" class="table-input" /></td>
              <td>
                <input
                  v-model.number="row.数量"
                  type="number"
                  min="0"
                  class="table-input qty-input"
                />
              </td>
              <td>
                <button type="button" class="btn-link" @click="removeNetworkDevice(index)">删除</button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- 安全设备表 -->
    <div class="block">
      <div class="block-header">
        <span class="block-title">安全设备</span>
        <button type="button" class="btn-mini" @click="addSecurityDevice">新增设备</button>
      </div>
      <div class="table-wrapper">
        <table class="asset-table">
          <thead>
            <tr>
              <th style="width: 24px">#</th>
              <th>设备名称</th>
              <th>品牌</th>
              <th>型号</th>
              <th>用途</th>
              <th style="width: 80px">数量</th>
              <th style="width: 70px">操作</th>
            </tr>
          </thead>
          <tbody>
            <tr v-if="securityDevices.length === 0">
              <td colspan="7" class="empty-cell">暂无数据，点击右上角“新增设备”添加</td>
            </tr>
            <tr v-for="(row, index) in securityDevices" :key="row._id">
              <td>{{ index + 1 }}</td>
              <td><input v-model="row.设备名称" class="table-input" /></td>
              <td><input v-model="row.品牌" class="table-input" /></td>
              <td><input v-model="row.型号" class="table-input" /></td>
              <td><input v-model="row.用途" class="table-input" /></td>
              <td>
                <input
                  v-model.number="row.数量"
                  type="number"
                  min="0"
                  class="table-input qty-input"
                />
              </td>
              <td>
                <button type="button" class="btn-link" @click="removeSecurityDevice(index)">删除</button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- 服务器表 -->
    <div class="block">
      <div class="block-header">
        <span class="block-title">服务器</span>
        <button type="button" class="btn-mini" @click="addServer">新增服务器</button>
      </div>
      <div class="table-wrapper">
        <table class="asset-table">
          <thead>
            <tr>
              <th style="width: 24px">#</th>
              <th>设备名称</th>
              <th>品牌</th>
              <th>型号</th>
              <th>操作系统及版本</th>
              <th>数据库管理系统及版本</th>
              <th style="width: 80px">数量</th>
              <th style="width: 70px">操作</th>
            </tr>
          </thead>
          <tbody>
            <tr v-if="servers.length === 0">
              <td colspan="8" class="empty-cell">暂无数据，点击右上角“新增服务器”添加</td>
            </tr>
            <tr v-for="(row, index) in servers" :key="row._id">
              <td>{{ index + 1 }}</td>
              <td><input v-model="row.设备名称" class="table-input" /></td>
              <td><input v-model="row.品牌" class="table-input" /></td>
              <td><input v-model="row.型号" class="table-input" /></td>
              <td>
                <input
                  v-model="row.服务器操作系统及版本"
                  class="table-input"
                  placeholder="如 CentOS 7.6"
                />
              </td>
              <td>
                <input
                  v-model="row.数据库管理系统及版本"
                  class="table-input"
                  placeholder="如 MySQL 8.0"
                />
              </td>
              <td>
                <input
                  v-model.number="row.数量"
                  type="number"
                  min="0"
                  class="table-input qty-input"
                />
              </td>
              <td>
                <button type="button" class="btn-link" @click="removeServer(index)">删除</button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- 终端设备表 -->
    <div class="block">
      <div class="block-header">
        <span class="block-title">终端设备</span>
        <button type="button" class="btn-mini" @click="addTerminal">新增终端</button>
      </div>
      <div class="table-wrapper">
        <table class="asset-table">
          <thead>
            <tr>
              <th style="width: 24px">#</th>
              <th>设备名称</th>
              <th>品牌</th>
              <th>型号</th>
              <th>操作系统及版本</th>
              <th>设备用途</th>
              <th style="width: 80px">数量</th>
              <th style="width: 70px">操作</th>
            </tr>
          </thead>
          <tbody>
            <tr v-if="terminals.length === 0">
              <td colspan="8" class="empty-cell">暂无数据，点击右上角“新增终端”添加</td>
            </tr>
            <tr v-for="(row, index) in terminals" :key="row._id">
              <td>{{ index + 1 }}</td>
              <td><input v-model="row.设备名称" class="table-input" /></td>
              <td><input v-model="row.品牌" class="table-input" /></td>
              <td><input v-model="row.型号" class="table-input" /></td>
              <td>
                <input
                  v-model="row.操作系统及版本"
                  class="table-input"
                  placeholder="如 Windows 10"
                />
              </td>
              <td><input v-model="row.设备用途" class="table-input" /></td>
              <td>
                <input
                  v-model.number="row.数量"
                  type="number"
                  min="0"
                  class="table-input qty-input"
                />
              </td>
              <td>
                <button type="button" class="btn-link" @click="removeTerminal(index)">删除</button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- 业务应用系统（简单列表） -->
    <div class="block">
      <div class="block-header">
        <span class="block-title">业务应用系统</span>
        <button type="button" class="btn-mini" @click="addBizSystem">新增系统</button>
      </div>
      <div class="table-wrapper">
        <table class="asset-table">
          <thead>
            <tr>
              <th style="width: 24px">#</th>
              <th>系统名称</th>
              <th>主要功能</th>
              <th>开发厂商</th>
              <th style="width: 70px">操作</th>
            </tr>
          </thead>
          <tbody>
            <tr v-if="bizSystems.length === 0">
              <td colspan="5" class="empty-cell">暂无数据，点击右上角“新增系统”添加</td>
            </tr>
            <tr v-for="(row, index) in bizSystems" :key="row._id">
              <td>{{ index + 1 }}</td>
              <td><input v-model="row.系统名称" class="table-input" /></td>
              <td><input v-model="row.主要功能" class="table-input" /></td>
              <td><input v-model="row.开发厂商" class="table-input" /></td>
              <td>
                <button type="button" class="btn-link" @click="removeBizSystem(index)">删除</button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref, watch } from 'vue';
import type { TemplateData } from '@/types';

interface Props {
  templateData: TemplateData;
}

const props = defineProps<Props>();

type DeviceRow = {
  _id: number;
  设备名称: string;
  品牌?: string;
  型号?: string;
  用途?: string;
  数量?: number | null;
  [key: string]: any;
};

type BizSystemRow = {
  _id: number;
  系统名称: string;
  主要功能?: string;
  开发厂商?: string;
};

const roomInfo = ref<{ 机房名称?: string; 机房位置?: string }>({});
const networkDevices = ref<DeviceRow[]>([]);
const securityDevices = ref<DeviceRow[]>([]);
const servers = ref<DeviceRow[]>([]);
const terminals = ref<DeviceRow[]>([]);
const bizSystems = ref<BizSystemRow[]>([]);

let uid = 1;
const nextId = () => uid++;

function safeParseObject(str: unknown): Record<string, any> {
  if (typeof str !== 'string' || !str.trim()) return {};
  try {
    return JSON.parse(str);
  } catch {
    return {};
  }
}

function safeParseArray<T extends Record<string, any>>(str: unknown): T[] {
  if (typeof str !== 'string' || !str.trim()) return [];
  try {
    const parsed = JSON.parse(str);
    if (Array.isArray(parsed)) {
      return parsed as T[];
    }
    return [];
  } catch {
    return [];
  }
}

function serializeObject(obj: Record<string, any>): string {
  return JSON.stringify(obj || {});
}

function serializeArray(arr: any[]): string {
  return JSON.stringify(arr || []);
}

function loadFromTemplate() {
  const asset = props.templateData.资产情况;

  roomInfo.value = safeParseObject(asset.机房);
  networkDevices.value = safeParseArray<Omit<DeviceRow, '_id'>>(asset.网络设备).map((d) => ({
    _id: nextId(),
    设备名称: d.设备名称 ?? '',
    品牌: d.品牌 ?? '',
    型号: d.型号 ?? '',
    用途: d.用途 ?? '',
    数量: typeof d.数量 === 'number' ? d.数量 : (d.数量 ?? 1),
  }));
  securityDevices.value = safeParseArray<Omit<DeviceRow, '_id'>>(asset.安全设备).map((d) => ({
    _id: nextId(),
    设备名称: d.设备名称 ?? '',
    品牌: d.品牌 ?? '',
    型号: d.型号 ?? '',
    用途: d.用途 ?? '',
    数量: typeof d.数量 === 'number' ? d.数量 : (d.数量 ?? 1),
  }));
  servers.value = safeParseArray<Omit<DeviceRow, '_id'>>(asset.服务器).map((d) => ({
    _id: nextId(),
    设备名称: d.设备名称 ?? '',
    品牌: d.品牌 ?? '',
    型号: d.型号 ?? '',
    服务器操作系统及版本: (d as any).服务器操作系统及版本 ?? '',
    数据库管理系统及版本: (d as any).数据库管理系统及版本 ?? '',
    数量: typeof d.数量 === 'number' ? d.数量 : (d.数量 ?? 1),
  }));
  terminals.value = safeParseArray<Omit<DeviceRow, '_id'>>(asset.终端).map((d) => ({
    _id: nextId(),
    设备名称: d.设备名称 ?? '',
    品牌: d.品牌 ?? '',
    型号: d.型号 ?? '',
    操作系统及版本: (d as any).操作系统及版本 ?? '',
    设备用途: (d as any).设备用途 ?? '',
    数量: typeof d.数量 === 'number' ? d.数量 : (d.数量 ?? 1),
  }));
  bizSystems.value = safeParseArray<Omit<BizSystemRow, '_id'>>(asset.业务应用系统).map((d) => ({
    _id: nextId(),
    ...d,
  }));
}

function syncToTemplate() {
  const asset = props.templateData.资产情况;
  asset.机房 = serializeObject(roomInfo.value);

  const stripId = (rows: DeviceRow[]) =>
    rows.map(({ _id, ...rest }) => ({
      ...rest,
    }));

  asset.网络设备 = serializeArray(stripId(networkDevices.value));
  asset.安全设备 = serializeArray(stripId(securityDevices.value));
  asset.服务器 = serializeArray(stripId(servers.value));
  asset.终端 = serializeArray(stripId(terminals.value));
  asset.业务应用系统 = serializeArray(
    bizSystems.value.map(({ _id, ...rest }) => ({
      ...rest,
    })),
  );
}

onMounted(() => {
  loadFromTemplate();
});

watch(
  () => props.templateData.资产情况,
  () => {
    loadFromTemplate();
  },
  { deep: true },
);

watch([roomInfo, networkDevices, securityDevices, servers, terminals, bizSystems], () => {
  syncToTemplate();
}, { deep: true });

function addNetworkDevice() {
  networkDevices.value.push({
    _id: nextId(),
    设备名称: '',
    品牌: '',
    型号: '',
    用途: '',
    数量: 1,
  });
}

function removeNetworkDevice(index: number) {
  networkDevices.value.splice(index, 1);
}

function addSecurityDevice() {
  securityDevices.value.push({
    _id: nextId(),
    设备名称: '',
    品牌: '',
    型号: '',
    用途: '',
    数量: 1,
  });
}

function removeSecurityDevice(index: number) {
  securityDevices.value.splice(index, 1);
}

function addServer() {
  servers.value.push({
    _id: nextId(),
    设备名称: '',
    品牌: '',
    型号: '',
    服务器操作系统及版本: '',
    数据库管理系统及版本: '',
    数量: 1,
  });
}

function removeServer(index: number) {
  servers.value.splice(index, 1);
}

function addTerminal() {
  terminals.value.push({
    _id: nextId(),
    设备名称: '',
    品牌: '',
    型号: '',
    操作系统及版本: '',
    设备用途: '',
    数量: 1,
  });
}

function removeTerminal(index: number) {
  terminals.value.splice(index, 1);
}

function addBizSystem() {
  bizSystems.value.push({
    _id: nextId(),
    系统名称: '',
    主要功能: '',
    开发厂商: '',
  });
}

function removeBizSystem(index: number) {
  bizSystems.value.splice(index, 1);
}
</script>

<style scoped>
.module-title {
  font-size: 16px;
  font-weight: 600;
  margin-bottom: 20px;
  color: #1f2937;
}

.block {
  margin-bottom: 20px;
  padding: 16px;
  border-radius: 8px;
  border: 1px solid #e5e7eb;
  background: #fafafa;
}

.block-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 10px;
  gap: 8px;
}

.block-title {
  font-size: 14px;
  font-weight: 600;
  color: #111827;
}

.block-tip {
  font-size: 12px;
  color: #6b7280;
}

.room-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 12px;
}

.room-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.form-label {
  font-size: 13px;
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

.table-wrapper {
  overflow-x: auto;
}

.asset-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
  background: #fff;
}

.asset-table th,
.asset-table td {
  border: 1px solid #e5e7eb;
  padding: 6px 8px;
  text-align: left;
}

.asset-table th {
  background: #f9fafb;
  font-weight: 500;
  color: #4b5563;
}

.table-input {
  width: 100%;
  border: none;
  outline: none;
  font-size: 13px;
  padding: 2px 4px;
  background: transparent;
}

.table-input:focus {
  outline: 1px solid #3b82f6;
  border-radius: 2px;
  background: #eff6ff;
}

.qty-input {
  text-align: right;
}

.btn-mini {
  border: 1px solid #3b82f6;
  background: #eff6ff;
  color: #1d4ed8;
  border-radius: 9999px;
  font-size: 12px;
  padding: 4px 10px;
  cursor: pointer;
  white-space: nowrap;
}

.btn-mini:hover {
  background: #dbeafe;
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
  padding: 10px;
  font-size: 12px;
}
</style>