<script setup lang="ts">
import { ref, computed } from 'vue'
import type { SkillWithStatus, Category } from '../data/skills'
import { embeddedSkills, categoryLabels } from '../data/skills'
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

const grouped = computed(() => {
  const groups = new Map<Category, SkillWithStatus[]>()
  for (const s of skills.value) {
    if (!groups.has(s.category)) groups.set(s.category, [])
    groups.get(s.category)!.push(s)
  }
  return groups
})

const categoryOrder: Category[] = ['superpowers', 'frontend', 'utility']

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

    <div v-if="skills.length === 0" class="empty-text">
      没有匹配的技能
    </div>

    <div v-for="cat in categoryOrder" :key="cat" class="category-group">
      <h3 v-if="grouped.get(cat)?.length" class="category-title">
        {{ categoryLabels[cat] }}
      </h3>
      <div class="skill-list">
        <SkillCard
          v-for="skill in grouped.get(cat)"
          :key="skill.name"
          :skill="skill"
          @import="emit('import', $event)"
          @uninstall="emit('uninstall', $event)"
        />
      </div>
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

.category-group {
  margin-bottom: 1.25rem;
}

.category-title {
  font-family: 'Fredoka', sans-serif;
  font-size: 0.95rem;
  font-weight: 600;
  color: #78350f;
  margin: 0 0 0.5rem 0;
}

.skill-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.empty-text {
  text-align: center;
  padding: 2rem 0;
  color: #b45309;
  font-family: 'Nunito', sans-serif;
  font-size: 0.85rem;
}
</style>
