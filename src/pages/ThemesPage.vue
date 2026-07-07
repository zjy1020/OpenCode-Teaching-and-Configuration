<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '../lib/tauri'

const installed = ref(false)
const scanning = ref(true)
const working = ref(false)
const error = ref('')

onMounted(async () => {
  await check()
})

async function check() {
  scanning.value = true
  error.value = ''
  try {
    installed.value = await invoke<boolean>('check_theme_installed')
  } catch (e: any) {
    error.value = String(e)
  } finally {
    scanning.value = false
  }
}

async function handleImport() {
  working.value = true
  error.value = ''
  try {
    await invoke('import_theme')
    installed.value = true
  } catch (e: any) {
    error.value = String(e)
  } finally {
    working.value = false
  }
}

async function handleRemove() {
  working.value = true
  error.value = ''
  try {
    await invoke('remove_theme')
    installed.value = false
  } catch (e: any) {
    error.value = String(e)
  } finally {
    working.value = false
  }
}

async function handleReset() {
  working.value = true
  error.value = ''
  try {
    await invoke('reset_tui_config')
  } catch (e: any) {
    error.value = String(e)
  } finally {
    working.value = false
  }
}

async function handleSync() {
  working.value = true
  error.value = ''
  try {
    await invoke('sync_author_config')
    installed.value = true
  } catch (e: any) {
    error.value = String(e)
  } finally {
    working.value = false
  }
}
</script>

<template>
  <div>
    <div class="page-header">
      <div>
        <h2 class="page-title">主题管理</h2>
        <p class="page-desc" v-if="!scanning">
          主题 {{ installed ? '已安装' : '未导入' }}
        </p>
      </div>
      <div class="toolbar">
        <button class="toolbar-btn" :disabled="scanning || working" @click="check">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M21.5 2v6h-6M2.5 22v-6h6M2 11.5a10 10 0 0 1 18.8-4.3M22 12.5a10 10 0 0 1-18.8 4.2"/></svg>
          刷新
        </button>
        <button class="toolbar-btn primary" :disabled="scanning || working" @click="handleSync">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4"/></svg>
          同步作者配置
        </button>
        <button class="toolbar-btn" :disabled="scanning || working" @click="handleReset">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M21.5 2v6h-6M2.5 22v-6h6M2 11.5a10 10 0 0 1 18.8-4.3M22 12.5a10 10 0 0 1-18.8 4.2"/></svg>
          恢复默认
        </button>
      </div>
    </div>

    <div v-if="error" class="error-banner">{{ error }}</div>

    <p v-if="scanning" class="loading-text">扫描中...</p>

    <div v-else class="theme-card">
      <div class="theme-info">
        <span class="theme-name">ember-glow</span>
        <span class="theme-desc">作者推荐主题</span>
      </div>
      <div class="theme-actions">
        <span v-if="installed" class="badge-installed">已安装</span>
        <button
          v-if="!installed"
          class="action-btn import"
          :disabled="working"
          @click="handleImport"
        >导入</button>
        <button
          v-if="installed"
          class="action-btn remove"
          :disabled="working"
          @click="handleRemove"
        >卸载</button>
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
.toolbar {
  display: flex;
  gap: 0.5rem;
  flex-shrink: 0;
}
.toolbar-btn {
  display: inline-flex;
  align-items: center;
  gap: 0.35rem;
  padding: 0.4rem 0.85rem;
  border-radius: 20px;
  font-family: 'Nunito', sans-serif;
  font-size: 0.8rem;
  font-weight: 500;
  border: 2px solid #d97706;
  background: transparent;
  color: #d97706;
  cursor: pointer;
  transition: all 0.15s;
  white-space: nowrap;
}
.toolbar-btn:hover:not(:disabled) {
  background: #fef3c7;
}
.toolbar-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}
.toolbar-btn.primary {
  background: #fbbf24;
  color: #78350f;
  box-shadow: 3px 3px 6px rgba(146, 64, 14, 0.1);
}
.toolbar-btn.primary:hover:not(:disabled) {
  opacity: 0.85;
}
.loading-text {
  font-family: 'Nunito', sans-serif;
  font-size: 0.9rem;
  color: #b45309;
}
.error-banner {
  font-family: 'Nunito', sans-serif;
  font-size: 0.85rem;
  color: #dc2626;
  background: #fef2f2;
  border: 1px solid #fca5a5;
  border-radius: 10px;
  padding: 0.5rem 1rem;
  margin-bottom: 1rem;
}
.theme-card {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.75rem 1rem;
  background: #fffbf5;
  border: 1px solid #e8d8c8;
  border-radius: 12px;
  margin-bottom: 0.5rem;
}
.theme-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
}
.theme-name {
  font-family: 'JetBrains Mono', 'Fira Code', monospace;
  font-size: 0.9rem;
  font-weight: 600;
  color: #78350f;
}
.theme-desc {
  font-family: 'Nunito', sans-serif;
  font-size: 0.78rem;
  color: #92400e;
}
.theme-actions {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}
.badge-installed {
  font-family: 'Nunito', sans-serif;
  font-size: 0.78rem;
  font-weight: 600;
  color: #059669;
  background: #ecfdf5;
  padding: 3px 10px;
  border-radius: 12px;
}
.action-btn {
  font-family: 'Nunito', sans-serif;
  font-size: 0.8rem;
  font-weight: 600;
  padding: 4px 14px;
  border-radius: 16px;
  border: 2px solid;
  cursor: pointer;
  transition: all 0.12s;
}
.action-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}
.action-btn.import {
  background: #fbbf24;
  border-color: #fbbf24;
  color: #78350f;
}
.action-btn.import:hover:not(:disabled) {
  opacity: 0.85;
}
.action-btn.remove {
  background: transparent;
  border-color: #dc2626;
  color: #dc2626;
}
.action-btn.remove:hover:not(:disabled) {
  background: #fef2f2;
}
</style>
