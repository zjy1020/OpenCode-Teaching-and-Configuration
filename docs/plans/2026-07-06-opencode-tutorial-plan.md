# OpenCode 教学工具 — 实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 构建一个 Vue 3 前端的 OpenCode 教学工具，含教学文档、Skills 管理和 Commands 管理三大板块。

**架构:** 数据驱动的内容架构，内嵌资源 + 本地扫描（mock），侧边栏导航 + 内容区 Markdown 渲染。

**Tech Stack:** Vue 3 + TypeScript + Vite + Tailwind CSS 4 + Vue Router + markdown-it

## 设计系统（所有任务必须遵守）

### 风格：Claymorphism（黏土风）
- 软 3D 双阴影：`box-shadow: 6px 6px 12px rgba(146,64,14,0.1), -3px -3px 8px rgba(255,255,255,0.4)`
- 圆角 14-16px
- 边框 2-3px 实色
- 内阴影用于凹陷效果：`box-shadow: inset 2px 2px 4px rgba(146,64,14,0.06)`

### 配色（Pudding 焦糖布丁）
```css
--color-primary: #D97706    /* 琥珀主色 */
--color-secondary: #F59E0B  /* 金 */
--color-background: #FFFBEB /* 暖黄底 */
--color-sidebar: linear-gradient(180deg,#fef3c7,#fde68a)
--color-titlebar: linear-gradient(90deg,#fbbf24,#f59e0b)
--color-text: #92400E       /* 正文 */
--color-heading: #78350F    /* 标题 */
--color-muted: #FCF6F0
--color-border: #FAEEE1
--color-surface: #FFFFFF
```

### 字体
- **标题**: Fredoka (Google Font) — `font-family: 'Fredoka', sans-serif; font-weight: 600`
- **正文**: Nunito (Google Font) — `font-family: 'Nunito', sans-serif; font-weight: 400`
- **代码**: 系统 monospace
- CSS Import: `@import url('https://fonts.googleapis.com/css2?family=Fredoka:wght@400;500;600;700&family=Nunito:wght@300;400;500;600;700&display=swap');`

### 图标规则
- 禁止用 emoji 作为结构化图标（导航、按钮等）
- 导航图标使用 Lucide SVG 图标（BookOpen, Wrench, Zap 等）
- 窗口按钮（最小化/最大化/关闭）保留系统符号

---

### Task 1: 项目脚手架搭建

**Files:**
- Create: `opencode-tutorial/package.json`
- Create: `opencode-tutorial/vite.config.ts`
- Create: `opencode-tutorial/tsconfig.json`
- Create: `opencode-tutorial/tsconfig.node.json`
- Create: `opencode-tutorial/index.html`
- Create: `opencode-tutorial/src/main.ts`
- Create: `opencode-tutorial/src/style.css`
- Create: `opencode-tutorial/src/vite-env.d.ts`

- [ ] **Step 1: 初始化项目文件**

```json:package.json
{
  "name": "opencode-tutorial",
  "private": true,
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vue-tsc -b && vite build",
    "preview": "vite preview"
  },
  "dependencies": {
    "vue": "^3.5.0",
    "vue-router": "^4.5.0",
    "markdown-it": "^14.1.0"
  },
  "devDependencies": {
    "@vitejs/plugin-vue": "^5.2.0",
    "@tailwindcss/vite": "^4.1.0",
    "tailwindcss": "^4.1.0",
    "typescript": "~5.7.0",
    "vite": "^6.3.0",
    "vue-tsc": "^2.2.0",
    "@types/markdown-it": "^14.1.0"
  }
}
```

```ts:vite.config.ts
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import tailwindcss from '@tailwindcss/vite'

export default defineConfig({
  plugins: [vue(), tailwindcss()],
})
```

```json:tsconfig.json
{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForExpose": true,
    "module": "ESNext",
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "skipLibCheck": true,
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "isolatedModules": true,
    "moduleDetection": "force",
    "noEmit": true,
    "jsx": "preserve",
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true,
    "paths": {
      "@/*": ["./src/*"]
    }
  },
  "include": ["src/**/*.ts", "src/**/*.tsx", "src/**/*.vue"]
}
```

```json:tsconfig.node.json
{
  "compilerOptions": {
    "target": "ES2022",
    "lib": ["ES2023"],
    "module": "ESNext",
    "skipLibCheck": true,
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "isolatedModules": true,
    "moduleDetection": "force",
    "noEmit": true,
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true
  },
  "include": ["vite.config.ts"]
}
```

```html:index.html
<!DOCTYPE html>
<html lang="zh-CN">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="/vite.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>OpenCode 教学工具</title>
  </head>
  <body class="bg-gray-50 text-gray-900">
    <div id="app"></div>
    <script type="module" src="/src/main.ts"></script>
  </body>
</html>
```

```ts:src/vite-env.d.ts
/// <reference types="vite/client" />
```

```ts:src/main.ts
import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import './style.css'

createApp(App).use(router).mount('#app')
```

