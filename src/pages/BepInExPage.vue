<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { useGameStore } from '@/store/gameStore'
import { useSettingsStore } from '@/store/settingsStore'
import AppTopBar from '@/components/layout/AppTopBar.vue'
import BaseButton from '@/components/ui/BaseButton.vue'
import BaseBadge from '@/components/ui/BaseBadge.vue'
import { Download, Trash2, RefreshCw, Loader, AlertCircle } from 'lucide-vue-next'
import {
  checkBepInExStatus, fetchBepInExReleases, installBepInEx, uninstallBepInEx,
  type ReleaseAsset, type InstallProgress, type BepInExIntegrity,
} from '@/api'

const route = useRoute()
const gameStore = useGameStore()
const settingsStore = useSettingsStore()
const proxy = computed(() => settingsStore.settings.downloadProxy ?? '')

const gameId = computed(() => route.params.id as string)
const game = computed(() => gameStore.games.find(g => g.id === gameId.value))

const installed = ref(false)
const installedVersion = ref('')
const integrity = ref<BepInExIntegrity | null>(null)
const releases = ref<ReleaseAsset[]>([])
const selectedRelease = ref<ReleaseAsset | null>(null)

const loadingReleases = ref(false)
const installing = ref(false)
const progress = ref(0)
const statusText = ref('')
const error = ref('')

const integrityItems = computed(() => {
  if (!integrity.value) return []
  const ig = integrity.value
  return [
    { label: 'winhttp.dll（注入器）', ok: ig.winhttp_dll, required: true },
    { label: 'doorstop_config.ini', ok: ig.doorstop_cfg, required: true },
    { label: '.doorstop_version', ok: ig.doorstop_version, required: true },
    { label: 'BepInEx/core/ 目录', ok: ig.core_dir, required: true },
    { label: 'core/BepInEx*.dll（核心）', ok: ig.core_bepinex_dll, required: true },
    { label: 'changelog.txt', ok: ig.changelog, required: false },
  ]
})

async function loadStatus() {
  if (!game.value) return
  const s = await checkBepInExStatus(game.value.path)
  installed.value = s.installed
  installedVersion.value = s.version ?? ''
  integrity.value = s.integrity
}

async function loadReleases() {
  loadingReleases.value = true
  error.value = ''
  try {
    const list = await fetchBepInExReleases(proxy.value)
    releases.value = list
    if (list.length > 0) selectedRelease.value = list[0]
  } catch (e: any) {
    error.value = `获取版本列表失败: ${e}`
    // fallback: 显示占位提示
  } finally {
    loadingReleases.value = false
  }
}

async function install() {
  if (!selectedRelease.value || !game.value) return
  installing.value = true
  progress.value = 0
  statusText.value = '正在准备...'
  error.value = ''

  const onProgress = (p: InstallProgress) => {
    progress.value = p.percent
    statusText.value = p.message
  }

  try {
    await installBepInEx(
      game.value.path,
      selectedRelease.value.download_url,
      selectedRelease.value.version,
      onProgress,
      proxy.value,
    )
    await loadStatus()
  } catch (e: any) {
    error.value = String(e)
  } finally {
    installing.value = false
  }
}

async function uninstall() {
  if (!game.value) return
  try {
    await uninstallBepInEx(game.value.path)
    installed.value = false
    installedVersion.value = ''
    statusText.value = '已卸载'
  } catch (e: any) {
    error.value = String(e)
  }
}

onMounted(async () => {
  await loadStatus()
  await loadReleases()
})
</script>

<template>
  <div class="page">
    <AppTopBar>
      <template #actions>
        <BaseButton variant="ghost" size="sm" :disabled="loadingReleases" @click="loadReleases">
          <Loader v-if="loadingReleases" :size="13" class="spin" />
          <RefreshCw v-else :size="13" />
          刷新版本
        </BaseButton>
      </template>
    </AppTopBar>

    <div class="page__body">
      <!-- Error -->
      <div v-if="error" class="error-bar">
        <AlertCircle :size="13" />
        <span>{{ error }}</span>
        <button class="error-bar__close" @click="error = ''">×</button>
      </div>

      <!-- Current status -->
      <section class="section">
        <h3 class="section__title">当前状态</h3>
        <div class="status-card">
          <div class="status-card__row">
            <span class="text-secondary text-sm">安装状态</span>
            <BaseBadge :variant="installed ? 'success' : 'danger'">
              {{ installed ? '已安装' : '未安装' }}
            </BaseBadge>
          </div>
          <div v-if="installed" class="status-card__row">
            <span class="text-secondary text-sm">当前版本</span>
            <span class="text-sm font-mono">{{ installedVersion || '未知' }}</span>
          </div>
          <div class="status-card__row">
            <span class="text-secondary text-sm">游戏目录</span>
            <span class="text-sm text-muted truncate" style="max-width:280px">{{ game?.path }}</span>
          </div>

          <!-- 完整性检查 -->
          <div v-if="integrity" class="status-card__row status-card__row--integrity">
            <span class="text-secondary text-sm">完整性检查</span>
            <div class="integrity-grid">
              <div
                v-for="item in integrityItems"
                :key="item.label"
                class="integrity-item"
                :class="item.ok ? 'integrity-item--ok' : (item.required ? 'integrity-item--missing' : 'integrity-item--optional')"
              >
                <span class="integrity-item__dot" />
                <span class="integrity-item__label">{{ item.label }}</span>
                <span v-if="!item.required" class="integrity-item__tag">可选</span>
              </div>
              <div class="integrity-score">
                必须项 {{ integrity.score }} / 5
                <span class="text-xs text-muted">（≥4 视为已安装）</span>
              </div>
            </div>
          </div>
        </div>
      </section>

      <!-- Version picker -->
      <section class="section">
        <h3 class="section__title">选择版本</h3>

        <div v-if="loadingReleases" class="releases-loading">
          <Loader :size="16" class="spin text-muted" />
          <span class="text-sm text-muted">正在获取版本列表...</span>
        </div>

        <div v-else-if="releases.length === 0" class="releases-loading">
          <AlertCircle :size="16" class="text-muted" />
          <span class="text-sm text-muted">无法获取版本列表，请检查网络或使用代理</span>
        </div>

        <div v-else class="release-list">
          <button
            v-for="r in releases"
            :key="`${r.version}-${r.arch}`"
            class="release-item"
            :class="{ 'release-item--selected': selectedRelease === r }"
            @click="selectedRelease = r"
          >
            <div class="release-item__left">
              <span class="release-item__ver">{{ r.version }}</span>
              <BaseBadge variant="muted">{{ r.arch }}</BaseBadge>
            </div>
            <span class="text-xs text-muted">{{ r.published_at }}</span>
          </button>
        </div>
      </section>

      <!-- Actions + Progress -->
      <section class="section section--actions">
        <!-- Progress bar -->
        <div v-if="installing" class="progress-wrap">
          <div class="progress-bar">
            <div class="progress-bar__fill" :style="{ width: `${progress}%` }" />
          </div>
          <span class="text-xs text-muted">{{ statusText }} {{ progress }}%</span>
        </div>

        <div class="action-row">
          <BaseButton
            :disabled="!selectedRelease || installing || loadingReleases"
            :loading="installing"
            @click="install"
          >
            <Download :size="13" />
            {{ installed ? '重新安装' : '安装' }}
          </BaseButton>
          <BaseButton
            v-if="installed"
            variant="danger"
            :disabled="installing"
            @click="uninstall"
          >
            <Trash2 :size="13" />
            卸载
          </BaseButton>
        </div>

        <p v-if="statusText && !installing" class="text-sm text-muted">{{ statusText }}</p>
      </section>
    </div>
  </div>
