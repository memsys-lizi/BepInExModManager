<script setup lang="ts">
import { computed } from 'vue'
import { storeToRefs } from 'pinia'
import { useSettingsStore, type ThemeMode } from '@/store/settingsStore'
import { useI18n } from '@/i18n'
import AppTopBar from '@/components/layout/AppTopBar.vue'
import BaseButton from '@/components/ui/BaseButton.vue'
import { RotateCcw } from 'lucide-vue-next'

const settingsStore = useSettingsStore()
const { settings } = storeToRefs(settingsStore)
const { t } = useI18n()

const themeValue = computed({
  get: () => settingsStore.settings.themeMode,
  set: (v: string) => settingsStore.setThemeMode(v as ThemeMode),
})
</script>

<template>
  <div class="page">
    <AppTopBar>
      <template #actions>
        <BaseButton variant="ghost" size="sm" @click="settingsStore.reset">
          <RotateCcw :size="13" />
          {{ t.settings.reset }}
        </BaseButton>
      </template>
    </AppTopBar>

    <div class="page__body">
      <!-- Download -->
      <section class="section">
        <div class="section__title">{{ t.settings.download }}</div>
        <div class="settings-table">
          <div class="settings-row">
            <div class="settings-row__label">
              <span>{{ t.settings.source }}</span>
              <span class="text-xs text-muted">{{ t.settings.sourceDesc }}</span>
            </div>
            <select v-model="settings.bepinexSource" class="settings-select">
              <option value="github">{{ t.settings.sourceGithub }}</option>
              <option value="mirror">{{ t.settings.sourceMirror }}</option>
            </select>
          </div>
          <div class="settings-row">
            <div class="settings-row__label">
              <span>{{ t.settings.proxy }}</span>
            </div>
            <input
              v-model="settings.downloadProxy"
              class="settings-input"
              :placeholder="t.settings.proxyPlaceholder"
            />
          </div>
        </div>
      </section>

      <!-- General -->
      <section class="section">
        <div class="section__title">{{ t.settings.general }}</div>
        <div class="settings-table">
          <div class="settings-row">
            <div class="settings-row__label">
              <span>{{ t.settings.language }}</span>
            </div>
            <select v-model="settings.language" class="settings-select">
              <option value="zh-CN">简体中文</option>
              <option value="en-US">English</option>
            </select>
          </div>
          <div class="settings-row">
            <div class="settings-row__label">
              <span>{{ t.sidebar.theme }}</span>
            </div>
            <select v-model="themeValue" class="settings-select">
              <option value="system">{{ t.sidebar.themeSystem }}</option>
              <option value="light">{{ t.sidebar.themeLight }}</option>
              <option value="dark">{{ t.sidebar.themeDark }}</option>
            </select>
          </div>
        </div>
      </section>

      <!-- About -->
      <section class="section">
        <div class="section__title">{{ t.settings.about }}</div>
        <div class="settings-table">
          <div class="settings-row">
            <span class="text-secondary text-sm">BepInEx Mod Manager</span>
            <span class="text-muted text-sm">v0.1.0</span>
          </div>
          <div class="settings-row">
            <span class="text-secondary text-sm">{{ t.settings.stack }}</span>
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
  width: 200px;
  border: 1px solid var(--color-border);
  background: var(--color-surface-2);
  color: var(--color-text-primary);
  transition: border-color var(--transition-fast);
}
.settings-select:focus,
.settings-input:focus {
  outline: none;
  border-color: var(--color-border-2);
}
</style>