```css:src/style.css
@import "tailwindcss";
@import url('https://fonts.googleapis.com/css2?family=Fredoka:wght@400;500;600;700&family=Nunito:wght@300;400;500;600;700&display=swap');

:root {
  --color-primary: #D97706;
  --color-secondary: #F59E0B;
  --color-background: #FFFBEB;
  --color-text: #92400E;
  --color-heading: #78350F;
  --color-muted: #FCF6F0;
  --color-border: #FAEEE1;
  --color-surface: #FFFFFF;
  --font-heading: 'Fredoka', sans-serif;
  --font-body: 'Nunito', sans-serif;
}

body {
  font-family: var(--font-body);
  color: var(--color-text);
  background: var(--color-background);
}

h1, h2, h3, h4, h5, h6 {
  font-family: var(--font-heading);
  color: var(--color-heading);
}
```

- [ ] **Step 2: 安装依赖并验证**

```bash
cd Desktop/opencode-tutorial && npm install
```

Expected: dependencies installed without errors.

```bash
cd Desktop/opencode-tutorial && npm run dev
```

Expected: Vite dev server starts on localhost, no console errors. Kill with Ctrl+C.

---

### Task 2: 路由定义

**Files:**
- Create: `src/router/index.ts`
- Create: `src/pages/WelcomePage.vue`
- Create: `src/pages/TeachingPage.vue`
- Create: `src/pages/SkillsPage.vue`
- Create: `src/pages/CommandsPage.vue`

- [ ] **Step 1: 创建路由配置和页面占位**

```ts:src/router/index.ts
import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'welcome',
      component: () => import('../pages/WelcomePage.vue'),
    },
    {
      path: '/teaching/:slug?',
      name: 'teaching',
      component: () => import('../pages/TeachingPage.vue'),
    },
    {
      path: '/skills',
      name: 'skills',
      component: () => import('../pages/SkillsPage.vue'),
    },
    {
      path: '/commands',
      name: 'commands',
      component: () => import('../pages/CommandsPage.vue'),
    },
  ],
})

export default router
```

- [ ] **Step 2: 创建四个页面组件（暂时占位内容）**

```vue:src/pages/WelcomePage.vue
<script setup lang="ts">
</script>

<template>
  <div>
    <h2 class="text-2xl font-bold mb-4">欢迎使用 OpenCode 教学工具</h2>
    <p class="text-gray-600">从左侧菜单选择内容开始学习。</p>
  </div>
</template>
```

```vue:src/pages/TeachingPage.vue
<script setup lang="ts">
import { useRoute } from 'vue-router'

const route = useRoute()
const slug = route.params.slug as string | undefined
</script>

<template>
  <div>
    <h2 class="text-2xl font-bold mb-4">教学文档</h2>
    <p class="text-gray-500" v-if="!slug">请从左侧目录选择一篇文档</p>
    <p v-else>当前文档: {{ slug }}</p>
  </div>
</template>
```

```vue:src/pages/SkillsPage.vue
<script setup lang="ts">
</script>

<template>
  <div>
    <h2 class="text-2xl font-bold mb-4">技能管理</h2>
  </div>
</template>
```

```vue:src/pages/CommandsPage.vue
<script setup lang="ts">
</script>

<template>
  <div>
    <h2 class="text-2xl font-bold mb-4">命令管理</h2>
  </div>
</template>
```

- [ ] **Step 3: 验证路由**

```bash
cd Desktop/opencode-tutorial && npm run dev
```

访问 `/`、`/teaching`、`/skills`、`/commands`，各页面能正常显示。

---

### Task 3: App 主布局 + 侧边栏

**Files:**
- Modify: `src/App.vue`
- Create: `src/components/Sidebar.vue`

- [ ] **Step 1: 创建侧边栏组件**

