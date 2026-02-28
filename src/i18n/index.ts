import { computed } from 'vue'
import { useSettingsStore } from '@/store/settingsStore'
import zh from './zh'
import en from './en'

type Locale = typeof zh

const locales: Record<string, Locale> = { 'zh-CN': zh, 'en-US': en }

export function useI18n() {
  const settingsStore = useSettingsStore()
  const t = computed(() => locales[settingsStore.settings.language] ?? zh)
  return { t }
}

// 在组件外部（如 store）使用时的辅助
export function getLocale(lang: string): Locale {
  return locales[lang] ?? zh
}
