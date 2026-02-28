<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { useGameStore } from '@/store/gameStore'
import { ChevronRight } from 'lucide-vue-next'

const route = useRoute()
const gameStore = useGameStore()

const breadcrumbs = computed(() => {
  const crumbs: { label: string; name?: string; params?: Record<string, string> }[] = []
  const name = route.name as string
  const game = gameStore.games.find(g => g.id === route.params.id)

  if (name === 'home') {
    crumbs.push({ label: '首页' })
  } else if (name === 'settings') {
    crumbs.push({ label: '设置' })
  } else if (game) {
    crumbs.push({ label: game.name, name: 'game-detail', params: { id: game.id } })
    if (name === 'bepinex-installer') crumbs.push({ label: 'BepInEx' })
    if (name === 'mod-config') crumbs.push({ label: 'Mod 配置' })
  }

  return crumbs
})
</script>

<template>
  <header class="topbar">
    <nav class="topbar__breadcrumb">
      <template v-for="(crumb, i) in breadcrumbs" :key="i">
        <ChevronRight v-if="i > 0" :size="12" class="topbar__sep" />
        <span
          class="topbar__crumb"
          :class="{ 'topbar__crumb--last': i === breadcrumbs.length - 1 }"
        >{{ crumb.label }}</span>
      </template>
    </nav>

    <div class="topbar__right">
      <slot name="actions" />
    </div>
  </header>
</template>

<style scoped>
.topbar {
  height: var(--topbar-height);
  border-bottom: 1px solid var(--color-border);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 var(--space-5);
  flex-shrink: 0;
  background: var(--color-bg);
}

.topbar__breadcrumb {
  display: flex;
  align-items: center;
  gap: var(--space-1);
}

.topbar__sep {
  color: var(--color-text-muted);
}

.topbar__crumb {
  font-size: var(--text-sm);
  color: var(--color-text-muted);
}

.topbar__crumb--last {
  color: var(--color-text-primary);
  font-weight: 500;
}

.topbar__right {
  display: flex;
  align-items: center;
  gap: var(--space-2);
}
</style>