```vue:src/components/Sidebar.vue
<script setup lang="ts">
import { useRoute, useRouter } from 'vue-router'
import { ref } from 'vue'

const route = useRoute()
const router = useRouter()
const searchQuery = ref('')

const navItems = [
  { label: '教学文档', icon: '📖', path: '/teaching' },
  { label: '技能管理', icon: '🧰', path: '/skills' },
  { label: '命令管理', icon: '⚡', path: '/commands' },
]

const teachingChapters = [
  { label: '基础介绍', slug: 'intro' },
  { label: '快速上手', slug: 'quickstart' },
  { label: '使用 Skills', slug: 'skills-guide' },
  { label: 'Commands 定制', slug: 'commands-guide' },
]

function isActive(path: string) {
  return route.path.startsWith(path)
}

function navigate(path: string) {
  router.push(path)
}

function emitSearch() {
  // will be wired up with useSearch later
}
</script>

<template>
  <aside class="w-64 bg-slate-800 text-white flex flex-col h-full">
    <div class="p-4 border-b border-slate-700">
      <h1 class="text-lg font-bold text-sky-400">OpenCode 教学</h1>
    </div>

    <nav class="flex-1 overflow-y-auto p-3 space-y-1">
      <!-- 教学文档 -->
      <div class="mb-4">
        <div
          class="flex items-center gap-2 px-3 py-2 rounded-lg cursor-pointer hover:bg-slate-700 font-medium text-sky-400"
          @click="navigate('/teaching')"
        >
          <span>📖</span>
          <span>教学文档</span>
        </div>
        <div class="ml-4 mt-1 space-y-1">
          <div
            v-for="ch in teachingChapters"
            :key="ch.slug"
            class="px-3 py-1.5 rounded-md cursor-pointer text-sm text-slate-300 hover:bg-slate-700 hover:text-white transition-colors"
            :class="{ 'bg-slate-700 text-white': route.params.slug === ch.slug }"
            @click="navigate(`/teaching/${ch.slug}`)"
          >
            {{ ch.label }}
          </div>
        </div>
      </div>

      <!-- 技能管理 -->
      <div
        class="flex items-center gap-2 px-3 py-2 rounded-lg cursor-pointer hover:bg-slate-700 font-medium"
        :class="{ 'text-emerald-400': isActive('/skills'), 'text-slate-300': !isActive('/skills') }"
        @click="navigate('/skills')"
      >
        <span>🧰</span>
        <span>技能管理</span>
      </div>

      <!-- 命令管理 -->
      <div
        class="flex items-center gap-2 px-3 py-2 rounded-lg cursor-pointer hover:bg-slate-700 font-medium"
        :class="{ 'text-amber-400': isActive('/commands'), 'text-slate-300': !isActive('/commands') }"
        @click="navigate('/commands')"
      >
        <span>⚡</span>
        <span>命令管理</span>
      </div>
    </nav>

    <!-- 搜索 -->
    <div class="p-3 border-t border-slate-700">
      <div class="relative">
        <input
          v-model="searchQuery"
          type="text"
          placeholder="搜索..."
          class="w-full bg-slate-700 text-white rounded-lg px-3 py-2 text-sm placeholder-slate-400 outline-none focus:ring-2 focus:ring-sky-500"
          @input="emitSearch"
        />
      </div>
    </div>
  </aside>
</template>
```

- [ ] **Step 2: 更新 App.vue 主布局**

```vue:src/App.vue
<script setup lang="ts">
import Sidebar from './components/Sidebar.vue'
</script>

<template>
  <div class="flex h-screen">
    <Sidebar />
    <main class="flex-1 overflow-y-auto p-6">
      <router-view />
    </main>
  </div>
</template>
```

- [ ] **Step 3: 验证布局**

```bash
cd Desktop/opencode-tutorial && npm run dev
```

检查侧边栏三板块显示正常，点击导航切换路由，搜索框存在。

---

### Task 4: 教学文档数据 + ContentPage

**Files:**
- Create: `src/content/index.ts`
- Create: `src/content/01-intro.md`
- Create: `src/content/02-quickstart.md`
- Create: `src/content/03-skills-guide.md`
- Create: `src/content/04-commands-guide.md`
- Create: `src/components/ContentPage.vue`
- Modify: `src/pages/TeachingPage.vue`

- [ ] **Step 1: 创建教学文档目录索引和示例文档**

```ts:src/content/index.ts
export interface Chapter {
  slug: string
  title: string
}

export const chapters: Chapter[] = [
  { slug: 'intro', title: '基础介绍' },
  { slug: 'quickstart', title: '快速上手' },
  { slug: 'skills-guide', title: '使用 Skills' },
  { slug: 'commands-guide', title: 'Commands 定制' },
]
```

```md:src/content/01-intro.md
# 什么是 OpenCode？

OpenCode 是一个 CLI 编程助手...

## 主要特性

- 代码生成与补全
- 多模型支持
- Skills 插件系统
- Commands 自定义命令
```

```md:src/content/02-quickstart.md
# 快速上手

## 安装

```bash
npm install -g opencode
```

## 基本使用

```bash
opencode "你的问题"
```
```

```md:src/content/03-skills-guide.md
# 使用 Skills

Skills 是 OpenCode 的扩展能力...

## 加载 Skill

在对话中通过 @ 引用 skill。
```

```md:src/content/04-commands-guide.md
# Commands 定制

Commands 是自定义命令的入口点...
```

- [ ] **Step 2: 创建 ContentPage 组件（Markdown 渲染）**

```vue:src/components/ContentPage.vue
<script setup lang="ts">
import { computed } from 'vue'
import MarkdownRenderer from './MarkdownRenderer.vue'

const props = defineProps<{
  content: string
  title: string
}>()
</script>

<template>
  <article class="prose prose-slate max-w-none">
    <MarkdownRenderer :content="content" />
  </article>
</template>
```

- [ ] **Step 3: 创建 MarkdownRenderer 组件**

