<template>
  <div class="port-scan">
    <div class="card-header">
      <div class="card-header-icon">🔍</div>
      <h2 class="card-title">端口扫描</h2>
    </div>
    <div class="card-content">
      <p>这里是「端口扫描」的功能内容区域</p>
      <p class="card-desc">一键扫描目标端口状态，生成等保合规性检测报告</p>
    </div>
  </div>
</template>

<script setup lang="ts">
// 后续可扩展端口扫描的具体功能逻辑
</script>

<style scoped>
.port-scan {
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