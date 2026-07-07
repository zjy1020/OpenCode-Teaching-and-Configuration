<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '../lib/tauri'

interface ThemeDefs {
  [key: string]: string
}

interface ThemeColors {
  [key: string]: string | { dark: string; light: string }
}

interface ThemeJson {
  defs?: ThemeDefs
  theme: ThemeColors
}

const theme = ref<Record<string, string>>({})
const loading = ref(true)
const error = ref('')

function resolveColorValue(val: string | { dark: string; light: string }, defs: ThemeDefs): string {
  if (typeof val === 'string') {
    if (val.startsWith('#')) return val
    return defs[val] || val
  }
  return resolveColorValue(val.light, defs)
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
    <div v-else class="preview-terminal" :style="{ background: theme.background, color: theme.text, borderColor: theme.border }">

      <div class="term-titlebar" :style="{ background: theme.backgroundElement, borderBottomColor: theme.border, color: theme.textMuted }">
        <span class="titlebar-dot" style="background:#ff5f57"></span>
        <span class="titlebar-dot" style="background:#febc2e"></span>
        <span class="titlebar-dot" style="background:#28c840"></span>
        <span class="titlebar-label">OpenCode TUI</span>
      </div>

      <div class="term-body">

        <div class="preview-section">
          <div class="md-heading" :style="{ color: theme.markdownHeading }">## Markdown 预览</div>
          <p class="md-text" :style="{ color: theme.markdownText }">
            这是一段普通文本。这是 <span class="md-strong" :style="{ color: theme.markdownStrong }">加粗文字</span>，
            这是 <span class="md-emph" :style="{ color: theme.markdownEmph }">斜体文字</span>。
            行内 <span class="md-code" :style="{ color: theme.markdownCode, background: theme.backgroundPanel }">code</span> 示例。
          </p>
        </div>

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

        <div class="preview-section">
          <p class="diff-label" :style="{ color: theme.textMuted }">Diff 预览:</p>
          <div class="diff-added" :style="{ background: theme.diffAddedBg, color: theme.diffAdded }">+ function greet(name) {</div>
          <div class="diff-removed" :style="{ background: theme.diffRemovedBg, color: theme.diffRemoved }">- function greet() {</div>
        </div>

        <div class="preview-section">
          <div class="mode-badge" :style="{ background: theme.primary, color: '#FFFFFF' }">
            <span class="mode-icon">&#x25B6;</span> Build
          </div>
          <div class="mode-badge" :style="{ background: theme.secondary, color: '#FFFFFF' }">
            <span class="mode-icon">&#x25B6;</span> Plan
          </div>
        </div>

        <div class="preview-section" style="margin-bottom:0">
          <span class="selection-demo" :style="{ background: theme.selection, color: theme.selectionForeground }">
            选中文字效果 - 蓝底白字，清晰可辨
          </span>
        </div>

      </div>
    </div>
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
  padding: 2px 6px;
  border-radius: 3px;
  font-size: 0.76rem;
}
</style>
