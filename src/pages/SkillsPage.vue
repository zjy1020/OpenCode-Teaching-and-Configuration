<script setup lang="ts">
import { ref, onMounted } from 'vue'
import SkillList from '../components/SkillList.vue'
import TuiConfigForm from '../components/TuiConfigForm.vue'
import { embeddedSkills } from '../data/skills'
import { useScanner } from '../composables/useScanner'
import { useImport } from '../composables/useImport'
import { invoke } from '../lib/tauri'

const { scanning, scanSkills, error: scanErr } = useScanner()
const { importing, importSkill, removeSkill, error: importErr } = useImport()
const localSkills = ref<string[]>([])
const importingAll = ref(false)
const progress = ref({ done: 0, total: 0 })
const error = ref('')

onMounted(async () => {
  const result = await scanSkills()
  localSkills.value = result.installed
})

async function handleRefresh() {
  error.value = ''
  const result = await scanSkills()
  localSkills.value = result.installed
  if (scanErr.value) error.value = scanErr.value
}

async function handleImport(name: string) {
  error.value = ''
  const ok = await importSkill(name)
  if (ok && !localSkills.value.includes(name)) {
    localSkills.value.push(name)
  }
  if (importErr.value) error.value = importErr.value
}

async function handleImportAll() {
  error.value = ''
  importingAll.value = true
  const targets = embeddedSkills.filter(s => !localSkills.value.includes(s.name))
  progress.value = { done: 0, total: targets.length }
  for (const skill of targets) {
    const ok = await importSkill(skill.name)
    if (ok && !localSkills.value.includes(skill.name)) {
      localSkills.value.push(skill.name)
    }
    progress.value.done++
    if (importErr.value) {
      error.value = importErr.value
      break
    }
  }
  importingAll.value = false
}

async function handleUninstall(name: string) {
  error.value = ''
  const ok = await removeSkill(name)
  if (ok) {
    localSkills.value = localSkills.value.filter(n => n !== name)
  }
  if (importErr.value) error.value = importErr.value
}

async function handleOpenFolder() {
  const dir = await invoke<string>('get_opencode_dir')
  await invoke('open_folder', { path: dir + '\\skills' })
}

const available = embeddedSkills.filter(s => !localSkills.value.includes(s.name)).length
</script>

<template>
  <div>
    <div class="page-header">
      <div>
        <h2 class="page-title">技能管理</h2>
        <p class="page-desc">已安装 {{ localSkills.length }}/{{ embeddedSkills.length }} · 可导入 {{ available }}</p>
      </div>
      <div class="toolbar">
        <button class="toolbar-btn" :disabled="scanning" @click="handleRefresh">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M21.5 2v6h-6M2.5 22v-6h6M2 11.5a10 10 0 0 1 18.8-4.3M22 12.5a10 10 0 0 1-18.8 4.2"/></svg>
          刷新
        </button>
        <button class="toolbar-btn" :disabled="scanning" @click="handleOpenFolder">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>
          打开文件夹
        </button>
        <button class="toolbar-btn primary" :disabled="importing || importingAll" @click="handleImportAll">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
          {{ importingAll ? `正在导入 ${progress.done}/${progress.total}...` : '一键导入' }}
        </button>
      </div>
    </div>
    <div v-if="error" class="error-banner">{{ error }}</div>
    <p v-if="scanning" class="loading-text">扫描中...</p>
    <SkillList v-else :local-skills="localSkills" @import="handleImport" @uninstall="handleUninstall" />
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
  font-size: 0.85rem;
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
</style>
