<script setup lang="ts">
import { computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { chapters } from '../content'
import ContentPage from '../components/ContentPage.vue'

const route = useRoute()
const router = useRouter()
const slug = computed(() => route.params.slug as string | undefined)

const chapter = computed(() =>
  chapters.find(c => c.slug === slug.value)
)

const modules = import.meta.glob('../content/*.md', { query: '?raw', import: 'default', eager: true }) as Record<string, string>

const content = computed(() => {
  if (!slug.value) return ''
  const key = Object.keys(modules).find(k => k.includes(slug.value!))
  return key ? modules[key] : ''
})

const descriptions: Record<string, string> = {
  'install-guide': '安装、常用命令、brainstorming 写文档、frontend 出方案、子代理执行',
  'free-quota': '每天白嫖 opencode 额度的流程与 VPN 换 IP 方法',
  'vision-model': '配置视觉模型实现截图分析、画图等图片理解能力',
}

function goDoc(slug: string) {
  router.push(`/teaching/${slug}`)
}
</script>

<template>
  <div v-if="!slug">
    <h2 class="page-title">教学文档</h2>
    <p class="page-desc">选择一篇文档开始学习 OpenCode 的使用</p>
    <div class="doc-grid">
      <div
        v-for="ch in chapters"
        :key="ch.slug"
        class="doc-card"
        @click="goDoc(ch.slug)"
      >
        <span class="doc-icon">
          <svg v-if="ch.slug === 'install-guide'" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/></svg>
          <svg v-else-if="ch.slug === 'free-quota'" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="8" width="18" height="12" rx="2"/><path d="M12 2v6"/><path d="M9 5h6"/></svg>
          <svg v-else width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/></svg>
        </span>
        <div class="doc-info">
          <div class="doc-name">{{ ch.title }}</div>
          <div class="doc-desc">{{ descriptions[ch.slug] }}</div>
        </div>
        <span class="doc-arrow">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><line x1="5" y1="12" x2="19" y2="12"/><polyline points="12 5 19 12 12 19"/></svg>
        </span>
      </div>
    </div>
  </div>
  <div v-else-if="!chapter">
    <p class="not-found">文档未找到</p>
  </div>
  <ContentPage v-else :content="content" :title="chapter.title" />
</template>

<style scoped>
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
  margin: 0 0 1.5rem 0;
}

.doc-grid {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.doc-card {
  display: flex;
  align-items: center;
  gap: 1rem;
  background: #fff;
  border-radius: 14px;
  border: 2px solid #fde68a;
  padding: 1rem 1.25rem;
  cursor: pointer;
  transition: all 0.15s;
  box-shadow: 6px 6px 12px rgba(146, 64, 14, 0.06), -3px -3px 8px rgba(255, 255, 255, 0.4);
}

.doc-card:hover {
  border-color: #d97706;
  transform: translateY(-1px);
  box-shadow: 6px 8px 16px rgba(146, 64, 14, 0.1), -3px -3px 8px rgba(255, 255, 255, 0.4);
}

.doc-icon {
  font-size: 1.8rem;
  flex-shrink: 0;
}

.doc-info {
  flex: 1;
  min-width: 0;
}

.doc-name {
  font-family: 'Fredoka', sans-serif;
  font-size: 0.95rem;
  font-weight: 500;
  color: #78350f;
  margin-bottom: 0.15rem;
}

.doc-desc {
  font-family: 'Nunito', sans-serif;
  font-size: 0.8rem;
  color: #92400e;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.doc-arrow {
  flex-shrink: 0;
  font-family: 'Nunito', sans-serif;
  font-size: 1.1rem;
  color: #d97706;
  transition: transform 0.15s;
}

.doc-card:hover .doc-arrow {
  transform: translateX(3px);
}

.not-found {
  font-family: 'Nunito', sans-serif;
  color: #b45309;
}
</style>
