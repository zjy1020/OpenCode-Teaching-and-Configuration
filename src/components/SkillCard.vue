<script setup lang="ts">
import type { SkillWithStatus } from '../data/skills'
import { categoryLabels } from '../data/skills'

defineProps<{
  skill: SkillWithStatus
}>()

const emit = defineEmits<{
  import: [name: string]
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

const categoryStyles: Record<string, string> = {
  superpowers: 'background: #ede9fe; color: #5b21b6;',
  frontend: 'background: #fce7f3; color: #9d174d;',
  utility: 'background: #d1fae5; color: #065f46;',
}
</script>

<template>
  <div class="skill-card">
    <div class="skill-info">
      <div class="skill-name">{{ skill.name }}</div>
      <div class="skill-desc">{{ skill.description }}</div>
    </div>
    <div class="skill-meta">
      <span class="skill-category" :style="categoryStyles[skill.category]">
        {{ categoryLabels[skill.category] }}
      </span>
      <span class="skill-status" :style="statusStyles[skill.status]">
        {{ statusLabels[skill.status] }}
      </span>
      <a
        v-if="skill.repo"
        :href="skill.repo"
        target="_blank"
        rel="noopener noreferrer"
        class="skill-gh-btn"
        title="GitHub 仓库"
      >
        <svg viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
          <path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0 0 24 12c0-6.63-5.37-12-12-12z"/>
        </svg>
      </a>
      <span v-else class="skill-no-repo">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="15" y1="9" x2="9" y2="15"/><line x1="9" y1="9" x2="15" y2="15"/></svg>
      </span>
      <button
        v-if="skill.status === 'available'"
        class="skill-import-btn"
        @click="emit('import', skill.name)"
      >
        导入
      </button>
      <button
        v-if="skill.status === 'installed'"
        class="skill-uninstall-btn"
        @click="emit('uninstall', skill.name)"
      >
        卸载
      </button>
    </div>
  </div>
</template>

<style scoped>
.skill-card {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  background: #fff;
  border-radius: 14px;
  border: 2px solid #fde68a;
  padding: 0.75rem 1rem;
  box-shadow: 6px 6px 12px rgba(146, 64, 14, 0.06), -3px -3px 8px rgba(255, 255, 255, 0.4);
  transition: border-color 0.15s;
}

.skill-card:hover {
  border-color: #d97706;
}

.skill-info {
  min-width: 0;
  margin-right: auto;
}

.skill-meta {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  flex-shrink: 0;
}

.skill-name {
  font-family: 'Fredoka', sans-serif;
  font-weight: 500;
  font-size: 0.9rem;
  color: #78350f;
}

.skill-desc {
  font-family: 'Nunito', sans-serif;
  font-size: 0.8rem;
  color: #92400e;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.skill-category {
  flex-shrink: 0;
  padding: 0.15rem 0.6rem;
  border-radius: 20px;
  font-family: 'Nunito', sans-serif;
  font-size: 0.7rem;
  font-weight: 500;
  white-space: nowrap;
}

.skill-status {
  flex-shrink: 0;
  padding: 0.15rem 0.6rem;
  border-radius: 20px;
  font-family: 'Nunito', sans-serif;
  font-size: 0.75rem;
  font-weight: 500;
}

.skill-gh-btn {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background: #1f2937;
  color: #fff;
  text-decoration: none;
  border: none;
  cursor: pointer;
  transition: background 0.15s;
}

.skill-gh-btn:hover {
  background: #374151;
}

.skill-no-repo {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background: #fef3c7;
  color: #b45309;
  font-size: 0.85rem;
  font-weight: 600;
}

.skill-import-btn {
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

.skill-import-btn:hover {
  opacity: 0.85;
}

.skill-uninstall-btn {
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

.skill-uninstall-btn:hover {
  background: #fef2f2;
  border-color: #ef4444;
}
</style>
