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
  uninstall: [name: string]
}>()

const filters = [
  { key: 'all' as const, label: '全部' },
  { key: 'installed' as const, label: '已安装' },
  { key: 'available' as const, label: '可导入' },
]
</script>

<template>
  <div>
    <div v-if="selectedCommand">
      <CommandDetail :command="selectedCommand" @back="selected = null" />
    </div>
    <div v-else>
      <div class="filter-bar">
        <button
          v-for="f in filters"
          :key="f.key"
          class="filter-btn"
          :class="{ active: filter === f.key }"
          @click="filter = f.key"
        >
          {{ f.label }}
        </button>
      </div>

      <div class="cmd-list">
        <CommandCard
          v-for="cmd in commands"
          :key="cmd.name"
          :command="cmd"
          @view="selected = $event"
          @import="emit('import', $event)"
          @uninstall="emit('uninstall', $event)"
        />
      </div>

      <p v-if="commands.length === 0" class="empty-text">
        没有匹配的命令
      </p>
    </div>
  </div>
</template>

<style scoped>
.filter-bar {
  display: flex;
  gap: 0.75rem;
  margin-bottom: 1.25rem;
}

.filter-btn {
  padding: 0.35rem 0.85rem;
  border-radius: 20px;
  font-family: 'Nunito', sans-serif;
  font-size: 0.85rem;
  border: 2px solid transparent;
  cursor: pointer;
  transition: all 0.15s;
  background: rgba(255, 255, 255, 0.5);
  color: #92400e;
  box-shadow: 2px 2px 4px rgba(146, 64, 14, 0.06);
}

.filter-btn:hover {
  background: #fef3c7;
}

.filter-btn.active {
  background: #fbbf24;
  color: #78350f;
  border-color: #d97706;
  box-shadow: 3px 3px 6px rgba(146, 64, 14, 0.1);
}

.cmd-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.empty-text {
  text-align: center;
  padding: 2rem 0;
  color: #b45309;
  font-family: 'Nunito', sans-serif;
  font-size: 0.85rem;
}
</style>