```vue:src/components/MarkdownRenderer.vue
<script setup lang="ts">
import { computed } from 'vue'
import MarkdownIt from 'markdown-it'

const md = new MarkdownIt({ html: true, linkify: true })

const props = defineProps<{
  content: string
}>()

const rendered = computed(() => md.render(props.content))
</script>

<template>
  <div class="markdown-body" v-html="rendered" />
</template>

<style scoped>
.markdown-body h1 { @apply text-2xl font-bold mb-4 mt-6; }
.markdown-body h2 { @apply text-xl font-semibold mb-3 mt-5; }
.markdown-body h3 { @apply text-lg font-semibold mb-2 mt-4; }
.markdown-body p { @apply mb-3 leading-relaxed; }
.markdown-body code { @apply bg-gray-100 rounded px-1.5 py-0.5 text-sm font-mono; }
.markdown-body pre { @apply bg-gray-900 text-gray-100 rounded-lg p-4 mb-4 overflow-x-auto; }
.markdown-body pre code { @apply bg-transparent p-0 text-inherit; }
.markdown-body ul { @apply list-disc pl-6 mb-3 space-y-1; }
.markdown-body ol { @apply list-decimal pl-6 mb-3 space-y-1; }
.markdown-body a { @apply text-sky-600 hover:underline; }
.markdown-body blockquote { @apply border-l-4 border-gray-300 pl-4 italic text-gray-600 mb-3; }
</style>
```

- [ ] **Step 4: 更新 TeachingPage 加载文档**

```vue:src/pages/TeachingPage.vue
<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { chapters } from '../content'
import ContentPage from '../components/ContentPage.vue'

const route = useRoute()
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
</script>

<template>
  <div v-if="!slug" class="flex items-center justify-center h-full text-gray-400">
    <p>从左侧目录选择一篇文档开始学习</p>
  </div>
  <div v-else-if="!chapter" class="text-gray-500">
    文档未找到
  </div>
  <ContentPage v-else :content="content" :title="chapter.title" />
</template>
```

- [ ] **Step 5: 验证文档渲染**

```bash
cd Desktop/opencode-tutorial && npm run dev
```

点击侧边栏章节，Markdown 内容正确渲染。

---

### Task 5: 内嵌 Skills 数据 + 技能列表

**Files:**
- Create: `src/data/skills.ts`
- Create: `src/components/SkillList.vue`
- Create: `src/components/SkillCard.vue`
- Modify: `src/pages/SkillsPage.vue`

- [ ] **Step 1: 定义技能数据类型和内嵌清单**

```ts:src/data/skills.ts
export interface Skill {
  name: string
  description: string
  dir: string
}

export type SkillStatus = 'installed' | 'available' | 'local-only'

export interface SkillWithStatus extends Skill {
  status: SkillStatus
}

export const embeddedSkills: Skill[] = [
  { name: 'brainstorming', description: '把想法变成设计的协作对话工具', dir: 'brainstorming' },
  { name: 'concise-response-skill', description: '减少 token 浪费的精简回复', dir: 'concise-response-skill' },
  { name: 'exam-review-skill', description: '大学期末复习刷题工具', dir: 'exam-review-skill' },
  { name: 'frontend-design', description: '创建高质量前端界面', dir: 'frontend-design' },
  { name: 'web-access', description: '联网操作与网页抓取', dir: 'web-access' },
]
```

- [ ] **Step 2: 创建 SkillCard 组件**

```vue:src/components/SkillCard.vue
<script setup lang="ts">
import type { SkillWithStatus } from '../data/skills'

defineProps<{
  skill: SkillWithStatus
}>()

const emit = defineEmits<{
  import: [name: string]
}>()

const statusLabels: Record<string, string> = {
  installed: '已安装',
  available: '可导入',
  'local-only': '仅本地',
}

const statusColors: Record<string, string> = {
  installed: 'bg-green-100 text-green-700',
  available: 'bg-blue-100 text-blue-700',
  'local-only': 'bg-gray-100 text-gray-500',
}
</script>

<template>
  <div class="flex items-center gap-4 bg-white rounded-lg border border-gray-200 p-4 hover:border-gray-300 transition-colors">
    <div class="flex-1 min-w-0">
      <div class="font-medium text-gray-900">{{ skill.name }}</div>
      <div class="text-sm text-gray-500 truncate">{{ skill.description }}</div>
    </div>
    <span
      class="shrink-0 px-2.5 py-0.5 rounded-full text-xs font-medium"
      :class="statusColors[skill.status]"
    >
      {{ statusLabels[skill.status] }}
    </span>
    <button
      v-if="skill.status === 'available'"
      class="shrink-0 px-3 py-1.5 text-sm rounded-lg bg-sky-500 text-white hover:bg-sky-600 transition-colors"
      @click="emit('import', skill.name)"
    >
      导入
    </button>
  </div>
</template>
```

- [ ] **Step 3: 创建 SkillList 组件**

