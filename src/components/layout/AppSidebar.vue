<template>
  <aside class="sidebar">
    <div
      class="nav-item"
      v-for="item in navList"
      :key="item.id"
      :class="{ active: activeNavId === item.id, hover: hoverNavId === item.id }"
      @click="handleNavClick(item.id)"
      @mouseenter="hoverNavId = item.id"
      @mouseleave="hoverNavId = 0"
    >
      <div class="nav-icon">{{ item.icon }}</div>
      <div class="nav-name">{{ item.name }}</div>
      <div class="nav-active-indicator" v-if="activeNavId === item.id"></div>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import type { NavItem } from '@/types';

// 定义Props（简化）
const props = defineProps<{
  navList: NavItem[];
  activeNavId: number;
}>();

// 定义Emits（Vue3推荐写法）
const emit = defineEmits<{
  (e: 'update:activeNavId', id: number): void;
}>();

const hoverNavId = ref(0);
const localActiveNavId = ref(props.activeNavId);

// 监听props变化
watch(() => props.activeNavId, (val) => {
  localActiveNavId.value = val;
});

// 处理导航点击
const handleNavClick = (id: number) => {
  localActiveNavId.value = id;
  emit('update:activeNavId', id);
};
</script>

<style scoped>
/* 原有样式保持不变 */
.sidebar {
  width: 200px;
  background: #f9fafb;
  border-right: 1px solid #e5e7eb;
  padding: 20px 0;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 20px;
  cursor: pointer;
  position: relative;
  transition: background-color 0.2s;
}

.nav-item.hover,
.nav-item:hover {
  background: #e5e7eb;
}

.nav-item.active {
  background: #eff6ff;
  color: #3b82f6;
}

.nav-icon {
  font-size: 16px;
}

.nav-name {
  font-size: 14px;
  font-weight: 500;
}

.nav-active-indicator {
  position: absolute;
  left: 0;
  top: 0;
  height: 100%;
  width: 4px;
  background: #3b82f6;
}
</style>