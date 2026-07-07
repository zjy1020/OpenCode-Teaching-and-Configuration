<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '../lib/tauri'

interface ThemeDefs { [key: string]: string }
interface ThemeColors { [key: string]: string | { dark: string; light: string } }
interface ThemeJson { defs?: ThemeDefs; theme: ThemeColors }

type TabId = 'overview' | 'markdown' | 'code' | 'diff' | 'selection' | 'mode'

const theme = ref<Record<string, string>>({})
const loading = ref(true)
const error = ref('')
const activeTab = ref<TabId>('overview')
const copied = ref(false)

const tabs: { id: TabId; label: string }[] = [
  { id: 'overview', label: '总览' },
  { id: 'markdown', label: 'Markdown' },
  { id: 'code', label: '代码块' },
  { id: 'diff', label: 'Diff' },
  { id: 'selection', label: '选中' },
  { id: 'mode', label: '模式' },
]

const codeSnippets: Record<TabId, { label: string; key: string }[]> = {
  overview: [
    { label: '主色', key: 'primary, secondary, accent' },
    { label: '文字', key: 'text, textMuted' },
    { label: '背景', key: 'background, backgroundPanel, backgroundElement' },
    { label: '边框', key: 'border, borderActive, borderSubtle' },
    { label: '语法', key: 'syntaxKeyword, syntaxFunction, syntaxString, syntaxVariable, syntaxComment, syntaxNumber, syntaxType, syntaxOperator' },
    { label: 'Markdown', key: 'markdownText, markdownHeading, markdownLink, markdownCode, markdownStrong' },
    { label: 'Diff', key: 'diffAdded, diffRemoved, diffAddedBg, diffRemovedBg' },
    { label: '选中', key: 'selection, selectionForeground' },
  ],
  markdown: [
    { label: '普通文本', key: 'markdownText' },
    { label: '标题', key: 'markdownHeading' },
    { label: '链接', key: 'markdownLink, markdownLinkText' },
    { label: '行内代码', key: 'markdownCode' },
    { label: '加粗', key: 'markdownStrong' },
    { label: '斜体', key: 'markdownEmph' },
    { label: '引用', key: 'markdownBlockQuote' },
    { label: '列表', key: 'markdownListItem, markdownListEnumeration' },
    { label: '代码块', key: 'markdownCodeBlock' },
    { label: '分割线', key: 'markdownHorizontalRule' },
  ],
  code: [
    { label: '关键字', key: 'syntaxKeyword' },
    { label: '函数名', key: 'syntaxFunction' },
    { label: '变量', key: 'syntaxVariable' },
    { label: '字符串', key: 'syntaxString' },
    { label: '数字', key: 'syntaxNumber' },
    { label: '类型', key: 'syntaxType' },
    { label: '运算符', key: 'syntaxOperator' },
    { label: '注释', key: 'syntaxComment' },
    { label: '标点', key: 'syntaxPunctuation' },
  ],
  diff: [
    { label: '新增文字', key: 'diffAdded' },
    { label: '删除文字', key: 'diffRemoved' },
    { label: '新增背景', key: 'diffAddedBg' },
    { label: '删除背景', key: 'diffRemovedBg' },
    { label: '上下文', key: 'diffContext, diffContextBg' },
    { label: '高亮新增', key: 'diffHighlightAdded' },
    { label: '高亮删除', key: 'diffHighlightRemoved' },
    { label: '行号', key: 'diffLineNumber' },
    { label: 'Hunk 头部', key: 'diffHunkHeader' },
  ],
  selection: [
    { label: '选中背景', key: 'selection' },
    { label: '选中文字', key: 'selectionForeground' },
    { label: '列表选中文字', key: 'selectedListItemText' },
  ],
  mode: [
    { label: 'Build 模式 (primary)', key: 'primary' },
    { label: 'Plan 模式 (secondary)', key: 'secondary' },
    { label: '强调色 (accent)', key: 'accent' },
    { label: '信息色 (info)', key: 'info' },
  ],
}

