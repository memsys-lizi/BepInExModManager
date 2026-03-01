<script setup lang="ts">
import { Download, Github, Star } from 'lucide-vue-next'
</script>

<template>
  <section class="hero">
    <div class="hero__inner">
      <div class="hero__badge">
        <Star :size="12" />
        <span>开源免费 · Windows 桌面应用</span>
      </div>

      <h1 class="hero__title">
        BepInEx<br />
        <span class="hero__title--dim">Mod Manager</span>
      </h1>

      <p class="hero__desc">
        为 BepInEx 游戏模组打造的现代化管理工具。<br />
        多游戏管理、一键安装 BepInEx、拖拽安装 Mod、<br />
        可视化配置编辑，开箱即用。
      </p>

      <div class="hero__actions" id="download">
        <a
          href="https://github.com/memsys-lizi/BepInExModManager/releases"
          target="_blank"
          rel="noopener noreferrer"
          class="hero__btn hero__btn--primary"
        >
          <Download :size="16" />
          下载最新版本
        </a>
        <a
          href="https://github.com/memsys-lizi/BepInExModManager"
          target="_blank"
          rel="noopener noreferrer"
          class="hero__btn hero__btn--ghost"
        >
          <Github :size="16" />
          查看源码
        </a>
      </div>

      <div class="hero__meta">
        <span class="hero__meta-item">
          <span class="hero__dot hero__dot--green" />
          基于 Tauri 2 + Rust
        </span>
        <span class="hero__meta-sep">·</span>
        <span class="hero__meta-item">支持 Windows</span>
        <span class="hero__meta-sep">·</span>
        <span class="hero__meta-item">中文 / English</span>
      </div>

      <!-- App mockup -->
      <div class="hero__mockup">
        <div class="hero__window">
          <div class="hero__window-bar">
            <span class="hero__window-title">BepInEx Mod Manager</span>
            <div class="hero__window-btns">
              <span class="hero__window-btn" />
              <span class="hero__window-btn" />
              <span class="hero__window-btn" />
            </div>
          </div>
          <div class="hero__window-body">
            <div class="hero__window-sidebar">
              <div class="hero__sidebar-item hero__sidebar-item--active">首页</div>
              <div class="hero__sidebar-divider" />
              <div class="hero__sidebar-item">
                <span class="hero__sidebar-dot hero__sidebar-dot--green" />
                Rhythm Doctor
              </div>
              <div class="hero__sidebar-item">
                <span class="hero__sidebar-dot hero__sidebar-dot--gray" />
                Hollow Knight
              </div>
              <div class="hero__sidebar-item">
                <span class="hero__sidebar-dot hero__sidebar-dot--green" />
                Celeste
              </div>
              <div class="hero__sidebar-bottom">设置</div>
            </div>
            <div class="hero__window-content">
              <div class="hero__content-header">游戏列表</div>
              <div class="hero__game-grid">
                <div class="hero__game-card" v-for="g in games" :key="g.name">
                  <div class="hero__game-icon" :style="{ background: g.color }">{{ g.initial }}</div>
                  <div class="hero__game-info">
                    <div class="hero__game-name">{{ g.name }}</div>
                    <div class="hero__game-meta">{{ g.mods }} Mods · <span :class="g.installed ? 'hero__game-ok' : 'hero__game-no'">{{ g.installed ? 'BepInEx 已安装' : '未安装' }}</span></div>
                  </div>
                </div>
                <div class="hero__game-add">+ 添加游戏</div>
              </div>
            </div>
          </div>
        </div>
        <div class="hero__mockup-glow" />
      </div>
    </div>
  </section>
</template>

<script lang="ts">
const games = [
  { name: 'Rhythm Doctor', mods: 12, installed: true, color: '#2d2d2d', initial: 'R' },
  { name: 'Hollow Knight', mods: 8, installed: true, color: '#1e2a3a', initial: 'H' },
  { name: 'Celeste', mods: 3, installed: false, color: '#3a1e2a', initial: 'C' },
]
</script>

<style scoped>
.hero {
  padding: 140px 24px 80px;
  text-align: center;
  position: relative;
  overflow: hidden;
}

.hero::before {
  content: '';
  position: absolute;
  top: 0; left: 50%;
  transform: translateX(-50%);
  width: 800px;
  height: 600px;
  background: radial-gradient(ellipse at center top, rgba(255,255,255,0.04) 0%, transparent 70%);
  pointer-events: none;
}

.hero__inner {
  max-width: 900px;
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 24px;
}

.hero__badge {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 5px 12px;
  border: 1px solid var(--color-border-2);
  border-radius: 100px;
  font-size: 12px;
  color: var(--color-text-secondary);
  background: var(--color-surface);
}

.hero__title {
  font-size: clamp(3rem, 8vw, 6rem);
  font-weight: 900;
  letter-spacing: -0.04em;
  line-height: 1.0;
  color: var(--color-text-primary);
}

