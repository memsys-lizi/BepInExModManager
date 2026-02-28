<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { useGameStore } from '@/store/gameStore'
import { useSettingsStore } from '@/store/settingsStore'
import { useI18n } from '@/i18n'
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

const { t } = useI18n()

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
        <!-- 操作按钮固定在顶部 -->
        <BaseButton
          variant="ghost"
          size="sm"
          :disabled="loadingReleases"
          @click="loadReleases"
        >
          <Loader v-if="loadingReleases" :size="13" class="spin" />
          <RefreshCw v-else :size="13" />
          {{ t.bepinex.refreshList }}
        </BaseButton>
        <BaseButton
          size="sm"
          :disabled="!selectedRelease || installing || loadingReleases"
          :loading="installing"
          @click="install"
        >
          <Download :size="13" />
          {{ installed ? t.bepinex.reinstall : t.bepinex.install }}
        </BaseButton>
        <BaseButton
          v-if="installed"
          variant="danger"
          size="sm"
          :disabled="installing"
          @click="uninstall"
        >
          <Trash2 :size="13" />
          {{ t.bepinex.uninstall }}
        </BaseButton>
      </template>
    </AppTopBar>

    <!-- 进度条贴着 TopBar 下方固定显示 -->
    <div v-if="installing" class="progress-strip">
      <div class="progress-strip__fill" :style="{ width: `${progress}%` }" />
      <span class="progress-strip__text">{{ statusText }} {{ progress }}%</span>
    </div>

    <div class="page__body">
      <!-- Error -->
      <div v-if="error" class="error-bar">
        <AlertCircle :size="13" />
        <span>{{ error }}</span>
        <button class="error-bar__close" @click="error = ''">×</button>
      </div>

      <!-- 左右双栏：状态 + 版本列表 -->
      <div class="two-col">
        <!-- 左：当前状态 -->
        <div class="col-left">
          <h3 class="section__title">{{ t.bepinex.currentStatus }}</h3>
          <div class="status-card">
            <div class="status-card__row">
              <span class="text-secondary text-sm">{{ t.bepinex.installed }}</span>
              <BaseBadge :variant="installed ? 'success' : 'danger'">
                {{ installed ? t.bepinex.installed : t.bepinex.notInstalled }}
              </BaseBadge>
            </div>
            <div v-if="installed" class="status-card__row">
              <span class="text-secondary text-sm">{{ t.bepinex.version }}</span>
              <span class="text-sm font-mono">{{ installedVersion || t.common.unknown }}</span>
            </div>
            <div class="status-card__row">
              <span class="text-secondary text-sm">{{ t.bepinex.gameDir }}</span>
              <span class="text-xs text-muted truncate" style="max-width:180px">{{ game?.path }}</span>
            </div>

            <!-- 完整性检查 -->
            <div v-if="integrity" class="status-card__row status-card__row--integrity">
              <span class="text-secondary text-sm">{{ t.bepinex.integrity }}</span>
              <div class="integrity-grid">
                <div
                  v-for="item in integrityItems"
                  :key="item.label"
                  class="integrity-item"
                  :class="item.ok ? 'integrity-item--ok' : (item.required ? 'integrity-item--missing' : 'integrity-item--optional')"
                >
                  <span class="integrity-item__dot" />
                  <span class="integrity-item__label">{{ item.label }}</span>
                  <span v-if="!item.required" class="integrity-item__tag">{{ t.common.unknown }}</span>
                </div>
                <div class="integrity-score">
                  {{ integrity.score }} / 5
                  <span class="text-xs text-muted">{{ t.bepinex.integrityDesc }}</span>
                </div>
              </div>
            </div>
          </div>

          <!-- 安装完成提示 -->
          <p v-if="statusText && !installing" class="status-done text-sm text-muted">
            {{ statusText }}
          </p>
        </div>

        <!-- 右：版本列表 -->
        <div class="col-right">
          <h3 class="section__title">{{ t.bepinex.selectVersion }}</h3>
          <p class="release-hint">{{ t.bepinex.versionHint }}</p>

          <div v-if="loadingReleases" class="releases-loading">
            <Loader :size="16" class="spin text-muted" />
            <span class="text-sm text-muted">{{ t.bepinex.fetching }}</span>
          </div>

          <div v-else-if="releases.length === 0" class="releases-loading">
            <AlertCircle :size="16" class="text-muted" />
            <span class="text-sm text-muted">{{ t.bepinex.fetchFailed }}</span>
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
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.page { display: flex; flex-direction: column; height: 100%; }

/* 进度条贴着 TopBar 下方 */
.progress-strip {
  position: relative;
  height: 28px;
  background: var(--color-surface);
  border-bottom: 1px solid var(--color-border);
  display: flex;
  align-items: center;
  flex-shrink: 0;
  overflow: hidden;
}
.progress-strip__fill {
  position: absolute;
  left: 0; top: 0; bottom: 0;
  background: var(--color-accent);
  opacity: 0.12;
  transition: width 200ms ease;
}
.progress-strip__text {
  position: relative;
  font-size: var(--text-xs);
  color: var(--color-text-secondary);
  padding: 0 var(--space-4);
}

.page__body {
  flex: 1;
  overflow-y: auto;
  padding: var(--space-5);
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
  margin-bottom: var(--space-4);
}
.error-bar__close { margin-left: auto; cursor: pointer; font-size: var(--text-md); color: inherit; }

/* 左右双栏布局 */
.two-col {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--space-5);
  align-items: start;
}

.section__title {
  font-size: var(--text-xs);
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
  gap: 4px;
}

.integrity-item {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: var(--text-xs);
}

.integrity-item__dot {
  width: 6px;
  height: 6px;
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
  padding: 0 3px;
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

.status-done {
  margin-top: var(--space-3);
}

.release-hint {
  font-size: var(--text-xs);
  color: var(--color-text-muted);
  margin-bottom: var(--space-3);
  line-height: 1.6;
}
.release-hint strong {
  color: var(--color-text-secondary);
  font-weight: 600;
}

/* 版本列表 */
.releases-loading {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-4) 0;
}

.release-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
  max-height: 320px;
  overflow-y: auto;
}

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

.spin { animation: spin 1s linear infinite; }
@keyframes spin { to { transform: rotate(360deg); } }
</style>
