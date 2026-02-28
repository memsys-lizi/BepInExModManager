<script setup lang="ts">
import { RouterView } from 'vue-router'
import AppTitleBar from '@/components/layout/AppTitleBar.vue'
import AppSidebar from '@/components/layout/AppSidebar.vue'

// 初始化 settings store（触发主题应用）
import { useSettingsStore } from '@/store/settingsStore'
useSettingsStore()
</script>

<template>
  <div class="app-shell">
    <!-- Custom title bar - spans full width at top -->
    <AppTitleBar />

    <!-- Content area: sidebar + main -->
    <div class="app-body">
      <AppSidebar />
      <main class="app-main">
        <RouterView />
      </main>
    </div>
  </div>
</template>

<style>
.app-shell {
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100%;
  overflow: hidden;
}

.app-body {
  flex: 1;
  display: flex;
  overflow: hidden;
  min-height: 0;
}

.app-main {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background: var(--color-bg);
}
</style>
