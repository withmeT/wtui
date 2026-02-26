<template>
  <div class="app-container">
    <!-- 顶部标题栏 -->
    <AppHeader
      v-model:searchText="searchText"
      v-model:isSearchFocus="isSearchFocus"
      @search="handleSearch"
    />

    <!-- 主内容区 -->
    <div class="main-content">
      <!-- 左侧导航栏 -->
      <AppSidebar
        :navList="navList"
        v-model:activeNavId="activeNavId"
      />

      <!-- 右侧内容区 -->
      <main class="content-area">
        <!-- 搜索结果提示 -->
        <div v-if="searchResult" class="search-tip" :class="searchTipClass">
          <span class="tip-icon">{{ getTipIcon() }}</span>
          <span class="tip-text">{{ cleanText(`搜索关键词：${searchText} → ${searchResult}`) }}</span>
        </div>

        <!-- 内容切换过渡 -->
        <transition name="content-fade">
          <div class="content-card" :key="activeNavId">
            <!-- 定级备案 -->
            <LevelFiling v-if="activeNavId === 1" />

            <!-- 文件格式转换 -->
            <FileConvert v-else-if="activeNavId === 2" />

            <!-- 端口扫描 -->
            <PortScan v-else-if="activeNavId === 3" />

            <!-- 并网检测 -->
            <GridCheck v-else-if="activeNavId === 4" />
          </div>


        </transition>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { AppHeader, AppSidebar } from './components/layout';
import LevelFiling from './components/business/level-filing/LevelFiling.vue';
import FileConvert from './components/business/file-convert/FileConvert.vue';
import PortScan from './components/business/port-scan/PortScan.vue';
import GridCheck from './components/business/grid-check/GridCheck.vue';
import type { NavItem } from './types';
import { cleanText, getSearchTipClass } from './utils/common';

// 导航列表数据
const navList = ref<NavItem[]>([
  { id: 1, name: '定级备案', icon: '📊' },
  { id: 2, name: '文件格式转换', icon: '📁' },
  { id: 3, name: '端口扫描', icon: '🔍' },
  { id: 4, 
    name: '并网检测', 
    icon: '📅' }
]);

// 搜索相关
const searchText = ref('');
const isSearchFocus = ref(false);
const searchResult = ref('');
const activeNavId = ref(4); // 默认选中并网检测

// 计算搜索提示样式
const searchTipClass = computed(() => {
  return getSearchTipClass(searchResult.value);
});

// 获取提示图标
const getTipIcon = (): string => {
  if (!searchResult.value) return '';
  if (searchResult.value.includes('找到')) return '✅';
  if (searchResult.value.includes('请输入')) return '⚠️';
  return '❌';
};

// 处理搜索
const handleSearch = () => {
  if (!searchText.value.trim()) {
    searchResult.value = '请输入搜索关键词';
    return;
  }

  // 模拟搜索逻辑
  const lowerSearchText = searchText.value.toLowerCase().trim();
  const matchedNav = navList.value.find(item => 
    item.name.toLowerCase().includes(lowerSearchText)
  );

  if (matchedNav) {
    activeNavId.value = matchedNav.id;
    searchResult.value = `找到匹配功能：${matchedNav.name}`;
  } else {
    searchResult.value = '未找到匹配的功能';
  }

  // 3秒后隐藏搜索提示
  setTimeout(() => {
    searchResult.value = '';
  }, 3000);
};
</script>

<style scoped>
.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100%;
  background: #f3f4f6;
}

.main-content {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.content-area {
  flex: 1;
  padding: 20px;
  overflow-y: auto;
}

.search-tip {
  padding: 12px;
  border-radius: 4px;
  margin-bottom: 20px;
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
}

.search-tip-success {
  background: #dcfce7;
  color: #16a34a;
}

.search-tip-warning {
  background: #fffbeb;
  color: #d97706;
}

.search-tip-error {
  background: #fee2e2;
  color: #dc2626;
}

.tip-icon {
  font-size: 16px;
}

.content-card {
  width: 100%;
  height: 100%;
}

/* 过渡动画 */
.content-fade-enter-from,
.content-fade-leave-to {
  opacity: 0;
  transform: translateY(10px);
}

.content-fade-enter-active,
.content-fade-leave-active {
  transition: all 0.3s ease;
}
</style>