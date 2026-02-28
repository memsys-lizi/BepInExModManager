import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
import type { AppSettings } from '@/types'

export type ThemeMode = 'dark' | 'light' | 'system'

const STORAGE_KEY = 'bmm-settings'

function loadFromStorage(): Partial<AppSettings & { themeMode: ThemeMode }> {
  try {
    const raw = localStorage.getItem(STORAGE_KEY)
    return raw ? JSON.parse(raw) : {}
  } catch {
    return {}
  }
}

const DEFAULTS: AppSettings & { themeMode: ThemeMode } = {
  theme: 'dark',
  themeMode: 'system',
  bepinexSource: 'github',
  language: 'zh-CN',
  windowWidth: 900,
  windowHeight: 600,
}

// 将主题 class 应用到 document.documentElement
function applyTheme(mode: ThemeMode) {
  const el = document.documentElement
  const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches

  if (mode === 'dark' || (mode === 'system' && prefersDark)) {
    el.removeAttribute('data-theme')
  } else {
    el.setAttribute('data-theme', 'light')
  }
}

export const useSettingsStore = defineStore('settings', () => {
  const saved = loadFromStorage()
  const settings = ref<AppSettings & { themeMode: ThemeMode }>({
    ...DEFAULTS,
    ...saved,
  })

  // 监听系统主题变化（只在 system 模式下响应）
  const systemMediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
  systemMediaQuery.addEventListener('change', () => {
    if (settings.value.themeMode === 'system') {
      applyTheme('system')
    }
  })

  // 初始化应用主题
  applyTheme(settings.value.themeMode)

  function setThemeMode(mode: ThemeMode) {
    settings.value.themeMode = mode
    applyTheme(mode)
  }

  function cycleTheme() {
    const order: ThemeMode[] = ['system', 'light', 'dark']
    const current = settings.value.themeMode
    const next = order[(order.indexOf(current) + 1) % order.length]
    setThemeMode(next)
  }

  function update(patch: Partial<AppSettings & { themeMode: ThemeMode }>) {
    settings.value = { ...settings.value, ...patch }
    if (patch.themeMode) applyTheme(patch.themeMode)
  }

  function reset() {
    settings.value = { ...DEFAULTS }
    applyTheme(DEFAULTS.themeMode)
  }

  // 持久化到 localStorage
  watch(settings, (val) => {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(val))
  }, { deep: true })

  return { settings, setThemeMode, cycleTheme, update, reset }
})