```vue:src/components/SkillList.vue
<script setup lang="ts">
import { ref, computed } from 'vue'
import type { SkillWithStatus } from '../data/skills'
import { embeddedSkills } from '../data/skills'
import SkillCard from './SkillCard.vue'

const props = defineProps<{
  localSkills: string[]
}>()

const filter = ref<'all' | 'installed' | 'available'>('all')

const skills = computed<SkillWithStatus[]>(() => {
  const all: SkillWithStatus[] = embeddedSkills.map(s => ({
    ...s,
    status: props.localSkills.includes(s.name) ? 'installed' : 'available',
  }))
  return filter.value === 'all'
    ? all
    : all.filter(s => s.status === filter.value)
})

const emit = defineEmits<{
  import: [name: string]
}>()
</script>

<template>
  <div>
    <div class="flex items-center gap-2 mb-4">
      <button
        v-for="f in ([{ k: 'all', l: '全部' }, { k: 'installed', l: '已安装' }, { k: 'available', l: '可导入' }] as const)"
        :key="f.k"
        class="px-3 py-1.5 rounded-lg text-sm transition-colors"
        :class="filter === f.k ? 'bg-slate-800 text-white' : 'bg-gray-100 text-gray-600 hover:bg-gray-200'"
        @click="filter = f.k"
      >
        {{ f.l }}
      </button>
    </div>

    <div class="space-y-2">
      <SkillCard
        v-for="skill in skills"
        :key="skill.name"
        :skill="skill"
        @import="emit('import', $event)"
      />
    </div>

    <p v-if="skills.length === 0" class="text-gray-400 text-center py-8">
      没有匹配的技能
    </p>
  </div>
</template>
```

- [ ] **Step 4: 更新 SkillsPage**

```vue:src/pages/SkillsPage.vue
<script setup lang="ts">
import { ref } from 'vue'
import SkillList from '../components/SkillList.vue'

// mock: 假设本地已安装这些技能
const localSkills = ref<string[]>(['brainstorming', 'web-access'])

function handleImport(name: string) {
  if (!localSkills.value.includes(name)) {
    localSkills.value.push(name)
  }
}
</script>

<template>
  <div>
    <h2 class="text-2xl font-bold mb-1">技能管理</h2>
    <p class="text-gray-500 text-sm mb-6">
      检测本地技能 + 从内嵌清单选择性导入
    </p>
    <SkillList :local-skills="localSkills" @import="handleImport" />
  </div>
</template>
```

---

### Task 6: 内嵌 Commands 数据 + 命令列表

**Files:**
- Create: `src/data/commands.ts`
- Create: `src/components/CommandList.vue`
- Create: `src/components/CommandCard.vue`
- Create: `src/components/CommandDetail.vue`
- Modify: `src/pages/CommandsPage.vue`

- [ ] **Step 1: 定义命令数据类型和内嵌清单**

```ts:src/data/commands.ts
export interface Command {
  name: string
  description: string
  content: string
}

export type CommandStatus = 'installed' | 'available' | 'local-only'

export interface CommandWithStatus extends Command {
  status: CommandStatus
}

export const embeddedCommands: Command[] = [
  {
    name: 'see.md',
    description: '分析图片（截图/拖拽/URL 均可，自动收集所有图片走视觉模型分析）',
    content: `---
description: 分析图片（截图/拖拽/URL 均可，自动收集所有图片走视觉模型分析）
---

## 用途

用户想让你分析图片。**不要切换模型**，在当前模型下按以下流程处理：

## 处理流程

1. 收集所有图片来源
2. 调用视觉模型分析
3. 汇总结果回复用户`,
  },
]
```

- [ ] **Step 2: 创建 CommandCard 组件**

```vue:src/components/CommandCard.vue
<script setup lang="ts">
import type { CommandWithStatus } from '../data/commands'

defineProps<{
  command: CommandWithStatus
}>()

const emit = defineEmits<{
  import: [name: string]
  view: [name: string]
}>()

const statusLabels: Record<string, string> = {
  installed: '已安装',
  available: '可导入',
  'local-only': '仅本地',
}

const statusColors: Record<string, string> = {
  installed: 'bg-green-100 text-green-700',
  available: 'bg-blue-100 text-blue-700',
  'local-only': 'bg-gray-100 text-gray-500',
}
</script>

<template>
  <div class="flex items-center gap-4 bg-white rounded-lg border border-gray-200 p-4 hover:border-gray-300 transition-colors">
    <div class="flex-1 min-w-0">
      <div class="flex items-center gap-2">
        <code class="text-sm font-mono bg-gray-100 rounded px-1.5 py-0.5">{{ command.name }}</code>
      </div>
      <div class="text-sm text-gray-500 mt-1 truncate">{{ command.description }}</div>
    </div>
    <button
      class="shrink-0 px-3 py-1.5 text-sm rounded-lg text-sky-600 hover:bg-sky-50 transition-colors"
      @click="emit('view', command.name)"
    >
      查看
    </button>
    <span
      class="shrink-0 px-2.5 py-0.5 rounded-full text-xs font-medium"
      :class="statusColors[command.status]"
    >
      {{ statusLabels[command.status] }}
    </span>
    <button
      v-if="command.status === 'available'"
      class="shrink-0 px-3 py-1.5 text-sm rounded-lg bg-amber-500 text-white hover:bg-amber-600 transition-colors"
      @click="emit('import', command.name)"
    >
      导入
    </button>
  </div>
</template>
```

- [ ] **Step 3: 创建 CommandDetail 组件**