.hero__title--dim {
  color: var(--color-text-muted);
}

.hero__desc {
  font-size: 16px;
  color: var(--color-text-secondary);
  line-height: 1.8;
  max-width: 480px;
}

.hero__actions {
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
  justify-content: center;
}

.hero__btn {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 11px 24px;
  font-size: 14px;
  font-weight: 500;
  border-radius: var(--radius-sm);
  transition: opacity 150ms ease, background 150ms ease, border-color 150ms ease;
}

.hero__btn--primary {
  background: var(--color-text-primary);
  color: #0a0a0a;
}

.hero__btn--primary:hover {
  opacity: 0.88;
}

.hero__btn--ghost {
  border: 1px solid var(--color-border);
  color: var(--color-text-secondary);
  background: var(--color-surface);
}

.hero__btn--ghost:hover {
  border-color: var(--color-border-2);
  color: var(--color-text-primary);
  background: var(--color-surface-2);
}

.hero__meta {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 13px;
  color: var(--color-text-muted);
  flex-wrap: wrap;
  justify-content: center;
}

.hero__meta-item {
  display: flex;
  align-items: center;
  gap: 5px;
}

.hero__dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  flex-shrink: 0;
}

.hero__dot--green { background: #4ade80; }
.hero__meta-sep { color: var(--color-border-2); }

/* ── App Mockup ── */
.hero__mockup {
  margin-top: 24px;
  position: relative;
  width: 100%;
  max-width: 780px;
}

.hero__mockup-glow {
  position: absolute;
  bottom: -60px;
  left: 50%;
  transform: translateX(-50%);
  width: 500px;
  height: 120px;
  background: radial-gradient(ellipse, rgba(255,255,255,0.06) 0%, transparent 70%);
  pointer-events: none;
}

.hero__window {
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  overflow: hidden;
  background: var(--color-surface);
  box-shadow: 0 40px 80px rgba(0,0,0,0.6);
}

.hero__window-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 16px;
  height: 40px;
  border-bottom: 1px solid var(--color-border);
  background: var(--color-surface);
}

.hero__window-title {
  font-size: 12px;
  color: var(--color-text-muted);
}

.hero__window-btns {
  display: flex;
  gap: 6px;
}

.hero__window-btn {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: var(--color-border-2);
}

.hero__window-body {
  display: flex;
  height: 340px;
  background: #f5f5f4;
}

.hero__window-sidebar {
  width: 170px;
  flex-shrink: 0;
  border-right: 1px solid var(--color-border);
  padding: 8px;
  display: flex;
  flex-direction: column;
  gap: 2px;
  background: var(--color-bg);
}

.hero__sidebar-item {
  padding: 7px 10px;
  font-size: 12px;
  color: var(--color-text-secondary);
  border-radius: var(--radius-sm);
  cursor: default;
  display: flex;
  align-items: center;
  gap: 6px;
}

.hero__sidebar-item--active {
  background: var(--color-surface-2);
  color: var(--color-text-primary);
}

.hero__sidebar-divider {
  height: 1px;
  background: var(--color-border);
  margin: 4px 0;
}

.hero__sidebar-dot {
  width: 5px;
  height: 5px;
  border-radius: 50%;
  flex-shrink: 0;
}
.hero__sidebar-dot--green { background: #4ade80; }
.hero__sidebar-dot--gray { background: var(--color-border-2); }

.hero__sidebar-bottom {
  margin-top: auto;
  padding: 7px 10px;
  font-size: 12px;
  color: var(--color-text-muted);
  border-top: 1px solid var(--color-border);
}

.hero__window-content {
  flex: 1;
  padding: 20px;
  overflow: hidden;
  text-align: left;
}

.hero__content-header {
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin-bottom: 16px;
}

.hero__game-grid {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.hero__game-card {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 12px;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  background: var(--color-bg);
  transition: border-color 150ms ease;
}

.hero__game-icon {
  width: 32px;
  height: 32px;
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 13px;
  font-weight: 700;
  color: var(--color-text-secondary);
  flex-shrink: 0;
}

.hero__game-name {
  font-size: 12px;
  font-weight: 500;
  color: var(--color-text-primary);
}

.hero__game-meta {
  font-size: 11px;
  color: var(--color-text-muted);
  margin-top: 2px;
}

.hero__game-ok { color: #4ade80; }
.hero__game-no { color: var(--color-text-muted); }

.hero__game-add {
  padding: 10px 12px;
  border: 1px dashed var(--color-border);
  border-radius: var(--radius-md);
  font-size: 12px;
  color: var(--color-text-muted);
  text-align: center;
}

@media (max-width: 600px) {
  .hero { padding: 120px 16px 60px; }
  .hero__window-sidebar { display: none; }
  .hero__window-body { height: 260px; }
}
</style>
