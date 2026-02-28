<script setup lang="ts">
import { ref, computed } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useSettingsStore } from '@/store/settingsStore'
import { Sun, Moon, Monitor, Minus, Square, X, PackageOpen } from 'lucide-vue-next'
import type { ThemeMode } from '@/store/settingsStore'

const appWindow = getCurrentWindow()
const settingsStore = useSettingsStore()

const isMaximized = ref(false)

// 初始化最大化状态
appWindow.isMaximized().then(v => { isMaximized.value = v })
appWindow.onResized(() => {
  appWindow.isMaximized().then(v => { isMaximized.value = v })
})

async function minimize() {
  await appWindow.minimize()
}

async function toggleMaximize() {
  await appWindow.toggleMaximize()
}

async function close() {
  await appWindow.close()
}

// 主题图标映射
const themeMode = computed(() => settingsStore.settings.themeMode)

const themeIcon = computed(() => {
  if (themeMode.value === 'light') return Sun
  if (themeMode.value === 'dark') return Moon
  return Monitor
})

const themeLabel = computed(() => {
  if (themeMode.value === 'light') return '浅色'
  if (themeMode.value === 'dark') return '深色'
  return '跟随系统'
})

function cycleTheme() {
  settingsStore.cycleTheme()
}
</script>

<template>
  <div class="titlebar" data-tauri-drag-region>
    <!-- Left: App icon + title -->
    <div class="titlebar__left" data-tauri-drag-region>
      <PackageOpen :size="13" class="titlebar__app-icon" />
      <span class="titlebar__title" data-tauri-drag-region>BepInEx Mod Manager</span>
    </div>

    <!-- Right: theme toggle + window controls -->
    <div class="titlebar__right">
      <!-- Theme cycle button -->
      <button
        class="titlebar__btn titlebar__btn--theme"
        :title="themeLabel"
        @click="cycleTheme"
      >
        <component :is="themeIcon" :size="12" />
        <span class="titlebar__theme-label">{{ themeLabel }}</span>
      </button>

      <div class="titlebar__divider" />

      <!-- Minimize -->
      <button class="titlebar__btn" title="最小化" @click="minimize">
        <Minus :size="13" />
      </button>

      <!-- Maximize / Restore -->
      <button class="titlebar__btn" :title="isMaximized ? '还原' : '最大化'" @click="toggleMaximize">
        <Square :size="11" :stroke-width="isMaximized ? 1.5 : 2" />
      </button>

      <!-- Close -->
      <button class="titlebar__btn titlebar__btn--close" title="关闭" @click="close">
        <X :size="13" />
      </button>
    </div>
  </div>
</template>

<style scoped>
.titlebar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 36px;
  background: var(--color-titlebar-bg);
  border-bottom: 1px solid var(--color-border);
  flex-shrink: 0;
  /* 允许整个标题栏拖动 */
  -webkit-app-region: drag;
  app-region: drag;
  padding: 0;
}

/* 禁止按钮区域的拖动，确保按钮可以点击 */
.titlebar__right,
.titlebar__btn {
  -webkit-app-region: no-drag;
  app-region: no-drag;
}

.titlebar__left {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 0 12px;
  flex: 1;
  min-width: 0;
}

.titlebar__app-icon {
  color: var(--color-titlebar-text);
  flex-shrink: 0;
}

.titlebar__title {
  font-size: 12px;
  font-weight: 500;
  color: var(--color-titlebar-text);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.titlebar__right {
  display: flex;
  align-items: center;
  height: 100%;
}

.titlebar__divider {
  width: 1px;
  height: 16px;
  background: var(--color-border);
  margin: 0 2px;
  -webkit-app-region: no-drag;
}

/* Window control buttons */
.titlebar__btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 42px;
  height: 36px;
  color: var(--color-titlebar-text);
  border-radius: 0;
  transition: background var(--transition-fast), color var(--transition-fast);
  cursor: pointer;
  flex-shrink: 0;
}

.titlebar__btn:hover {
  background: var(--color-surface-2);
  color: var(--color-text-primary);
}

/* Theme button - wider to fit text */
.titlebar__btn--theme {
  width: auto;
  padding: 0 10px;
  gap: 5px;
  border-radius: var(--radius-sm);
  margin-right: 4px;
}

.titlebar__btn--theme:hover {
  background: var(--color-accent-dim);
}

.titlebar__theme-label {
  font-size: 11px;
  white-space: nowrap;
}

/* Close button - red on hover */
.titlebar__btn--close:hover {
  background: var(--color-win-close-hover);
  color: #ffffff;
}
</style>
