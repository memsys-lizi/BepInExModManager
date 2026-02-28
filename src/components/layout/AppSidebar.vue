<script setup lang="ts">
import { useRouter, useRoute } from 'vue-router'
import { storeToRefs } from 'pinia'
import { useGameStore } from '@/store/gameStore'
import { Gamepad2, Plus, Settings, Home } from 'lucide-vue-next'

const router = useRouter()
const route = useRoute()
const gameStore = useGameStore()
const { games } = storeToRefs(gameStore)

function goToGame(id: string) {
  gameStore.setActiveGame(id)
  router.push({ name: 'game-detail', params: { id } })
}

function isGameActive(id: string) {
  return route.params.id === id
}

const isHomePage = () =>
  route.name === 'home' || (!route.params.id && route.name !== 'settings')

const isSettingsPage = () => route.name === 'settings'
</script>

<template>
  <aside class="sidebar">

    <!-- ── 游戏列表（滚动区） ── -->
    <div class="sidebar__section-label">游戏</div>

    <nav class="sidebar__nav">
      <button
        v-for="game in games"
        :key="game.id"
        class="sidebar__item"
        :class="{ 'sidebar__item--active': isGameActive(game.id) }"
        @click="goToGame(game.id)"
      >
        <img
          v-if="game.iconBase64"
          :src="game.iconBase64"
          class="sidebar__icon-img"
          alt=""
        />
        <Gamepad2 v-else :size="14" class="sidebar__icon" />
        <span class="truncate">{{ game.name }}</span>
      </button>

      <div v-if="games.length === 0" class="sidebar__empty">
        暂无游戏
      </div>
    </nav>

    <!-- 添加游戏按钮 -->
    <button class="sidebar__add" @click="router.push({ name: 'home' })">
      <Plus :size="13" />
      <span>添加游戏</span>
    </button>

    <!-- ── 底部固定 Tab ── -->
    <div class="sidebar__bottom">
      <button
        class="sidebar__tab"
        :class="{ 'sidebar__tab--active': isHomePage() }"
        @click="router.push({ name: 'home' })"
      >
        <Home :size="14" />
        <span>首页</span>
      </button>

      <button
        class="sidebar__tab"
        :class="{ 'sidebar__tab--active': isSettingsPage() }"
        @click="router.push({ name: 'settings' })"
      >
        <Settings :size="14" />
        <span>设置</span>
      </button>
    </div>

  </aside>
</template>

<style scoped>
.sidebar {
  width: var(--sidebar-width);
  height: 100%;
  background: var(--color-surface);
  border-right: 1px solid var(--color-border);
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
  overflow: hidden;
}

/* Section label */
.sidebar__section-label {
  padding: var(--space-4) var(--space-4) var(--space-1);
  font-size: var(--text-xs);
  font-weight: 600;
  color: var(--color-text-muted);
  text-transform: uppercase;
  letter-spacing: 0.06em;
  flex-shrink: 0;
}

/* Scrollable game list */
.sidebar__nav {
  flex: 1;
  overflow-y: auto;
  padding: 0 var(--space-2);
  min-height: 0;
}

/* Game nav item */
.sidebar__item {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  width: 100%;
  padding: 6px var(--space-2);
  border-radius: var(--radius-sm);
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  text-align: left;
  transition: background var(--transition-fast), color var(--transition-fast);
  cursor: pointer;
}

.sidebar__item:hover {
  background: var(--color-surface-2);
  color: var(--color-text-primary);
}

.sidebar__item--active {
  background: var(--color-accent-dim);
  color: var(--color-text-primary);
}

.sidebar__icon { flex-shrink: 0; color: var(--color-text-muted); }
.sidebar__icon-img { width: 14px; height: 14px; object-fit: contain; flex-shrink: 0; border-radius: 2px; }

.sidebar__empty {
  padding: var(--space-3) var(--space-2);
  font-size: var(--text-sm);
  color: var(--color-text-muted);
}

/* Add game button */
.sidebar__add {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  margin: var(--space-2);
  padding: 6px var(--space-2);
  border-radius: var(--radius-sm);
  border: 1px dashed var(--color-border-2);
  font-size: var(--text-sm);
  color: var(--color-text-muted);
  background: transparent;
  transition: border-color var(--transition-fast), color var(--transition-fast),
              background var(--transition-fast);
  cursor: pointer;
  text-align: left;
  flex-shrink: 0;
}

.sidebar__add:hover {
  border-color: var(--color-text-muted);
  color: var(--color-text-secondary);
  background: var(--color-surface-2);
}

/* ── Bottom Tab bar ── */
.sidebar__bottom {
  display: flex;
  border-top: 1px solid var(--color-border);
  flex-shrink: 0;
}

.sidebar__tab {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 3px;
  padding: var(--space-2) 0;
  font-size: var(--text-xs);
  color: var(--color-text-muted);
  cursor: pointer;
  background: transparent;
  transition: background var(--transition-fast), color var(--transition-fast);
  border-right: 1px solid var(--color-border);
}

.sidebar__tab:last-child {
  border-right: none;
}

.sidebar__tab:hover {
  background: var(--color-surface-2);
  color: var(--color-text-secondary);
}

.sidebar__tab--active {
  color: var(--color-text-primary);
  background: var(--color-accent-dim);
}
</style>
