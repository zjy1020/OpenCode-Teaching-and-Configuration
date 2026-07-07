<script setup lang="ts">
import { computed, ref, onMounted, onUnmounted } from 'vue'
import { openUrl } from '../lib/tauri'
import MarkdownIt from 'markdown-it'
import anchor from 'markdown-it-anchor'

function slugify(s: string): string {
  return encodeURIComponent(
    s.trim().toLowerCase().replace(/\s+/g, '-')
  )
}

const md = new MarkdownIt({ html: true, linkify: true, breaks: true })
md.use(anchor, { permalink: false, level: [1, 2, 3], slugify })

md.renderer.rules.image = (tokens, idx) => {
  const token = tokens[idx]
  const src = md.utils.escapeHtml(token.attrGet('src') || '')
  const alt = md.utils.escapeHtml(token.content || '')
  return `<img src="${src}" alt="${alt}" class="img-clickable" onclick="document.getElementById('lightbox-img').src=this.src;document.getElementById('lightbox-overlay').classList.add('open')" />`
}

md.renderer.rules.fence = (tokens, idx) => {
  const token = tokens[idx]
  const raw = token.info.trim()
  const lang = raw ? `language-${raw}` : ''
  const label = raw || ''
  const code = md.utils.escapeHtml(token.content)
  const copyId = `copy-${idx}-${Date.now()}`
  return `<div class="code-block-wrapper">
    <div class="code-block-header">
      <span class="code-lang">${label}</span>
      <button class="copy-btn" onclick="
        navigator.clipboard.writeText(document.getElementById('${copyId}').textContent);
        this.textContent='已复制';
        setTimeout(()=>this.textContent='复制',2000);
      ">复制</button>
    </div>
    <pre class="${lang}"><code id="${copyId}">${code}</code></pre>
  </div>`
}

const props = defineProps<{
  content: string
}>()

const rendered = computed(() => md.render(props.content))
const bodyRef = ref<HTMLElement | null>(null)

function handleClick(e: MouseEvent) {
  const target = e.target as HTMLElement
  const link = target.closest('a')
  if (!link) return
  const href = link.getAttribute('href')
  if (!href || href.startsWith('#') || href.startsWith('/')) return
  e.preventDefault()
  openUrl(href)
}

onMounted(() => {
  bodyRef.value?.addEventListener('click', handleClick)
})
onUnmounted(() => {
  bodyRef.value?.removeEventListener('click', handleClick)
})
</script>

<template>
  <div ref="bodyRef" class="markdown-body" v-html="rendered" />
  <div id="lightbox-overlay" class="lightbox-overlay" onclick="this.classList.remove('open')">
    <span class="lightbox-close">&times;</span>
    <img id="lightbox-img" class="lightbox-img" src="" alt="" />
  </div>
</template>

