<script setup lang="ts">
import {
  Gamepad2, PackageOpen, Package, FileSliders,
  Play, Globe, Moon, Network,
} from 'lucide-vue-next'

const features = [
  {
    icon: Gamepad2,
    title: '多游戏管理',
    desc: '同时管理任意数量的游戏，各自独立的 Mod 列表与配置，直接选择 .exe 文件添加游戏。',
  },
  {
    icon: PackageOpen,
    title: 'BepInEx 管理',
    desc: '一键从 GitHub 下载并安装 BepInEx，自动检测安装状态与文件完整性，支持 x64 / x86 版本选择。',
  },
  {
    icon: Package,
    title: 'Mod 安装与管理',
    desc: '拖拽 .zip / .dll 文件安装 Mod，支持 URL 直链下载，智能处理命名冲突，启用/禁用无需重启游戏。',
  },
  {
    icon: FileSliders,
    title: '配置文件编辑',
    desc: '可视化编辑 BepInEx 及各 Mod 的 .cfg 文件，修改自动保存，内置 BepInEx.cfg 中文字段说明。',
  },
  {
    icon: Play,
    title: '一键启动游戏',
    desc: '无需离开软件，直接从 Mod 管理界面启动对应游戏，省去来回切换窗口的麻烦。',
  },
  {
    icon: Globe,
    title: '中文 / English',
    desc: '完整的中英文界面支持，可随时在设置中切换语言，所有 UI 文本均已本地化。',
  },
  {
    icon: Moon,
    title: '深色 / 浅色主题',
    desc: '支持深色、浅色和跟随系统三种主题模式，在设置页随时切换。',
  },
  {
    icon: Network,
    title: 'HTTP 代理支持',
    desc: '配置本地 HTTP 代理，解决国内 GitHub 下载速度慢的问题，也支持切换镜像下载源。',
  },
]
</script>

<template>
  <section class="features" id="features">
    <div class="features__inner">
      <div class="features__header">
        <h2 class="features__title">功能一览</h2>
        <p class="features__sub">涵盖 BepInEx 模组管理的全部核心需求</p>
      </div>

      <div class="features__grid">
        <div
          v-for="f in features"
          :key="f.title"
          class="feature-card"
        >
          <div class="feature-card__icon">
            <component :is="f.icon" :size="18" />
          </div>
          <h3 class="feature-card__title">{{ f.title }}</h3>
          <p class="feature-card__desc">{{ f.desc }}</p>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.features {
  padding: 100px 24px;
  border-top: 1px solid var(--color-border);
}

.features__inner {
  max-width: 1100px;
  margin: 0 auto;
}

.features__header {
  text-align: center;
  margin-bottom: 60px;
}

.features__title {
  font-size: clamp(1.8rem, 4vw, 2.8rem);
  font-weight: 800;
  letter-spacing: -0.03em;
  color: var(--color-text-primary);
  margin-bottom: 12px;
}

.features__sub {
  font-size: 15px;
  color: var(--color-text-secondary);
}

.features__grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 1px;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  overflow: hidden;
  background: var(--color-border);
}

.feature-card {
  background: var(--color-bg);
  padding: 28px 24px;
  transition: background 150ms ease;
}

.feature-card:hover {
  background: var(--color-surface);
}

.feature-card__icon {
  width: 36px;
  height: 36px;
  border: 1px solid var(--color-border-2);
  border-radius: var(--radius-sm);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-text-secondary);
  margin-bottom: 16px;
}

.feature-card__title {
  font-size: 14px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin-bottom: 8px;
}

.feature-card__desc {
  font-size: 13px;
  color: var(--color-text-secondary);
  line-height: 1.7;
}

@media (max-width: 1024px) {
  .features__grid { grid-template-columns: repeat(2, 1fr); }
}

@media (max-width: 580px) {
  .features { padding: 60px 16px; }
  .features__grid { grid-template-columns: 1fr; }
}
</style>
