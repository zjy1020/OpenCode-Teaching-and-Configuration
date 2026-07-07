<script setup lang="ts">
import TuiConfigForm from '../components/TuiConfigForm.vue'
import ThemePreview from '../components/ThemePreview.vue'
</script>

<template>
  <div>
    <div class="page-header">
      <div>
        <h2 class="page-title">主题管理</h2>
        <p class="page-desc">管理 OpenCode TUI 主题和配置</p>
      </div>
    </div>
    <div class="theme-layout">
      <div class="theme-config-col">
        <TuiConfigForm />
      </div>
      <div class="theme-preview-col">
        <ThemePreview />
      </div>
    </div>
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
.theme-layout {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1.5rem;
  align-items: start;
}
@media (max-width: 900px) {
  .theme-layout {
    grid-template-columns: 1fr;
  }
}
</style>
