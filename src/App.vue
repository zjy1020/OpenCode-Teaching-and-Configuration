<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import TitleBar from './components/TitleBar.vue'
import Sidebar from './components/Sidebar.vue'

const showBackTop = ref(false)
let mainEl: HTMLElement | null = null

function onScroll() {
  showBackTop.value = (mainEl?.scrollTop ?? 0) > 300
}

function scrollToTop() {
  mainEl?.scrollTo({ top: 0, behavior: 'smooth' })
}

onMounted(() => {
  mainEl = document.querySelector('.main-content')
  mainEl?.addEventListener('scroll', onScroll)
})
onUnmounted(() => {
  mainEl?.removeEventListener('scroll', onScroll)
})
</script>

<template>
  <div class="app-shell">
    <TitleBar />
    <div class="app-body">
      <Sidebar />
      <main class="main-content">
        <router-view />
      </main>
    </div>
    <button v-if="showBackTop" class="back-top" @click="scrollToTop">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="18 15 12 9 6 15"/></svg>
    </button>
  </div>
</template>

<style scoped>
.app-shell {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
}

.app-body {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.main-content {
  flex: 1;
  overflow-y: auto;
  padding: 1.5rem;
  background: #fffbeb;
}

.back-top {
  position: fixed;
  bottom: 2rem;
  right: 2rem;
  width: 40px;
  height: 40px;
  border: none;
  border-radius: 50%;
  background: #d97706;
  color: #fffbeb;
  cursor: pointer;
  box-shadow: 0 2px 12px rgba(120,53,15,0.25);
  transition: background 0.15s, transform 0.15s;
  z-index: 100;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0;
}
.back-top:hover {
  background: #b45309;
  transform: scale(1.1);
}
</style>
