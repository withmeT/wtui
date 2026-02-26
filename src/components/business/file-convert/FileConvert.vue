<template>
  <div class="file-convert">
    <div class="card-header">
      <div class="card-header-icon">📁</div>
      <h2 class="card-title">文件格式转换</h2>
    </div>
    <div class="card-content">
      <p>这里是「文件格式转换」的功能内容区域</p>
      <p class="card-desc">支持等保相关文档的格式转换（PDF/Word/Excel 互转）</p>
    </div>
  </div>
</template>

<script setup lang="ts">
// 后续可扩展文件转换的具体功能逻辑
</script>

<style scoped>
.file-convert {
  width: 100%;
  height: 100%;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 20px;
}

.card-header-icon {
  font-size: 20px;
}

.card-title {
  font-size: 18px;
  font-weight: 600;
  color: #1f2937;
  margin: 0;
}

.card-content {
  padding: 20px;
  background: #fff;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  line-height: 1.8;
}

.card-desc {
  margin-top: 12px;
  color: #6b7280;
}
</style>