</template>

<style scoped>
.page { display: flex; flex-direction: column; height: 100%; }

.page__body {
  flex: 1;
  overflow-y: auto;
  padding: var(--space-5);
  display: flex;
  flex-direction: column;
  gap: var(--space-5);
}

.error-bar {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-4);
  background: var(--color-danger-bg);
  color: var(--color-danger);
  font-size: var(--text-sm);
  border-radius: var(--radius-sm);
  flex-shrink: 0;
}
.error-bar__close { margin-left: auto; cursor: pointer; font-size: var(--text-md); color: inherit; }

.section__title {
  font-size: var(--text-sm);
  font-weight: 600;
  color: var(--color-text-muted);
  text-transform: uppercase;
  letter-spacing: 0.06em;
  margin-bottom: var(--space-3);
}

.status-card {
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  overflow: hidden;
}
.status-card__row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-3) var(--space-4);
  border-bottom: 1px solid var(--color-border);
}
.status-card__row:last-child { border-bottom: none; }
.status-card__row--integrity { align-items: flex-start; flex-wrap: wrap; gap: var(--space-2); }

.integrity-grid {
  display: flex;
  flex-direction: column;
  gap: 5px;
  min-width: 200px;
}

.integrity-item {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: var(--text-xs);
}

.integrity-item__dot {
  width: 7px;
  height: 7px;
  border-radius: 50%;
  flex-shrink: 0;
}

.integrity-item--ok      .integrity-item__dot { background: var(--color-success); }
.integrity-item--missing .integrity-item__dot { background: var(--color-danger); }
.integrity-item--optional .integrity-item__dot { background: var(--color-border-2); }

.integrity-item--ok      .integrity-item__label { color: var(--color-text-primary); }
.integrity-item--missing .integrity-item__label { color: var(--color-danger); }
.integrity-item--optional .integrity-item__label { color: var(--color-text-muted); }

.integrity-item__tag {
  margin-left: auto;
  font-size: 10px;
  color: var(--color-text-muted);
  border: 1px solid var(--color-border);
  border-radius: 2px;
  padding: 0 4px;
  line-height: 14px;
}

.integrity-score {
  margin-top: 4px;
  font-size: var(--text-xs);
  font-weight: 600;
  color: var(--color-text-secondary);
  padding-top: 4px;
  border-top: 1px solid var(--color-border);
}

.releases-loading {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-4) 0;
}

.release-list { display: flex; flex-direction: column; gap: var(--space-1); }

.release-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-sm);
  border: 1px solid var(--color-border);
  background: var(--color-surface);
  cursor: pointer;
  transition: border-color var(--transition-fast), background var(--transition-fast);
  text-align: left;
}
.release-item:hover { border-color: var(--color-border-2); background: var(--color-surface-2); }
.release-item--selected { border-color: var(--color-accent); }

.release-item__left { display: flex; align-items: center; gap: var(--space-2); }
.release-item__ver { font-size: var(--text-sm); font-weight: 500; font-family: var(--font-mono); }

.section--actions { display: flex; flex-direction: column; gap: var(--space-3); }
.action-row { display: flex; gap: var(--space-2); }

.progress-wrap { display: flex; flex-direction: column; gap: var(--space-1); }
.progress-bar {
  height: 3px;
  background: var(--color-border);
  border-radius: 2px;
  overflow: hidden;
}
.progress-bar__fill {
  height: 100%;
  background: var(--color-accent);
  transition: width 200ms ease;
}

.spin { animation: spin 1s linear infinite; }
@keyframes spin { to { transform: rotate(360deg); } }
</style>
