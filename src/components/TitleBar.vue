<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getWindow } from '../lib/tauri'

const win = ref<Awaited<ReturnType<typeof getWindow>>>(null!)

onMounted(async () => {
  win.value = await getWindow()
})

function minimize() { win.value?.minimize() }
function toggleMaximize() { win.value?.toggleMaximize() }
function close() { win.value?.close() }

function startDrag(e: MouseEvent) {
  if ((e.target as HTMLElement).closest('.window-controls')) return
  win.value?.startDragging()
}
</script>

<template>
  <div class="title-bar" @mousedown.prevent="startDrag">
    <span class="title-label">OpenCode 教学工具</span>
    <div class="window-controls">
      <button class="ctrl-btn" @click="minimize" title="最小化">
        <svg width="12" height="12" viewBox="0 0 12 12"><rect x="1" y="5.5" width="10" height="1" fill="currentColor"/></svg>
      </button>
      <button class="ctrl-btn" @click="toggleMaximize" title="最大化/还原">
        <svg width="12" height="12" viewBox="0 0 12 12"><rect x="2" y="2" width="8" height="8" rx="1" fill="none" stroke="currentColor" stroke-width="1"/></svg>
      </button>
      <button class="ctrl-btn ctrl-close" @click="close" title="关闭">
        <svg width="12" height="12" viewBox="0 0 12 12"><path d="M2 2l8 8M10 2l-8 8" stroke="currentColor" stroke-width="1.2"/></svg>
      </button>
    </div>
  </div>
</template>

<style scoped>
.title-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 36px;
  background: #fef3c7;
  border-bottom: 1px solid #fde68a;
  user-select: none;
  flex-shrink: 0;
}

.title-label {
  font-family: 'Fredoka', sans-serif;
  font-size: 0.85rem;
  font-weight: 600;
  color: #92400e;
  padding-left: 1rem;
}

.window-controls {
  display: flex;
  height: 100%;
}

.ctrl-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 46px;
  height: 100%;
  border: none;
  background: transparent;
  color: #92400e;
  cursor: pointer;
  transition: background 0.12s;
}

.ctrl-btn:hover {
  background: #fde68a;
}

.ctrl-close:hover {
  background: #ef4444;
  color: #fff;
}
</style>
