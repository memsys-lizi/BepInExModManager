<script setup lang="ts">
import { FolderOpen, PackageOpen, Package, FileSliders } from 'lucide-vue-next'

const steps = [
  {
    icon: FolderOpen,
    num: '01',
    title: '添加游戏',
    desc: '点击首页的「添加游戏」卡片，直接选择游戏的 .exe 可执行文件。软件自动识别游戏名称与路径，无需手动填写。',
  },
  {
    icon: PackageOpen,
    num: '02',
    title: '安装 BepInEx',
    desc: '进入游戏页，点击「去安装」前往 BepInEx 管理页。根据游戏架构选择 x64 或 x86 版本，一键下载安装。',
  },
  {
    icon: Package,
    num: '03',
    title: '安装 Mod',
    desc: '将 .zip 或 .dll 文件拖拽到软件窗口，或点击「安装 Mod」手动选择文件，也支持输入 URL 直链下载安装。',
  },
  {
    icon: FileSliders,
    num: '04',
    title: '管理与配置',
    desc: '通过开关启用/禁用 Mod，点击「配置文件」可视化编辑 .cfg 文件，修改即时自动保存，无需重启。',
  },
]
</script>

<template>
  <section class="howto" id="howto">
    <div class="howto__inner">
      <div class="howto__header">
        <h2 class="howto__title">快速上手</h2>
        <p class="howto__sub">四步完成从零到 Mod 管理</p>
      </div>

      <div class="howto__steps">
        <div
          v-for="(step, i) in steps"
          :key="step.num"
          class="howto-step"
        >
          <div class="howto-step__left">
            <div class="howto-step__num">{{ step.num }}</div>
            <div v-if="i < steps.length - 1" class="howto-step__line" />
          </div>
          <div class="howto-step__content">
            <div class="howto-step__icon-wrap">
              <component :is="step.icon" :size="16" />
            </div>
            <h3 class="howto-step__title">{{ step.title }}</h3>
            <p class="howto-step__desc">{{ step.desc }}</p>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.howto {
  padding: 100px 24px;
  border-top: 1px solid var(--color-border);
}

.howto__inner {
  max-width: 700px;
  margin: 0 auto;
}

.howto__header {
  text-align: center;
  margin-bottom: 60px;
}

.howto__title {
  font-size: clamp(1.8rem, 4vw, 2.8rem);
  font-weight: 800;
  letter-spacing: -0.03em;
  color: var(--color-text-primary);
  margin-bottom: 12px;
}

.howto__sub {
  font-size: 15px;
  color: var(--color-text-secondary);
}

.howto__steps {
  display: flex;
  flex-direction: column;
  gap: 0;
}

.howto-step {
  display: flex;
  gap: 24px;
}

.howto-step__left {
  display: flex;
  flex-direction: column;
  align-items: center;
  flex-shrink: 0;
  width: 40px;
}

.howto-step__num {
  width: 40px;
  height: 40px;
  border: 1px solid var(--color-border-2);
  border-radius: var(--radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  font-weight: 700;
  color: var(--color-text-muted);
  background: var(--color-surface);
  flex-shrink: 0;
}

.howto-step__line {
  width: 1px;
  flex: 1;
  background: var(--color-border);
  margin: 8px 0;
  min-height: 32px;
}

.howto-step__content {
  padding-bottom: 40px;
  flex: 1;
}

.howto-step__icon-wrap {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  color: var(--color-text-secondary);
  margin-bottom: 12px;
}

.howto-step__title {
  font-size: 16px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin-bottom: 8px;
  letter-spacing: -0.01em;
}

.howto-step__desc {
  font-size: 14px;
  color: var(--color-text-secondary);
  line-height: 1.75;
}

@media (max-width: 580px) {
  .howto { padding: 60px 16px; }
}
</style>