const activeSnippet = computed(() => codeSnippets[activeTab.value])

function resolveColorValue(val: string | { dark: string; light: string }, defs: ThemeDefs): string {
  if (typeof val === 'string') {
    if (val.startsWith('#')) return val
    return defs[val] || val
  }
  return resolveColorValue(val.light, defs)
}

function snippetJson(): string {
  const keys: string[] = []
  for (const group of activeSnippet.value) {
    for (const k of group.key.replace(/\s/g, '').split(',')) {
      if (theme.value[k]) keys.push(k)
    }
  }
  const obj: Record<string, string> = {}
  for (const k of [...new Set(keys)]) {
    obj[k] = theme.value[k] || ''
  }
  return JSON.stringify(obj, null, 2)
}

async function copyCode() {
  try {
    await navigator.clipboard.writeText(snippetJson())
    copied.value = true
    setTimeout(() => { copied.value = false }, 1500)
  } catch {}
}

onMounted(async () => {
  try {
    const raw = await invoke<string>('read_theme_content', { name: 'ember-glow' })
    const json: ThemeJson = JSON.parse(raw)
    const defs = json.defs || {}
    const resolved: Record<string, string> = {}
    for (const [key, value] of Object.entries(json.theme)) {
      resolved[key] = resolveColorValue(value, defs)
    }
    theme.value = resolved
  } catch (e: any) {
    error.value = String(e)
  } finally {
    loading.value = false
  }
})
</script>

