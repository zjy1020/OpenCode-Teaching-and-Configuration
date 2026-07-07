<script setup lang="ts">
import type { Command } from '../data/commands'
import MarkdownRenderer from './MarkdownRenderer.vue'

defineProps<{
  command: Command
}>()

const emit = defineEmits<{
  back: []
}>()
</script>

<template>
  <div>
    <button class="back-btn" @click="emit('back')">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><line x1="19" y1="12" x2="5" y2="12"/><polyline points="12 19 5 12 12 5"/></svg>
      返回列表
    </button>

    <div class="detail-card">
      <div class="detail-header">
        <code>{{ command.name }}</code>
      </div>
      <p class="detail-desc">{{ command.description }}</p>
      <div class="detail-content">
        <MarkdownRenderer :content="command.content" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.back-btn {
  font-family: 'Nunito', sans-serif;
  font-size: 0.85rem;
  color: #d97706;
  background: none;
  border: none;
  cursor: pointer;
  padding: 0.25rem 0;
  margin-bottom: 1rem;
  display: flex;
  align-items: center;
  gap: 0.25rem;
}

.back-btn:hover {
  color: #b45309;
}

.detail-card {
  background: #fff;
  border-radius: 16px;
  border: 2px solid #fde68a;
  padding: 1.25rem;
  box-shadow: 6px 6px 12px rgba(146, 64, 14, 0.06), -3px -3px 8px rgba(255, 255, 255, 0.4);
}

.detail-header code {
  font-family: ui-monospace, monospace;
  font-size: 1.1rem;
  background: #fef3c7;
  border-radius: 6px;
  padding: 0.15rem 0.6rem;
  color: #78350f;
}

.detail-desc {
  font-family: 'Nunito', sans-serif;
  font-size: 0.88rem;
  color: #92400e;
  margin: 0.75rem 0;
  line-height: 1.5;
}

.detail-content {
  border-top: 1px solid #fde68a;
  padding-top: 0.75rem;
}
</style>
