<script setup lang="ts">
import { useRoute, useRouter } from 'vue-router'

const route = useRoute()
const router = useRouter()

const teachingChapters = [
  { label: '安装与常用教学', slug: 'install-guide' },
  { label: '每天白嫖额度', slug: 'free-quota' },
  { label: '调用视觉模型', slug: 'vision-model' },
]

function isActive(path: string) {
  return route.path.startsWith(path)
}

function navigate(path: string) {
  router.push(path)
}

function keyNav(fn: () => void, e: KeyboardEvent) {
  if (e.key === 'Enter' || e.key === ' ') {
    e.preventDefault()
    fn()
  }
}
</script>

<template>
  <aside class="sidebar">
    <div class="sidebar-brand">OpenCode 教学</div>

    <nav class="sidebar-nav">
      <div class="nav-section">
        <div
          class="nav-item nav-section-title"
          :class="{ active: isActive('/teaching') }"
          tabindex="0"
          role="button"
          @click="navigate('/teaching')"
          @keydown="keyNav(() => navigate('/teaching'), $event)"
        >
          <span class="nav-icon">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20"/></svg>
          </span>
          <span>教学文档</span>
        </div>
        <div class="nav-subs">
          <div
            v-for="ch in teachingChapters"
            :key="ch.slug"
            class="nav-sub"
            :class="{ 'sub-active': route.params.slug === ch.slug }"
            tabindex="0"
            role="button"
            @click="navigate(`/teaching/${ch.slug}`)"
            @keydown="keyNav(() => navigate(`/teaching/${ch.slug}`), $event)"
          >
            {{ ch.label }}
          </div>
        </div>
      </div>

      <div
        class="nav-item"
        :class="{ active: isActive('/skills') }"
        tabindex="0"
        role="button"
        @click="navigate('/skills')"
        @keydown="keyNav(() => navigate('/skills'), $event)"
      >
        <span class="nav-icon">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"/></svg>
        </span>
        <span>技能管理</span>
      </div>

      <div
        class="nav-item"
        :class="{ active: isActive('/commands') }"
        tabindex="0"
        role="button"
        @click="navigate('/commands')"
        @keydown="keyNav(() => navigate('/commands'), $event)"
      >
        <span class="nav-icon">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="16 18 22 12 16 6"/><polyline points="8 6 2 12 8 18"/></svg>
        </span>
        <span>命令管理</span>
      </div>

      <div
        class="nav-item"
        :class="{ active: isActive('/themes') }"
        tabindex="0"
        role="button"
        @click="navigate('/themes')"
        @keydown="keyNav(() => navigate('/themes'), $event)"
      >
        <span class="nav-icon">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="4"/><path d="M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M6.34 17.66l-1.41 1.41M19.07 4.93l-1.41 1.41"/></svg>
        </span>
        <span>主题管理</span>
      </div>
    </nav>
  </aside>
</template>

<style scoped>
.sidebar {
  width: 16rem;
  background: linear-gradient(180deg, #fef3c7, #fde68a);
  display: flex;
  flex-direction: column;
  height: 100%;
}

.sidebar-brand {
  font-family: 'Fredoka', sans-serif;
  font-weight: 600;
  font-size: 0.95rem;
  color: #92400e;
  padding: 1rem 1.25rem;
  border-bottom: 1px solid rgba(146, 64, 14, 0.1);
}

.sidebar-nav {
  flex: 1;
  overflow-y: auto;
  padding: 0.75rem;
}

.nav-section {
  margin-bottom: 0.5rem;
}

.nav-section:last-child {
  margin-bottom: 0;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 0.75rem;
  border-radius: 14px;
  cursor: pointer;
  font-family: 'Fredoka', sans-serif;
  font-weight: 500;
  font-size: 0.85rem;
  color: #92400e;
  transition: all 0.15s;
  box-shadow: 3px 3px 6px rgba(146, 64, 14, 0.08), -2px -2px 4px rgba(255, 255, 255, 0.4);
  border: 2px solid rgba(255, 255, 255, 0.3);
  background: rgba(255, 255, 255, 0.55);
  margin-bottom: 0.5rem;
}

.nav-item:hover {
  background: rgba(255, 255, 255, 0.7);
}

.nav-item:focus-visible {
  outline: 2px solid #d97706;
  outline-offset: 2px;
}

.nav-item.active {
  background: #fffbeb;
  color: #b45309;
}

.nav-section-title {
  margin-bottom: 0.25rem;
}

.nav-icon {
  display: flex;
  align-items: center;
  flex-shrink: 0;
}

.nav-subs {
  margin-top: 4px;
}

.nav-sub {
  padding: 0.5rem 0.75rem;
  border-radius: 10px;
  font-family: 'Nunito', sans-serif;
  font-size: 0.78rem;
  color: #92400e;
  background: rgba(255, 255, 255, 0.35);
  margin-bottom: 3px;
  cursor: pointer;
  transition: all 0.15s;
}

.nav-sub:hover {
  background: rgba(255, 255, 255, 0.55);
}

.nav-sub:focus-visible {
  outline: 2px solid #d97706;
  outline-offset: 2px;
}

.nav-sub.sub-active {
  background: #fffbeb;
  color: #d97706;
  font-weight: 600;
  box-shadow: 2px 2px 4px rgba(146, 64, 14, 0.06);
}

</style>