```vue:src/components/CommandDetail.vue
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
    <button
      class="text-sm text-sky-600 hover:text-sky-700 mb-4 flex items-center gap-1"
      @click="emit('back')"
    >
      &larr; 返回列表
    </button>

    <div class="bg-white rounded-lg border border-gray-200 p-6">
      <div class="flex items-center gap-2 mb-4">
        <code class="text-lg font-mono bg-gray-100 rounded px-2 py-0.5">{{ command.name }}</code>
      </div>
      <p class="text-gray-600 mb-6">{{ command.description }}</p>

      <div class="border-t pt-4">
        <MarkdownRenderer :content="command.content" />
      </div>
    </div>
  </div>
</template>
```

- [ ] **Step 4: 创建 CommandList 组件**

```vue:src/components/CommandList.vue
<script setup lang="ts">
import { ref, computed } from 'vue'
import type { CommandWithStatus } from '../data/commands'
import { embeddedCommands } from '../data/commands'
import CommandCard from './CommandCard.vue'
import CommandDetail from './CommandDetail.vue'

const props = defineProps<{
  localCommands: string[]
}>()

const selected = ref<string | null>(null)
const filter = ref<'all' | 'installed' | 'available'>('all')

const commands = computed<CommandWithStatus[]>(() => {
  const all: CommandWithStatus[] = embeddedCommands.map(c => ({
    ...c,
    status: props.localCommands.includes(c.name) ? 'installed' : 'available',
  }))
  return filter.value === 'all'
    ? all
    : all.filter(c => c.status === filter.value)
})

const selectedCommand = computed(() =>
  embeddedCommands.find(c => c.name === selected.value)
)

const emit = defineEmits<{
  import: [name: string]
}>()
</script>

<template>
  <div>
    <div v-if="selectedCommand">
      <CommandDetail :command="selectedCommand" @back="selected = null" />
    </div>
    <div v-else>
      <div class="flex items-center gap-2 mb-4">
        <button
          v-for="f in ([{ k: 'all', l: '全部' }, { k: 'installed', l: '已安装' }, { k: 'available', l: '可导入' }] as const)"
          :key="f.k"
          class="px-3 py-1.5 rounded-lg text-sm transition-colors"
          :class="filter === f.k ? 'bg-slate-800 text-white' : 'bg-gray-100 text-gray-600 hover:bg-gray-200'"
          @click="filter = f.k"
        >
          {{ f.l }}
        </button>
      </div>

      <div class="space-y-2">
        <CommandCard
          v-for="cmd in commands"
          :key="cmd.name"
          :command="cmd"
          @view="selected = $event"
          @import="emit('import', $event)"
        />
      </div>

      <p v-if="commands.length === 0" class="text-gray-400 text-center py-8">
        没有匹配的命令
      </p>
    </div>
  </div>
</template>
```

- [ ] **Step 5: 更新 CommandsPage**

```vue:src/pages/CommandsPage.vue
<script setup lang="ts">
import { ref } from 'vue'
import CommandList from '../components/CommandList.vue'

const localCommands = ref<string[]>([])

function handleImport(name: string) {
  if (!localCommands.value.includes(name)) {
    localCommands.value.push(name)
  }
}
</script>

<template>
  <div>
    <h2 class="text-2xl font-bold mb-1">命令管理</h2>
    <p class="text-gray-500 text-sm mb-6">
      检测本地命令 + 从内嵌清单选择性导入
    </p>
    <CommandList :local-commands="localCommands" @import="handleImport" />
  </div>
</template>
```

---

### Task 7: useScanner + useImport composables

**Files:**
- Create: `src/composables/useScanner.ts`
- Create: `src/composables/useImport.ts`
- Modify: `src/pages/SkillsPage.vue`
- Modify: `src/pages/CommandsPage.vue`

- [ ] **Step 1: 创建 useScanner composable**

```ts:src/composables/useScanner.ts
import { ref } from 'vue'

export interface ScanResult {
  installed: string[]
}

export function useScanner() {
  const scanning = ref(false)
  const result = ref<ScanResult>({ installed: [] })

  async function scanSkills(): Promise<ScanResult> {
    scanning.value = true
    // Phase 1: mock — 返回空列表
    // Phase 2 (Tauri): 读取 ~/.opencode/skills/ 目录
    await new Promise(r => setTimeout(r, 300))
    result.value = { installed: [] }
    scanning.value = false
    return result.value
  }

  async function scanCommands(): Promise<ScanResult> {
    scanning.value = true
    await new Promise(r => setTimeout(r, 300))
    result.value = { installed: [] }
    scanning.value = false
    return result.value
  }

  return { scanning, result, scanSkills, scanCommands }
}
```

- [ ] **Step 2: 创建 useImport composable**

```ts:src/composables/useImport.ts
import { ref } from 'vue'

export function useImport() {
  const importing = ref(false)

  async function importSkill(name: string): Promise<boolean> {
    importing.value = true
    // Phase 1: mock — 直接成功
    // Phase 2 (Tauri): 复制内嵌资源到 ~/.opencode/skills/{name}/
    await new Promise(r => setTimeout(r, 500))
    importing.value = false
    return true
  }

  async function importCommand(name: string): Promise<boolean> {
    importing.value = true
    await new Promise(r => setTimeout(r, 500))
    importing.value = false
    return true
  }

  return { importing, importSkill, importCommand }
}
```

