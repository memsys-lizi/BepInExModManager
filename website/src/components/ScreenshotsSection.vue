<script setup lang="ts">
import { ref } from 'vue'
import { ChevronLeft, ChevronRight } from 'lucide-vue-next'

const tabs = [
  { id: 'mods',    label: 'Mod 管理',      src: '/screenshots/mods.png' },
  { id: 'bepinex', label: 'BepInEx 安装',  src: '/screenshots/bepinex.png' },
  { id: 'config',  label: '配置文件编辑',  src: '/screenshots/config.png' },
  { id: 'home',    label: '游戏列表',       src: '/screenshots/home.png' },
  { id: 'settings',label: '设置',          src: '/screenshots/settings.png' },
]

const active = ref(0)

function prev() { active.value = (active.value - 1 + tabs.length) % tabs.length }
function next() { active.value = (active.value + 1) % tabs.length }
</script>

<template>
  <section class="screenshots" id="screenshots">
    <div class="screenshots__inner">
      <div class="screenshots__header">
        <h2 class="screenshots__title">应用截图</h2>
        <p class="screenshots__sub">界面简洁，操作直观</p>
      </div>

      <!-- Tab bar -->
      <div class="screenshots__tabs">
        <button
          v-for="(tab, i) in tabs"
          :key="tab.id"
          class="screenshots__tab"
          :class="{ 'screenshots__tab--active': i === active }"
          @click="active = i"
        >
          {{ tab.label }}
        </button>
      </div>

      <!-- Screenshot display -->
      <div class="screenshots__display">
        <div class="screenshots__frame">
          <!-- Window chrome -->
          <div class="screenshots__chrome">
            <div class="screenshots__chrome-dots">
              <span /><span /><span />
            </div>
            <span class="screenshots__chrome-title">BepInEx Mod Manager</span>
            <div style="width: 60px;" />
          </div>
          <!-- Real screenshot -->
          <div class="screenshots__img-wrap">
            <transition name="fade" mode="out-in">
              <img
                :key="tabs[active].id"
                :src="tabs[active].src"
                :alt="tabs[active].label"
                class="screenshots__img"
              />
            </transition>
          </div>
        </div>

        <!-- Navigation -->
        <div class="screenshots__nav">
          <button class="screenshots__arrow" @click="prev" aria-label="上一张">
            <ChevronLeft :size="16" />
          </button>
          <div class="screenshots__dots">
            <button
              v-for="(_, i) in tabs"
              :key="i"
              class="screenshots__dot"
              :class="{ 'screenshots__dot--active': i === active }"
              @click="active = i"
              :aria-label="`切换到第 ${i + 1} 张`"
            />
          </div>
          <button class="screenshots__arrow" @click="next" aria-label="下一张">
            <ChevronRight :size="16" />
          </button>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.screenshots {
  padding: 100px 24px;
  border-top: 1px solid var(--color-border);
}

.screenshots__inner {
  max-width: 960px;
  margin: 0 auto;
}

.screenshots__header {
  text-align: center;
  margin-bottom: 40px;
}

.screenshots__title {
  font-size: clamp(1.8rem, 4vw, 2.8rem);
  font-weight: 800;
  letter-spacing: -0.03em;
  color: var(--color-text-primary);
  margin-bottom: 12px;
}

.screenshots__sub {
  font-size: 15px;
  color: var(--color-text-secondary);
}

.screenshots__tabs {
  display: flex;
  justify-content: center;
  gap: 4px;
  margin-bottom: 28px;
  flex-wrap: wrap;
}

.screenshots__tab {
  padding: 7px 16px;
  font-size: 13px;
  color: var(--color-text-secondary);
  border: 1px solid var(--color-border);
  border-radius: 100px;
  transition: all 150ms ease;
  background: transparent;
}

.screenshots__tab:hover {
  color: var(--color-text-primary);
  border-color: var(--color-border-2);
  background: var(--color-surface);
}

.screenshots__tab--active {
  color: var(--color-text-primary);
  background: var(--color-surface-2);
  border-color: var(--color-border-2);
}

.screenshots__display {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.screenshots__frame {
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  overflow: hidden;
  background: var(--color-surface);
  box-shadow: 0 32px 64px rgba(0, 0, 0, 0.5);
}

/* Fake window chrome */
.screenshots__chrome {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 16px;
  height: 38px;
  background: var(--color-bg);
  border-bottom: 1px solid var(--color-border);
  flex-shrink: 0;
}

.screenshots__chrome-dots {
  display: flex;
  gap: 6px;
}

.screenshots__chrome-dots span {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: var(--color-border-2);
}

.screenshots__chrome-title {
  font-size: 12px;
  color: var(--color-text-muted);
}

.screenshots__img-wrap {
  overflow: hidden;
  background: #f5f5f4;
  line-height: 0;
}

.screenshots__img {
  width: 100%;
  height: auto;
  display: block;
}

/* Fade transition */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 180ms ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

/* Navigation */
.screenshots__nav {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 16px;
}

.screenshots__arrow {
  width: 34px;
  height: 34px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  color: var(--color-text-secondary);
  background: var(--color-surface);
  transition: all 150ms ease;
}

.screenshots__arrow:hover {
  border-color: var(--color-border-2);
  color: var(--color-text-primary);
  background: var(--color-surface-2);
}

.screenshots__dots {
  display: flex;
  gap: 6px;
  align-items: center;
}

.screenshots__dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--color-border-2);
  transition: background 150ms ease, width 150ms ease;
  border: none;
  padding: 0;
}

.screenshots__dot--active {
  background: var(--color-text-secondary);
  width: 20px;
  border-radius: 3px;
}

@media (max-width: 580px) {
  .screenshots { padding: 60px 16px; }
}
</style>
