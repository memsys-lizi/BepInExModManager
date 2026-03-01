<script setup lang="ts">
import { Github } from 'lucide-vue-next'

const year = new Date().getFullYear()
</script>

<template>
  <footer class="footer">
    <div class="footer__inner">
      <div class="footer__left">
        <div class="footer__brand">
          <img src="/icon.jpg" alt="BepInEx Mod Manager" class="footer__icon" />
          <span class="footer__name">BepInEx Mod Manager</span>
        </div>
        <p class="footer__copy">
          &copy; {{ year }} small-lizi. MIT License.
        </p>
      </div>

      <div class="footer__links">
        <a
          href="https://github.com/memsys-lizi/BepInExModManager"
          target="_blank"
          rel="noopener noreferrer"
          class="footer__link"
        >
          <Github :size="14" />
          GitHub
        </a>
        <a
          href="https://github.com/memsys-lizi/BepInExModManager/releases"
          target="_blank"
          rel="noopener noreferrer"
          class="footer__link"
        >
          发布版本
        </a>
        <a
          href="https://github.com/memsys-lizi/BepInExModManager/issues"
          target="_blank"
          rel="noopener noreferrer"
          class="footer__link"
        >
          反馈问题
        </a>
        <a
          href="https://github.com/BepInEx/BepInEx"
          target="_blank"
          rel="noopener noreferrer"
          class="footer__link"
        >
          BepInEx 官方
        </a>
      </div>
    </div>
  </footer>
</template>

<style scoped>
.footer {
  border-top: 1px solid var(--color-border);
  padding: 36px 24px;
}

.footer__inner {
  max-width: 1100px;
  margin: 0 auto;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 24px;
  flex-wrap: wrap;
}

.footer__left {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.footer__brand {
  display: flex;
  align-items: center;
  gap: 8px;
}

.footer__icon {
  width: 22px;
  height: 22px;
  border-radius: 5px;
  object-fit: cover;
}

.footer__name {
  font-size: 14px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.footer__copy {
  font-size: 12px;
  color: var(--color-text-muted);
}

.footer__links {
  display: flex;
  align-items: center;
  gap: 4px;
  flex-wrap: wrap;
}

.footer__link {
  display: inline-flex;
  align-items: center;
  gap: 5px;
  padding: 5px 10px;
  font-size: 13px;
  color: var(--color-text-muted);
  border-radius: var(--radius-sm);
  transition: color 150ms ease, background 150ms ease;
}

.footer__link:hover {
  color: var(--color-text-primary);
  background: var(--color-surface);
}

@media (max-width: 580px) {
  .footer__inner { justify-content: center; text-align: center; }
  .footer__links { justify-content: center; }
}
</style>
