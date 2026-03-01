<script setup lang="ts">
const stack = [
  { name: 'Tauri 2', role: '桌面框架', href: 'https://tauri.app' },
  { name: 'Vue 3', role: '前端框架', href: 'https://vuejs.org' },
  { name: 'TypeScript', role: '类型系统', href: 'https://www.typescriptlang.org' },
  { name: 'Rust', role: '后端逻辑', href: 'https://www.rust-lang.org' },
  { name: 'Vite', role: '构建工具', href: 'https://vitejs.dev' },
  { name: 'Pinia', role: '状态管理', href: 'https://pinia.vuejs.org' },
  { name: 'Vue Router', role: '路由', href: 'https://router.vuejs.org' },
  { name: 'Lucide', role: '图标库', href: 'https://lucide.dev' },
  { name: 'reqwest', role: 'HTTP 客户端', href: 'https://github.com/seanmonstar/reqwest' },
  { name: 'Tokio', role: '异步运行时', href: 'https://tokio.rs' },
]
</script>

<template>
  <section class="tech" id="tech">
    <div class="tech__inner">
      <div class="tech__header">
        <h2 class="tech__title">技术栈</h2>
        <p class="tech__sub">基于现代化工具链构建，性能出色，打包体积轻量</p>
      </div>

      <div class="tech__grid">
        <a
          v-for="item in stack"
          :key="item.name"
          :href="item.href"
          target="_blank"
          rel="noopener noreferrer"
          class="tech-card"
        >
          <span class="tech-card__name">{{ item.name }}</span>
          <span class="tech-card__role">{{ item.role }}</span>
        </a>
      </div>
    </div>
  </section>
</template>

<style scoped>
.tech {
  padding: 100px 24px;
  border-top: 1px solid var(--color-border);
}

.tech__inner {
  max-width: 900px;
  margin: 0 auto;
}

.tech__header {
  text-align: center;
  margin-bottom: 48px;
}

.tech__title {
  font-size: clamp(1.8rem, 4vw, 2.8rem);
  font-weight: 800;
  letter-spacing: -0.03em;
  color: var(--color-text-primary);
  margin-bottom: 12px;
}

.tech__sub {
  font-size: 15px;
  color: var(--color-text-secondary);
}

.tech__grid {
  display: grid;
  grid-template-columns: repeat(5, 1fr);
  gap: 1px;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  overflow: hidden;
  background: var(--color-border);
}

.tech-card {
  background: var(--color-bg);
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 4px;
  transition: background 150ms ease;
}

.tech-card:hover {
  background: var(--color-surface);
}

.tech-card__name {
  font-size: 14px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.tech-card__role {
  font-size: 12px;
  color: var(--color-text-muted);
}

@media (max-width: 768px) {
  .tech__grid { grid-template-columns: repeat(3, 1fr); }
}

@media (max-width: 480px) {
  .tech { padding: 60px 16px; }
  .tech__grid { grid-template-columns: repeat(2, 1fr); }
}
</style>
