<script setup lang="ts">
import { useRouter, useRoute } from 'vue-router'
import { storeToRefs } from 'pinia'
import { useGameStore } from '@/store/gameStore'
import { useI18n } from '@/i18n'
import { Gamepad2, Home, Settings } from 'lucide-vue-next'

const router = useRouter()
const route = useRoute()
const gameStore = useGameStore()
const { games } = storeToRefs(gameStore)
const { t } = useI18n()

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

    <!-- ── 首页按钮（最顶部） ── -->
    <button
      class="sidebar__nav-btn"
      :class="{ 'sidebar__nav-btn--active': isHomePage() }"
      @click="router.push({ name: 'home' })"
    >
      <Home :size="14" class="sidebar__nav-btn-icon" />
      <span>{{ t.sidebar.home }}</span>
    </button>

    <!-- 分割线 -->
    <div class="sidebar__divider" />

    <!-- ── 游戏标签 ── -->
    <div class="sidebar__section-label">{{ t.sidebar.games }}</div>

    <!-- ── 游戏列表（滚动区） ── -->
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
        {{ t.sidebar.noGames }}
      </div>
    </nav>

    <!-- ── 底部：设置按钮 ── -->
    <div class="sidebar__bottom">
      <button
        class="sidebar__nav-btn sidebar__nav-btn--bottom"
        :class="{ 'sidebar__nav-btn--active': isSettingsPage() }"
        @click="router.push({ name: 'settings' })"
      >
        <Settings :size="14" class="sidebar__nav-btn-icon" />
        <span>{{ t.sidebar.settings }}</span>
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

/* ── 顶部/底部导航按钮（与添加游戏风格统一） ── */
.sidebar__nav-btn {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  width: calc(100% - var(--space-4));
  margin: var(--space-2) var(--space-2) 0;
  padding: 7px var(--space-2);
  border-radius: var(--radius-sm);
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  background: transparent;
  border: none;
  text-align: left;
  cursor: pointer;
  transition: background var(--transition-fast), color var(--transition-fast);
  flex-shrink: 0;
}

.sidebar__nav-btn:hover {
  background: var(--color-surface-2);
  color: var(--color-text-primary);
}

.sidebar__nav-btn--active {
  background: var(--color-accent-dim);
  color: var(--color-text-primary);
  font-weight: 500;
}

.sidebar__nav-btn-icon {
  flex-shrink: 0;
  color: var(--color-text-muted);
}
.sidebar__nav-btn--active .sidebar__nav-btn-icon,
.sidebar__nav-btn:hover .sidebar__nav-btn-icon {
  color: var(--color-text-primary);
}

/* 分割线 */
.sidebar__divider {
  height: 1px;
  background: var(--color-border);
  margin: var(--space-2) var(--space-2) 0;
  flex-shrink: 0;
}

/* Section label */
.sidebar__section-label {
  padding: var(--space-3) var(--space-4) var(--space-1);
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
  background: transparent;
  border: none;
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

/* ── 底部区域 ── */
.sidebar__bottom {
  flex-shrink: 0;
  border-top: 1px solid var(--color-border);
  padding-bottom: var(--space-2);
}

.sidebar__nav-btn--bottom {
  margin-top: var(--space-2);
}
</style>
