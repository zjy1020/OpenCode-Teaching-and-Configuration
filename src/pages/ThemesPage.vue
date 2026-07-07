<script setup lang="ts">
import TuiConfigForm from '../components/TuiConfigForm.vue'
</script>

<template>
  <div>
    <div class="page-header">
      <div>
        <h2 class="page-title">主题管理</h2>
        <p class="page-desc">管理 OpenCode TUI 主题和配置</p>
      </div>
    </div>
    <TuiConfigForm />
  </div>
</template>

<style scoped>
.page-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  margin-bottom: 1.5rem;
}
.page-title {
  font-family: 'Fredoka', sans-serif;
  font-size: 1.35rem;
  font-weight: 600;
  color: #78350f;
  margin: 0 0 0.25rem 0;
}
.page-desc {
  font-family: 'Nunito', sans-serif;
  font-size: 0.85rem;
  color: #92400e;
  margin: 0;
}
</style>