- [ ] **Step 3: 更新 SkillsPage 集成 useScanner + useImport**

```vue:src/pages/SkillsPage.vue
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import SkillList from '../components/SkillList.vue'
import { useScanner } from '../composables/useScanner'
import { useImport } from '../composables/useImport'

const { scanning, scanSkills } = useScanner()
const { importing, importSkill } = useImport()
const localSkills = ref<string[]>([])

onMounted(async () => {
  const result = await scanSkills()
  localSkills.value = result.installed
})

async function handleImport(name: string) {
  const ok = await importSkill(name)
  if (ok && !localSkills.value.includes(name)) {
    localSkills.value.push(name)
  }
}
</script>

<template>
  <div>
    <h2 class="text-2xl font-bold mb-1">技能管理</h2>
    <p class="text-gray-500 text-sm mb-6">
      检测本地技能 + 从内嵌清单选择性导入
    </p>
    <p v-if="scanning" class="text-gray-400">扫描中...</p>
    <SkillList v-else :local-skills="localSkills" @import="handleImport" />
  </div>
</template>
```

- [ ] **Step 4: 更新 CommandsPage 集成 useScanner + useImport**

```vue:src/pages/CommandsPage.vue
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import CommandList from '../components/CommandList.vue'
import { useScanner } from '../composables/useScanner'
import { useImport } from '../composables/useImport'

const { scanning, scanCommands } = useScanner()
const { importing, importCommand } = useImport()
const localCommands = ref<string[]>([])

onMounted(async () => {
  const result = await scanCommands()
  localCommands.value = result.installed
})

async function handleImport(name: string) {
  const ok = await importCommand(name)
  if (ok && !localCommands.value.includes(name)) {
    localCommands.value.push(name)
  }
}
</script>

<template>
  <div>
    <h2 class="text-2xl font-bold mb-1">命令管理</h2>
    <p class="text-gray-500 text-sm mb-6">
      检测本地命令 + 从内嵌清单选择性导入
    </p>
    <p v-if="scanning" class="text-gray-400">扫描中...</p>
    <CommandList v-else :local-commands="localCommands" @import="handleImport" />
  </div>
</template>
```

---

### Task 8: 全局搜索

**Files:**
- Create: `src/composables/useSearch.ts`
- Create: `src/components/SearchResults.vue`
- Modify: `src/components/Sidebar.vue`
- Modify: `src/App.vue`

- [ ] **Step 1: 创建 useSearch composable**

```ts:src/composables/useSearch.ts
import { ref, computed } from 'vue'
import { embeddedSkills } from '../data/skills'
import { embeddedCommands } from '../data/commands'
import { chapters } from '../content'

export interface SearchResult {
  type: 'teaching' | 'skill' | 'command'
  title: string
  description: string
  route: string
}

export function useSearch() {
  const query = ref('')
  const results = computed<SearchResult[]>(() => {
    const q = query.value.toLowerCase().trim()
    if (!q) return []

    const hits: SearchResult[] = []

    for (const ch of chapters) {
      if (ch.title.includes(q)) {
        hits.push({ type: 'teaching', title: ch.title, description: '教学文档', route: `/teaching/${ch.slug}` })
      }
    }

    for (const sk of embeddedSkills) {
      if (sk.name.includes(q) || sk.description.includes(q)) {
        hits.push({ type: 'skill', title: sk.name, description: sk.description, route: '/skills' })
      }
    }

    for (const cmd of embeddedCommands) {
      if (cmd.name.includes(q) || cmd.description.includes(q)) {
        hits.push({ type: 'command', title: cmd.name, description: cmd.description, route: '/commands' })
      }
    }

    return hits.slice(0, 20)
  })

  return { query, results }
}
```

- [ ] **Step 2: 创建 SearchResults 组件**

```vue:src/components/SearchResults.vue
<script setup lang="ts">
import type { SearchResult } from '../composables/useSearch'
import { useRouter } from 'vue-router'

const props = defineProps<{
  results: SearchResult[]
  query: string
}>()

const router = useRouter()

function go(route: string) {
  router.push(route)
}

const typeLabels: Record<string, string> = {
  teaching: '教学',
  skill: '技能',
  command: '命令',
}

const typeColors: Record<string, string> = {
  teaching: 'text-sky-600 bg-sky-50',
  skill: 'text-emerald-600 bg-emerald-50',
  command: 'text-amber-600 bg-amber-50',
}
</script>

<template>
  <div v-if="query" class="absolute top-full left-3 right-3 mt-1 bg-white rounded-lg shadow-lg border border-gray-200 max-h-80 overflow-y-auto z-50">
    <div v-if="results.length === 0" class="p-4 text-sm text-gray-400 text-center">
      没有搜索结果
    </div>
    <div v-else class="py-1">
      <div
        v-for="r in results"
        :key="r.route + r.title"
        class="px-4 py-2 hover:bg-gray-50 cursor-pointer flex items-center gap-3"
        @click="go(r.route)"
      >
        <span class="text-xs px-1.5 py-0.5 rounded font-medium shrink-0" :class="typeColors[r.type]">
          {{ typeLabels[r.type] }}
        </span>
        <div class="min-w-0 flex-1">
          <div class="text-sm font-medium text-gray-900">{{ r.title }}</div>
          <div class="text-xs text-gray-500 truncate">{{ r.description }}</div>
        </div>
      </div>
    </div>
  </div>
</template>
```