<template>
  <div class="theme-preview">
    <h3 class="preview-heading">主题预览</h3>

    <div v-if="loading" class="preview-status">加载中...</div>
    <div v-else-if="error" class="preview-status error">{{ error }}</div>
    <template v-else>

      <div class="tab-bar">
        <button
          v-for="tab in tabs"
          :key="tab.id"
          class="tab-btn"
          :class="{ active: activeTab === tab.id }"
          @click="activeTab = tab.id"
        >{{ tab.label }}</button>
      </div>

      <div class="preview-terminal" :style="{ background: theme.background, color: theme.text, borderColor: theme.border }">

        <div class="term-titlebar" :style="{ background: theme.backgroundElement, borderBottomColor: theme.border, color: theme.textMuted }">
          <span class="titlebar-dot" style="background:#ff5f57"></span>
          <span class="titlebar-dot" style="background:#febc2e"></span>
          <span class="titlebar-dot" style="background:#28c840"></span>
          <span class="titlebar-label">OpenCode TUI</span>
        </div>

        <div class="term-body">

          <template v-if="activeTab === 'overview' || activeTab === 'markdown'">
            <div class="preview-section">
              <div class="md-heading" :style="{ color: theme.markdownHeading }">## Markdown 预览</div>
              <p class="md-text" :style="{ color: theme.markdownText }">
                这是一段普通文本。
                <span class="md-strong" :style="{ color: theme.markdownStrong }">加粗文字</span>，
                <span class="md-emph" :style="{ color: theme.markdownEmph }">斜体文字</span>。
                行内 <span class="md-code" :style="{ color: theme.markdownCode, background: theme.backgroundPanel }">code</span>。
                <span :style="{ color: theme.markdownLink }">链接</span>
              </p>
            </div>
          </template>

          <template v-if="activeTab === 'overview' || activeTab === 'code'">
            <div class="preview-section">
              <div class="code-block" :style="{ background: theme.backgroundPanel, borderColor: theme.borderSubtle }">
                <div class="code-line">
                  <span class="ln" :style="{ color: theme.diffLineNumber }">1</span>
                  <span :style="{ color: theme.syntaxKeyword }">function</span>
                  <span :style="{ color: theme.syntaxFunction }">greet</span>
                  <span :style="{ color: theme.syntaxPunctuation }">(</span>
                  <span :style="{ color: theme.syntaxVariable }">name</span>
                  <span :style="{ color: theme.syntaxPunctuation }">)</span>
                  <span :style="{ color: theme.syntaxPunctuation }"> {</span>
                </div>
                <div class="code-line">
                  <span class="ln" :style="{ color: theme.diffLineNumber }">2</span>
                  <span :style="{ color: theme.syntaxKeyword }">const</span>
                  <span :style="{ color: theme.syntaxVariable }">msg</span>
                  <span :style="{ color: theme.syntaxOperator }">=</span>
                  <span :style="{ color: theme.syntaxString }">`Hello, <span :style="{ color: theme.syntaxVariable }">${name}</span>!`</span>
                </div>
                <div class="code-line">
                  <span class="ln" :style="{ color: theme.diffLineNumber }">3</span>
                  <span :style="{ color: theme.syntaxKeyword }">return</span>
                  <span :style="{ color: theme.syntaxVariable }">msg</span>
                </div>
                <div class="code-line">
                  <span class="ln" :style="{ color: theme.diffLineNumber }">4</span>
                  <span :style="{ color: theme.syntaxPunctuation }">}</span>
                </div>
              </div>
            </div>
          </template>

          <template v-if="activeTab === 'overview' || activeTab === 'diff'">
            <div class="preview-section">
              <p class="diff-label" :style="{ color: theme.textMuted }">Diff 预览</p>
              <div class="diff-added" :style="{ background: theme.diffAddedBg, color: theme.diffAdded }">+ function greet(name) {</div>
              <div class="diff-removed" :style="{ background: theme.diffRemovedBg, color: theme.diffRemoved }">- function greet() {</div>
              <div class="diff-added" :style="{ background: theme.diffAddedBg, color: theme.diffAdded }">+   const msg = `Hello!`</div>
              <div class="diff-removed" :style="{ background: theme.diffRemovedBg, color: theme.diffRemoved }">-   return null</div>
            </div>
          </template>

          <template v-if="activeTab === 'overview' || activeTab === 'selection'">
            <div class="preview-section">
              <p class="diff-label" :style="{ color: theme.textMuted }">拖拽选中效果</p>
              <div class="selection-demo" :style="{ background: theme.selection, color: theme.selectionForeground }">
                选中文字效果 - 清晰可辨
              </div>
            </div>
          </template>

          <template v-if="activeTab === 'overview' || activeTab === 'mode'">
            <div class="preview-section" :style="{ marginBottom: activeTab === 'mode' ? 0 : undefined }">
              <p class="diff-label" :style="{ color: theme.textMuted }">模式标识</p>
              <div class="mode-badge" :style="{ background: theme.primary, color: '#FFFFFF' }">
                <span class="mode-icon">&#x25B6;</span> Build
              </div>
              <div class="mode-badge" :style="{ background: theme.secondary, color: '#FFFFFF' }">
                <span class="mode-icon">&#x25B6;</span> Plan
              </div>
            </div>
          </template>

        </div>
      </div>

      <div class="code-panel">
        <div class="code-panel-header">
          <span class="code-panel-title">主题配置代码</span>
          <button class="copy-btn" :class="{ copied }" @click="copyCode">
            <span v-if="copied">已复制</span>
            <span v-else>复制</span>
          </button>
        </div>
        <pre class="code-panel-body"><code>{{ snippetJson() }}</code></pre>
      </div>

    </template>
  </div>
</template>

<style scoped>
.theme-preview {
  margin-top: 2rem;
}
.preview-heading {
  font-family: 'Fredoka', sans-serif;
  font-size: 1.1rem;
  font-weight: 600;
  color: #78350f;
  margin: 0 0 0.75rem 0;
}
.preview-status {
  font-family: 'Nunito', sans-serif;
  font-size: 0.85rem;
  color: #92400e;
}
.preview-status.error {
  color: #dc2626;
}
.tab-bar {
  display: flex;
  gap: 4px;
  margin-bottom: 0.75rem;
  flex-wrap: wrap;
}
.tab-btn {
  padding: 5px 14px;
  border-radius: 20px;
  border: 2px solid #d97706;
  background: rgba(255, 255, 255, 0.5);
  font-family: 'Nunito', sans-serif;
  font-size: 0.78rem;
  font-weight: 600;
  color: #92400e;
  cursor: pointer;
  transition: all 0.12s;
}
.tab-btn:hover {
  background: #fffbeb;
}
.tab-btn.active {
  background: #fbbf24;
  color: #78350f;
  border-color: #fbbf24;
}
.preview-terminal {
  border-radius: 12px;
  border: 2px solid;
  overflow: hidden;
  font-family: 'JetBrains Mono', 'Fira Code', 'Cascadia Code', monospace;
  font-size: 0.78rem;
  line-height: 1.6;
}
.term-titlebar {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 14px;
  border-bottom: 1px solid;
  font-family: 'Nunito', sans-serif;
  font-size: 0.72rem;
}
.titlebar-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
}
.titlebar-label {
  margin-left: auto;
}
.term-body {
  padding: 14px 16px;
}
.preview-section {
  margin-bottom: 12px;
}
.md-heading {
  font-weight: 700;
  font-size: 0.9rem;
  margin: 0 0 6px 0;
}
.md-text {
  margin: 0;
  line-height: 1.7;
}
.md-strong { font-weight: 700; }
.md-emph { font-style: italic; }
.md-code {
  padding: 1px 5px;
  border-radius: 4px;
  font-family: inherit;
  font-size: 0.76rem;
}
.code-block {
  border: 1px solid;
  border-radius: 8px;
  padding: 8px 0;
  font-size: 0.76rem;
  line-height: 1.8;
}
.code-line {
  padding: 0 12px;
  white-space: pre;
}
.ln {
  display: inline-block;
  width: 20px;
  text-align: right;
  margin-right: 14px;
  user-select: none;
}
.diff-label {
  font-family: 'Nunito', sans-serif;
  font-size: 0.78rem;
  margin: 0 0 4px 0;
}
.diff-added, .diff-removed {
  padding: 2px 10px;
  font-size: 0.76rem;
  border-radius: 4px;
  margin-bottom: 2px;
}
.mode-badge {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 3px 12px;
  border-radius: 20px;
  font-family: 'Nunito', sans-serif;
  font-size: 0.75rem;
  font-weight: 600;
  margin-right: 8px;
}
.mode-icon {
  font-size: 0.6rem;
}
.selection-demo {
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 0.76rem;
}
.code-panel {
  margin-top: 0.75rem;
  border-radius: 10px;
  border: 2px solid #d97706;
  overflow: hidden;
}
.code-panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 6px 12px;
  background: #fffbeb;
  border-bottom: 1px solid #e8d8c8;
}
.code-panel-title {
  font-family: 'Nunito', sans-serif;
  font-size: 0.75rem;
  font-weight: 600;
  color: #78350f;
}
.copy-btn {
  padding: 2px 10px;
  border-radius: 12px;
  border: 1px solid #d97706;
  background: #fff;
  font-family: 'Nunito', sans-serif;
  font-size: 0.7rem;
  font-weight: 600;
  color: #92400e;
  cursor: pointer;
}
.copy-btn:hover { background: #fbbf24; }
.copy-btn.copied {
  background: #059669;
  border-color: #059669;
  color: #fff;
}
.code-panel-body {
  margin: 0;
  padding: 10px 14px;
  background: #1e1e1e;
  color: #d4d4d4;
  font-family: 'JetBrains Mono', 'Fira Code', monospace;
  font-size: 0.72rem;
  line-height: 1.6;
  overflow-x: auto;
  max-height: 260px;
  overflow-y: auto;
}
</style>
