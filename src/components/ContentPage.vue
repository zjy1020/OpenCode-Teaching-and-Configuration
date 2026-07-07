<script setup lang="ts">
import { computed } from 'vue'
import MarkdownRenderer from './MarkdownRenderer.vue'
import TableOfContents from './TableOfContents.vue'

const props = defineProps<{
  content: string
  title: string
}>()

const body = computed(() =>
  props.content.replace(/^#\s+.*\n/, '')
)
</script>

<template>
  <div class="content-layout">
    <div class="content-main">
      <h2 class="page-title">{{ title }}</h2>
      <article>
        <MarkdownRenderer :content="body" />
      </article>
    </div>
    <TableOfContents :content="content" />
  </div>
</template>

<style scoped>
.content-layout {
  display: flex;
  gap: 2rem;
}

.content-main {
  flex: 1;
  min-width: 0;
}

.page-title {
  font-family: 'Fredoka', sans-serif;
  font-size: 1.35rem;
  font-weight: 600;
  color: #78350f;
  margin: 0 0 0.75rem 0;
}
</style>
