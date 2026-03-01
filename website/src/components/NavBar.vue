<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { Github } from 'lucide-vue-next'

const scrolled = ref(false)

function onScroll() {
  scrolled.value = window.scrollY > 40
}

onMounted(() => window.addEventListener('scroll', onScroll))
onUnmounted(() => window.removeEventListener('scroll', onScroll))
</script>

<template>
  <header class="navbar" :class="{ 'navbar--scrolled': scrolled }">
    <div class="navbar__inner">
      <a href="#" class="navbar__logo">
        <img src="/icon.jpg" alt="BepInEx Mod Manager" class="navbar__icon" />
        <span class="navbar__name">BepInEx Mod Manager</span>
      </a>

      <nav class="navbar__nav">
        <a href="#features" class="navbar__link">功能</a>
        <a href="#screenshots" class="navbar__link">截图</a>
        <a href="#howto" class="navbar__link">使用方法</a>
        <a href="#tech" class="navbar__link">技术栈</a>
      </nav>

      <div class="navbar__actions">
        <a
          href="https://github.com/memsys-lizi/BepInExModManager"
          target="_blank"
          rel="noopener noreferrer"
          class="navbar__github"
          aria-label="GitHub 仓库"
        >
          <Github :size="18" />
          <span>GitHub</span>
        </a>
        <a href="#download" class="navbar__cta">下载</a>
      </div>
    </div>
  </header>
</template>

<style scoped>
.navbar {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 100;
  padding: 0 24px;
  transition: background 200ms ease, border-color 200ms ease;
  border-bottom: 1px solid transparent;
}

.navbar--scrolled {
  background: rgba(10, 10, 10, 0.92);
  border-color: var(--color-border);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
}

.navbar__inner {
  max-width: 1100px;
  margin: 0 auto;
  height: 60px;
  display: flex;
  align-items: center;
  gap: 32px;
}

.navbar__logo {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-shrink: 0;
}

.navbar__icon {
  width: 28px;
  height: 28px;
  border-radius: 7px;
  object-fit: cover;
}

.navbar__name {
  font-size: 15px;
  font-weight: 600;
  color: var(--color-text-primary);
  letter-spacing: -0.01em;
}

.navbar__nav {
  display: flex;
  align-items: center;
  gap: 4px;
  flex: 1;
}

.navbar__link {
  padding: 6px 12px;
  font-size: 14px;
  color: var(--color-text-secondary);
  border-radius: var(--radius-sm);
  transition: color 150ms ease, background 150ms ease;
}

.navbar__link:hover {
  color: var(--color-text-primary);
  background: var(--color-surface-2);
}

.navbar__actions {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
}

.navbar__github {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  font-size: 14px;
  color: var(--color-text-secondary);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  transition: color 150ms ease, border-color 150ms ease, background 150ms ease;
}

.navbar__github:hover {
  color: var(--color-text-primary);
  border-color: var(--color-border-2);
  background: var(--color-surface);
}

.navbar__cta {
  padding: 7px 16px;
  font-size: 14px;
  font-weight: 500;
  color: #0a0a0a;
  background: var(--color-text-primary);
  border-radius: var(--radius-sm);
  transition: opacity 150ms ease;
}

.navbar__cta:hover {
  opacity: 0.88;
}

@media (max-width: 768px) {
  .navbar__nav { display: none; }
  .navbar__name { display: none; }
}

@media (max-width: 480px) {
  .navbar__github span { display: none; }
}
</style>
