<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '../lib/tauri'

interface Attention {
  enabled: boolean
  notifications: boolean
  sound: boolean
  volume: number
}

interface TuiConfig {
  theme: string
  diff_style: string
  attention: Attention
}

const embeddedThemes = ref<string[]>([])
const config = ref<TuiConfig>({
  theme: 'tokyonight',
  diff_style: 'auto',
  attention: { enabled: true, notifications: true, sound: true, volume: 0.4 },
})
const saving = ref(false)
const status = ref('')

onMounted(async () => {
  try {
    const c = await invoke<TuiConfig>('read_tui_config')
    config.value = c
  } catch {}
  try {
    embeddedThemes.value = await invoke<string[]>('list_embedded_themes')
  } catch {}
})

async function handleSave() {
  saving.value = true
  status.value = ''
  try {
    await invoke('write_tui_config', { config: JSON.parse(JSON.stringify(config.value)) })
    status.value = 'success'
  } catch (e: any) {
    status.value = `error: ${e}`
  }
  saving.value = false
  setTimeout(() => { status.value = '' }, 3000)
}
</script>

<template>
  <div class="config-section">
    <h3 class="config-heading">OpenCode TUI 配置</h3>
    <p class="config-desc">一键写入 ~/.config/opencode/tui.json</p>

    <div class="config-grid">
      <div class="field">
        <label class="field-label">主题 Theme</label>
        <select v-model="config.theme" class="select-input">
          <option v-for="t in embeddedThemes" :key="t" :value="t">{{ t }}</option>
        </select>
      </div>

      <div class="field">
        <label class="field-label">Diff 布局</label>
        <div class="radio-group">
          <label class="radio-item">
            <input type="radio" v-model="config.diff_style" value="auto" />
            <span>auto — 自适应</span>
          </label>
          <label class="radio-item">
            <input type="radio" v-model="config.diff_style" value="stacked" />
            <span>stacked — 上下排列</span>
          </label>
        </div>
      </div>

      <div class="field">
        <label class="field-label">通知 Attention</label>
        <div class="toggle-row">
          <label class="toggle-item">
            <input type="checkbox" v-model="config.attention.enabled" />
            <span>启用</span>
          </label>
          <label class="toggle-item" v-if="config.attention.enabled">
            <input type="checkbox" v-model="config.attention.notifications" />
            <span>系统通知</span>
          </label>
          <label class="toggle-item" v-if="config.attention.enabled">
            <input type="checkbox" v-model="config.attention.sound" />
            <span>提示音</span>
          </label>
        </div>
        <div v-if="config.attention.enabled" class="volume-row">
          <label class="field-label-small">音量 ({{ config.attention.volume }})</label>
          <input type="range" min="0" max="1" step="0.1" v-model.number="config.attention.volume" class="slider" />
        </div>
      </div>
    </div>

    <div class="action-row">
      <button class="save-btn" :disabled="saving" @click="handleSave">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/><polyline points="17 21 17 13 7 13 7 21"/><polyline points="7 3 7 8 15 8"/></svg>
        {{ saving ? '写入中...' : '一键写入配置' }}
      </button>
      <span v-if="status === 'success'" class="status-ok">✅ 已写入</span>
      <span v-else-if="status" class="status-err">{{ status }}</span>
    </div>
  </div>
</template>

<style scoped>
.config-section {
  margin-top: 2rem;
  padding: 1.25rem 1.5rem;
  background: #fffbf5;
  border: 1px solid #e8d8c8;
  border-radius: 14px;
}
.config-heading {
  font-family: 'Fredoka', sans-serif;
  font-size: 1.1rem;
  font-weight: 600;
  color: #78350f;
  margin: 0 0 0.2rem 0;
}
.config-desc {
  font-family: 'Nunito', sans-serif;
  font-size: 0.8rem;
  color: #92400e;
  margin: 0 0 1rem 0;
}
.config-grid {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}
.field {
  display: flex;
  flex-direction: column;
}
.field-label {
  font-family: 'Nunito', sans-serif;
  font-size: 0.85rem;
  font-weight: 600;
  color: #78350f;
  margin-bottom: 0.35rem;
}
.field-label-small {
  font-family: 'Nunito', sans-serif;
  font-size: 0.78rem;
  color: #92400e;
  margin-bottom: 0.25rem;
}
.select-input {
  padding: 0.45rem 0.6rem;
  border: 2px solid #d97706;
  border-radius: 10px;
  font-family: 'Nunito', sans-serif;
  font-size: 0.85rem;
  background: #fff;
  color: #78350f;
  outline: none;
  cursor: pointer;
}
.select-input:focus {
  border-color: #fbbf24;
  box-shadow: 0 0 0 3px rgba(251, 191, 36, 0.2);
}
.text-input {
  padding: 0.4rem 0.6rem;
  border: 2px solid #d97706;
  border-radius: 10px;
  font-family: 'Nunito', sans-serif;
  font-size: 0.85rem;
  background: #fff;
  color: #78350f;
  outline: none;
}
.text-input:focus {
  border-color: #fbbf24;
  box-shadow: 0 0 0 3px rgba(251, 191, 36, 0.2);
}
.radio-group {
  display: flex;
  gap: 1.2rem;
}
.radio-item {
  display: flex;
  align-items: center;
  gap: 0.35rem;
  font-family: 'Nunito', sans-serif;
  font-size: 0.85rem;
  color: #78350f;
  cursor: pointer;
}
.radio-item input[type="radio"] {
  accent-color: #d97706;
}
.toggle-row {
  display: flex;
  gap: 1rem;
  flex-wrap: wrap;
}
.toggle-item {
  display: flex;
  align-items: center;
  gap: 0.35rem;
  font-family: 'Nunito', sans-serif;
  font-size: 0.85rem;
  color: #78350f;
  cursor: pointer;
}
.toggle-item input[type="checkbox"] {
  accent-color: #d97706;
}
.volume-row {
  margin-top: 0.4rem;
}
.slider {
  width: 100%;
  max-width: 240px;
  accent-color: #d97706;
}
.action-row {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin-top: 1.25rem;
}
.save-btn {
  display: inline-flex;
  align-items: center;
  gap: 0.35rem;
  padding: 0.45rem 1.2rem;
  border-radius: 20px;
  font-family: 'Nunito', sans-serif;
  font-size: 0.85rem;
  font-weight: 600;
  border: none;
  background: #fbbf24;
  color: #78350f;
  cursor: pointer;
  box-shadow: 3px 3px 6px rgba(146, 64, 14, 0.12);
  transition: all 0.15s;
}
.save-btn:hover:not(:disabled) {
  opacity: 0.85;
}
.save-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
.status-ok {
  font-family: 'Nunito', sans-serif;
  font-size: 0.85rem;
  color: #059669;
  font-weight: 600;
}
.status-err {
  font-family: 'Nunito', sans-serif;
  font-size: 0.8rem;
  color: #dc2626;
}
</style>
