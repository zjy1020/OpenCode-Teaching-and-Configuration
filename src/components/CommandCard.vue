<script setup lang="ts">
import type { CommandWithStatus } from '../data/commands'

defineProps<{
  command: CommandWithStatus
}>()

const emit = defineEmits<{
  import: [name: string]
  view: [name: string]
  uninstall: [name: string]
}>()

const statusLabels: Record<string, string> = {
  installed: '已安装',
  available: '可导入',
  'local-only': '仅本地',
}

const statusStyles: Record<string, string> = {
  installed: 'background: #d1fae5; color: #065f46;',
  available: 'background: #dbeafe; color: #1e40af;',
  'local-only': 'background: #f1f5f9; color: #64748b;',
}
</script>

<template>
  <div class="cmd-card">
    <div class="cmd-info">
      <div class="cmd-name">
        <code>{{ command.name }}</code>
      </div>
      <div class="cmd-desc">{{ command.description }}</div>
    </div>
    <button class="cmd-view-btn" @click="emit('view', command.name)">
      查看
    </button>
    <span class="cmd-status" :style="statusStyles[command.status]">
      {{ statusLabels[command.status] }}
    </span>
    <button
      v-if="command.status === 'available'"
      class="cmd-import-btn"
      @click="emit('import', command.name)"
    >
      导入
    </button>
    <button
      v-if="command.status === 'installed'"
      class="cmd-uninstall-btn"
      @click="emit('uninstall', command.name)"
    >
      卸载
    </button>
  </div>
</template>

<style scoped>
.cmd-card {
  display: flex;
  align-items: center;
  gap: 1.25rem;
  background: #fff;
  border-radius: 14px;
  border: 2px solid #fde68a;
  padding: 0.75rem 1rem;
  box-shadow: 6px 6px 12px rgba(146, 64, 14, 0.06), -3px -3px 8px rgba(255, 255, 255, 0.4);
  transition: border-color 0.15s;
}

.cmd-card:hover {
  border-color: #d97706;
}

.cmd-info {
  flex: 1;
  min-width: 0;
}

.cmd-name code {
  font-family: ui-monospace, monospace;
  font-size: 0.85rem;
  background: #fef3c7;
  border-radius: 4px;
  padding: 0.1rem 0.45rem;
  color: #92400e;
}

.cmd-desc {
  font-family: 'Nunito', sans-serif;
  font-size: 0.8rem;
  color: #92400e;
  margin-top: 0.25rem;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.cmd-view-btn {
  flex-shrink: 0;
  padding: 0.3rem 0.75rem;
  border-radius: 20px;
  background: transparent;
  color: #d97706;
  font-family: 'Nunito', sans-serif;
  font-size: 0.8rem;
  border: 2px solid #d97706;
  cursor: pointer;
  transition: all 0.15s;
}

.cmd-view-btn:hover {
  background: #fef3c7;
}

.cmd-status {
  flex-shrink: 0;
  padding: 0.15rem 0.6rem;
  border-radius: 20px;
  font-family: 'Nunito', sans-serif;
  font-size: 0.75rem;
  font-weight: 500;
}

.cmd-import-btn {
  flex-shrink: 0;
  padding: 0.35rem 0.85rem;
  border-radius: 20px;
  background: #fbbf24;
  color: #78350f;
  font-family: 'Fredoka', sans-serif;
  font-size: 0.8rem;
  font-weight: 500;
  border: 2px solid #d97706;
  cursor: pointer;
  box-shadow: 3px 3px 6px rgba(146, 64, 14, 0.1);
  transition: opacity 0.15s;
}

.cmd-import-btn:hover {
  opacity: 0.85;
}

.cmd-uninstall-btn {
  flex-shrink: 0;
  padding: 0.35rem 0.85rem;
  border-radius: 20px;
  background: transparent;
  color: #dc2626;
  font-family: 'Fredoka', sans-serif;
  font-size: 0.8rem;
  font-weight: 500;
  border: 2px solid #fca5a5;
  cursor: pointer;
  transition: all 0.15s;
}

.cmd-uninstall-btn:hover {
  background: #fef2f2;
  border-color: #ef4444;
}
</style>
