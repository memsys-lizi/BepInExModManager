<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useSettingsStore } from '@/store/settingsStore'
import AppTopBar from '@/components/layout/AppTopBar.vue'
import BaseButton from '@/components/ui/BaseButton.vue'
import BaseToggle from '@/components/ui/BaseToggle.vue'
import { RotateCcw } from 'lucide-vue-next'

const settingsStore = useSettingsStore()
const { settings } = storeToRefs(settingsStore)
</script>

<template>
  <div class="page">
    <AppTopBar>
      <template #actions>
        <BaseButton variant="ghost" size="sm" @click="settingsStore.reset">
          <RotateCcw :size="13" />
          恢复默认
        </BaseButton>
      </template>
    </AppTopBar>

    <div class="page__body">
      <!-- Download -->
      <section class="section">
        <div class="section__title">下载</div>
        <div class="settings-table">
          <div class="settings-row">
            <div class="settings-row__label">
              <span>BepInEx 下载源</span>
              <span class="text-xs text-muted">影响安装时的下载速度</span>
            </div>
            <select v-model="settings.bepinexSource" class="settings-select">
              <option value="github">GitHub（官方）</option>
              <option value="mirror">镜像源</option>
            </select>
          </div>
          <div class="settings-row">
            <div class="settings-row__label">
              <span>HTTP 代理</span>
              <span class="text-xs text-muted">例如 http://127.0.0.1:7890</span>
            </div>
            <input
              v-model="settings.downloadProxy"
              class="settings-input"
              placeholder="留空则不使用代理"
            />
          </div>
        </div>
      </section>

      <!-- General -->
      <section class="section">
        <div class="section__title">通用</div>
        <div class="settings-table">
          <div class="settings-row">
            <div class="settings-row__label">
              <span>界面语言</span>
            </div>
            <select v-model="settings.language" class="settings-select">
              <option value="zh-CN">简体中文</option>
              <option value="en-US">English</option>
            </select>
          </div>
        </div>
      </section>

      <!-- About -->
      <section class="section">
        <div class="section__title">关于</div>
        <div class="settings-table">
          <div class="settings-row">
            <span class="text-secondary text-sm">BepInEx Mod Manager</span>
            <span class="text-muted text-sm">v0.1.0</span>
          </div>
          <div class="settings-row">
            <span class="text-secondary text-sm">技术栈</span>
            <span class="text-muted text-sm">Tauri 2 · Vue 3 · Rust</span>
          </div>
        </div>
      </section>
    </div>
  </div>
</template>

<style scoped>
.page { display: flex; flex-direction: column; height: 100%; }

.page__body {
  flex: 1;
  overflow-y: auto;
  padding: var(--space-5);
  display: flex;
  flex-direction: column;
  gap: var(--space-5);
}

.section__title {
  font-size: var(--text-sm);
  font-weight: 600;
  color: var(--color-text-muted);
  text-transform: uppercase;
  letter-spacing: 0.06em;
  margin-bottom: var(--space-3);
}

.settings-table {
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  overflow: hidden;
}

.settings-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-3) var(--space-4);
  border-bottom: 1px solid var(--color-border);
  gap: var(--space-4);
}
.settings-row:last-child { border-bottom: none; }

.settings-row__label {
  display: flex;
  flex-direction: column;
  gap: 2px;
  font-size: var(--text-sm);
  color: var(--color-text-primary);
}

.settings-select,
.settings-input {
  padding: 4px 8px;
  border-radius: var(--radius-sm);
  font-size: var(--text-sm);
  width: 180px;
}
</style>
