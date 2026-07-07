<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  content: string
}>()

interface TocItem {
  id: string
  text: string
  level: number
}

function slugify(s: string): string {
  return encodeURIComponent(
    s.trim().toLowerCase().replace(/\s+/g, '-')
  )
}

const headings = computed<TocItem[]>(() => {
  const items: TocItem[] = []
  const lines = props.content.split('\n')
  for (const line of lines) {
    const m = line.match(/^(#{1,3})\s+(.+)$/)
    if (m) {
      const level = m[1].length
      const text = m[2].replace(/[`*_~]/g, '')
      const id = slugify(text)
      items.push({ id, text, level })
    }
  }
  return items
})

function scrollTo(id: string) {
  const el = document.getElementById(id)
  if (el) el.scrollIntoView({ behavior: 'smooth', block: 'start' })
}
</script>

<template>
  <nav v-if="headings.length > 1" class="toc">
    <div class="toc-title">大纲</div>
    <div class="toc-list">
      <button
        v-for="h in headings"
        :key="h.id"
        class="toc-item"
        :class="'toc-level-' + h.level"
        @click="scrollTo(h.id)"
      >
        {{ h.text }}
      </button>
    </div>
  </nav>
</template>

<style scoped>
.toc {
  position: sticky;
  top: 1rem;
  width: 16rem;
  flex-shrink: 0;
  background: #fff;
  border-radius: 14px;
  padding: 1rem;
  border: 2px solid #fde68a;
  box-shadow: 6px 6px 12px rgba(146, 64, 14, 0.06), -3px -3px 8px rgba(255, 255, 255, 0.4);
}

.toc-title {
  font-family: 'Fredoka', sans-serif;
  font-size: 1rem;
  font-weight: 600;
  color: #78350f;
  margin-bottom: 0.75rem;
  padding-bottom: 0.5rem;
  border-bottom: 2px solid #fbbf24;
}

.toc-list {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.toc-item {
  text-align: left;
  background: none;
  border: none;
  cursor: pointer;
  font-family: 'Nunito', sans-serif;
  color: #92400e;
  padding: 0.35rem 0.6rem;
  border-radius: 8px;
  transition: all 0.15s;
  line-height: 1.4;
  border-left: 3px solid transparent;
}

.toc-item:hover {
  background: #fffbeb;
  border-left-color: #d97706;
}

.toc-level-2 {
  font-size: 0.85rem;
  font-weight: 500;
}

.toc-level-3 {
  font-size: 0.8rem;
  padding-left: 1.5rem;
  color: #b45309;
  border-left-color: #fde68a;
}

.toc-level-3:hover {
  border-left-color: #d97706;
}
</style>
