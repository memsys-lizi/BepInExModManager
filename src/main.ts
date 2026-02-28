import { createApp } from 'vue'
import { createPinia } from 'pinia'
import router from './router'
import App from './App.vue'
import './assets/main.css'

// 在 mount 之前立即应用主题，避免浅色模式下的闪白
;(() => {
  try {
    const saved = localStorage.getItem('bmm-settings')
    const themeMode = saved ? (JSON.parse(saved)?.themeMode ?? 'system') : 'system'
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
    const isLight = themeMode === 'light' || (themeMode === 'system' && !prefersDark)
    if (isLight) {
      document.documentElement.setAttribute('data-theme', 'light')
    }
  } catch {
    // ignore
  }
})()

const app = createApp(App)
app.use(createPinia())
app.use(router)
app.mount('#app')