<style>
.markdown-body h1 {
  font-family: 'Fredoka', sans-serif;
  font-size: 1.5rem;
  font-weight: 600;
  color: #7c2d12;
  margin-bottom: 1rem;
  margin-top: 0;
}
.markdown-body h2 {
  font-family: 'Fredoka', sans-serif;
  font-size: 1.2rem;
  font-weight: 600;
  color: #9a3412;
  margin-bottom: 0.75rem;
  margin-top: 1.25rem;
}
.markdown-body h3 {
  font-family: 'Fredoka', sans-serif;
  font-size: 1.05rem;
  font-weight: 500;
  color: #b45309;
  margin-bottom: 0.5rem;
  margin-top: 1rem;
}
.markdown-body p {
  margin-bottom: 0.75rem;
  line-height: 1.7;
  color: #5c3a0e;
}
.markdown-body strong {
  color: #78350f;
}
.markdown-body code {
  background: #f5e6d3;
  border-radius: 6px;
  padding: 0.15rem 0.5rem;
  font-size: 0.82em;
  font-family: 'Cascadia Code', 'Fira Code', 'JetBrains Mono', ui-monospace, monospace;
  color: #78350f;
}
.markdown-body pre {
  background: #1e1e2e;
  color: #cdd6f4;
  border-radius: 0;
  padding: 1rem 1.25rem;
  margin: 0;
  overflow-x: auto;
  font-size: 0.85rem;
  line-height: 1.6;
}
.markdown-body pre code {
  background: transparent;
  padding: 0;
  color: inherit;
  font-size: inherit;
  font-family: 'Cascadia Code', 'Fira Code', 'JetBrains Mono', ui-monospace, monospace;
}
.markdown-body ul, .markdown-body ol {
  padding-left: 1.5rem;
  margin-bottom: 0.75rem;
}
.markdown-body ul li, .markdown-body ol li {
  margin-bottom: 0.25rem;
  color: #5c3a0e;
  line-height: 1.6;
}
.markdown-body ul {
  list-style: disc;
}
.markdown-body ol {
  list-style: decimal;
}
.markdown-body blockquote {
  border-left: 4px solid #f59e0b;
  background: #fffbeb;
  padding: 0.5rem 1rem;
  border-radius: 0 10px 10px 0;
  margin: 0.75rem 0;
  color: #92400e;
  font-size: 0.9rem;
}
.markdown-body blockquote strong {
  color: #78350f;
}
.markdown-body a {
  color: #ca8a04;
  text-decoration: underline;
}
.markdown-body a:hover {
  color: #a16207;
}
.markdown-body img {
  max-width: 100%;
  border-radius: 12px;
  border: 2px solid #fde68a;
  margin: 0.75rem 0;
  display: block;
}
.markdown-body .img-clickable {
  cursor: zoom-in;
}

.lightbox-overlay {
  display: none;
  position: fixed;
  inset: 0;
  z-index: 9999;
  background: rgba(0,0,0,0.8);
  justify-content: center;
  align-items: center;
  cursor: zoom-out;
}
.lightbox-overlay.open {
  display: flex;
}
.lightbox-img {
  max-width: 90vw;
  max-height: 90vh;
  border-radius: 8px;
  box-shadow: 0 4px 30px rgba(0,0,0,0.5);
}
.lightbox-close {
  position: absolute;
  top: 1rem;
  right: 1.5rem;
  font-size: 2.5rem;
  color: #fff;
  cursor: pointer;
  line-height: 1;
}
.markdown-body .code-block-wrapper {
  margin-bottom: 1rem;
  border-radius: 12px;
  overflow: hidden;
  border: 1px solid #313244;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
}
.markdown-body .code-block-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: #181825;
  padding: 0.4rem 1rem;
  border-bottom: 1px solid #313244;
}
.markdown-body .code-lang {
  font-size: 0.72rem;
  font-family: 'Cascadia Code', 'Fira Code', 'JetBrains Mono', ui-monospace, monospace;
  color: #a6adc8;
  text-transform: lowercase;
}
.markdown-body .copy-btn {
  padding: 0.2rem 0.6rem;
  font-size: 0.72rem;
  font-family: 'Nunito', sans-serif;
  background: rgba(255,255,255,0.12);
  color: #d4d4d4;
  border: 1px solid rgba(255,255,255,0.15);
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.15s;
  z-index: 1;
}
.markdown-body .copy-btn:hover {
  background: rgba(255,255,255,0.25);
  color: #fff;
}
.markdown-body .code-block-wrapper pre {
  margin-bottom: 0;
}
.markdown-body table {
  width: 100%;
  border-collapse: collapse;
  margin: 0.75rem 0;
  font-size: 0.85rem;
}
.markdown-body th, .markdown-body td {
  border: 1px solid #fde68a;
  padding: 0.4rem 0.75rem;
  text-align: left;
  color: #5c3a0e;
}
.markdown-body th {
  background: #fef3c7;
  font-family: 'Fredoka', sans-serif;
  font-weight: 600;
}
.markdown-body tr:nth-child(even) td {
  background: #fffbeb;
}
</style>