- [ ] **Step 3: 更新 Sidebar 集成搜索**

```vue:src/components/Sidebar.vue
<script setup lang="ts">
import { useRoute, useRouter } from 'vue-router'
import { ref } from 'vue'
import { useSearch } from '../composables/useSearch'
import SearchResults from './SearchResults.vue'

const route = useRoute()
const router = useRouter()
const { query, results } = useSearch()
const showResults = ref(false)

const navItems = [
  { label: '教学文档', icon: '📖', path: '/teaching' },
  { label: '技能管理', icon: '🧰', path: '/skills' },
  { label: '命令管理', icon: '⚡', path: '/commands' },
]

const teachingChapters = [
  { label: '基础介绍', slug: 'intro' },
  { label: '快速上手', slug: 'quickstart' },
  { label: '使用 Skills', slug: 'skills-guide' },
  { label: 'Commands 定制', slug: 'commands-guide' },
]

function isActive(path: string) {
  return route.path.startsWith(path)
}

function navigate(path: string) {
  showResults.value = false
  router.push(path)
}
</script>

<template>
  <aside class="w-64 bg-slate-800 text-white flex flex-col h-full relative">
    <div class="p-4 border-b border-slate-700">
      <h1 class="text-lg font-bold text-sky-400">OpenCode 教学</h1>
    </div>

    <nav class="flex-1 overflow-y-auto p-3 space-y-1">
      <div class="mb-4">
        <div
          class="flex items-center gap-2 px-3 py-2 rounded-lg cursor-pointer hover:bg-slate-700 font-medium text-sky-400"
          @click="navigate('/teaching')"
        >
          <span>📖</span>
          <span>教学文档</span>
        </div>
        <div class="ml-4 mt-1 space-y-1">
          <div
            v-for="ch in teachingChapters"
            :key="ch.slug"
            class="px-3 py-1.5 rounded-md cursor-pointer text-sm text-slate-300 hover:bg-slate-700 hover:text-white transition-colors"
            :class="{ 'bg-slate-700 text-white': route.params.slug === ch.slug }"
            @click="navigate(`/teaching/${ch.slug}`)"
          >
            {{ ch.label }}
          </div>
        </div>
      </div>

      <div
        class="flex items-center gap-2 px-3 py-2 rounded-lg cursor-pointer hover:bg-slate-700 font-medium"
        :class="{ 'text-emerald-400': isActive('/skills'), 'text-slate-300': !isActive('/skills') }"
        @click="navigate('/skills')"
      >
        <span>🧰</span>
        <span>技能管理</span>
      </div>

      <div
        class="flex items-center gap-2 px-3 py-2 rounded-lg cursor-pointer hover:bg-slate-700 font-medium"
        :class="{ 'text-amber-400': isActive('/commands'), 'text-slate-300': !isActive('/commands') }"
        @click="navigate('/commands')"
      >
        <span>⚡</span>
        <span>命令管理</span>
      </div>
    </nav>

    <div class="p-3 border-t border-slate-700 relative">
      <div class="relative">
        <input
          v-model="query"
          type="text"
          placeholder="搜索..."
          class="w-full bg-slate-700 text-white rounded-lg px-3 py-2 text-sm placeholder-slate-400 outline-none focus:ring-2 focus:ring-sky-500"
          @focus="showResults = true"
          @blur="setTimeout(() => showResults = false, 200)"
        />
      </div>
      <SearchResults v-if="showResults" :results="results" :query="query" />
    </div>
  </aside>
</template>
```

---

### Task 9: 构建验证与最终修复

**Files:**
- No new files

- [ ] **Step 1: TypeScript 类型检查**

```bash
cd Desktop/opencode-tutorial && npx vue-tsc -b --noEmit
```

修复所有类型错误（预期可能有未使用的变量等，根据错误信息调整 `noUnusedLocals` 等配置或修复代码）。

- [ ] **Step 2: 构建生产版本**

```bash
cd Desktop/opencode-tutorial && npm run build
```

Expected: 构建成功，输出到 `dist/` 目录，无错误。

- [ ] **Step 3: 手动验证核心功能**

- 侧边栏导航：切换三个板块，子章节展开/收起
- 教学文档：点击章节，Markdown 渲染正确
- 技能列表：筛选器切换正常，点击"导入"状态变更为"已安装"
- 命令列表：筛选器切换正常，点击"查看"展示详情，导入正常
- 搜索：输入关键词，展示结果，点击跳转
