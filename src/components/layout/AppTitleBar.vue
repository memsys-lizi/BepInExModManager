<script setup lang="ts">
import { ref } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { Minus, Square, X, PackageOpen } from 'lucide-vue-next'

const appWindow = getCurrentWindow()

const isMaximized = ref(false)

appWindow.isMaximized().then(v => { isMaximized.value = v })
appWindow.onResized(() => {
  appWindow.isMaximized().then(v => { isMaximized.value = v })
})

async function minimize() { await appWindow.minimize() }
async function toggleMaximize() { await appWindow.toggleMaximize() }
async function close() { await appWindow.close() }
</script>

<template>
  <div class="titlebar" data-tauri-drag-region>
    <!-- Left: App icon + title -->
    <div class="titlebar__left" data-tauri-drag-region>
      <PackageOpen :size="13" class="titlebar__app-icon" />
      <span class="titlebar__title" data-tauri-drag-region>BepInEx Mod Manager</span>
    </div>

    <!-- Right: window controls -->
    <div class="titlebar__right">
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

/* Close button - red on hover */
.titlebar__btn--close:hover {
  background: var(--color-win-close-hover);
  color: #ffffff;
}
</style>
