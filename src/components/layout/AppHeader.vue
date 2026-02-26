<template>
  <header class="header">
    <div class="title-wrapper">
      <div class="title-icon">🛡️</div>
      <div class="title-text">等保工具平台</div>
    </div>
    <div class="search-wrapper">
      <input
        v-model="localSearchText"
        type="text"
        class="search-input"
        :class="{ 'search-input-focus': isSearchFocus }"
        placeholder="搜索功能/关键词..."
        @keyup.enter="handleSearch"
        @focus="isSearchFocus = true"
        @blur="isSearchFocus = false"
      />
      <button class="search-btn" @click="handleSearch">
        <span class="search-icon">🔍</span>
      </button>
    </div>
  </header>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'

// 1. 定义 Props（从父组件接收）
const props = withDefaults(defineProps<{
  searchText: string
  isSearchFocus: boolean
}>(), {
  searchText: '',
  isSearchFocus: false
})

// 2. 定义 Emits（向父组件同步更新）
const emit = defineEmits<{
  (e: 'update:searchText', value: string): void
  (e: 'update:isSearchFocus', value: boolean): void
  (e: 'search'): void
}>()

// 3. 本地响应式变量，承接 props
const localSearchText = ref(props.searchText)
const localIsSearchFocus = ref(props.isSearchFocus)

// 4. 监听 props 变化，同步到本地变量
watch(() => props.searchText, (newVal) => {
  localSearchText.value = newVal
})
watch(() => props.isSearchFocus, (newVal) => {
  localIsSearchFocus.value = newVal
})

// 5. 监听本地变量变化，同步到父组件
watch(localSearchText, (newVal) => {
  emit('update:searchText', newVal)
})
watch(localIsSearchFocus, (newVal) => {
  emit('update:isSearchFocus', newVal)
})

// 6. 处理搜索事件
const handleSearch = () => {
  emit('search')
}
</script>

<style scoped>
.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 20px;
  height: 60px;
  background: #fff;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.title-wrapper {
  display: flex;
  align-items: center;
  gap: 8px;
}

.title-icon {
  font-size: 20px;
}

.title-text {
  font-size: 18px;
  font-weight: 600;
  color: #1f2937;
}

.search-wrapper {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 300px;
}

.search-input {
  flex: 1;
  height: 36px;
  padding: 0 12px;
  border: 1px solid #d1d5db;
  border-radius: 4px;
  outline: none;
  transition: border-color 0.2s;
}

.search-input-focus {
  border-color: #3b82f6;
}

.search-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border: none;
  border-radius: 4px;
  background: #3b82f6;
  color: #fff;
  cursor: pointer;
  transition: background-color 0.2s;
}

.search-btn:hover {
  background: #2563eb;
}
</style>