<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useGameStore } from '@/store/gameStore'
import { useModStore } from '@/store/modStore'
import { useI18n } from '@/i18n'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { open as openDialog } from '@tauri-apps/plugin-dialog'
import AppTopBar from '@/components/layout/AppTopBar.vue'
import BaseButton from '@/components/ui/BaseButton.vue'
import BaseToggle from '@/components/ui/BaseToggle.vue'
import BaseModal from '@/components/ui/BaseModal.vue'
import {
  Download, RefreshCw, Trash2, FolderOpen, PackageOpen, PackagePlus,
  AlertCircle, Loader, Check, ChevronDown, ChevronRight, SlidersHorizontal,
  Folder, File, Play, Link,
} from 'lucide-vue-next'
import type { ModInfo } from '@/types'
import type { ConflictStrategy } from '@/api'
import {
  checkBepInExStatus, scanMods, enableMod, disableMod, deleteMod,
  openPluginsDir, installMod, installModFromUrl, launchGame,
} from '@/api'

const route = useRoute()
const router = useRouter()
const gameStore = useGameStore()
const modStore = useModStore()
const { t } = useI18n()

const gameId = computed(() => route.params.id as string)
const game = computed(() => gameStore.games.find(g => g.id === gameId.value))

const bepInstalled = ref(false)
const bepVersion = ref('')
const loadingMods = ref(false)
const modToDelete = ref<ModInfo | null>(null)
const searchText = ref('')
const error = ref('')
const expanded = ref<Set<string>>(new Set())

const gameMods = computed(() => {
  const all = modStore.getModsByGame(gameId.value).value
  if (!searchText.value) return all
  const q = searchText.value.toLowerCase()
  return all.filter(m => m.name.toLowerCase().includes(q))
})

function toggleExpand(modId: string) {
  if (expanded.value.has(modId)) expanded.value.delete(modId)
  else expanded.value.add(modId)
}

async function refresh() {
  if (!game.value) return
  loadingMods.value = true
  error.value = ''
  try {
    const status = await checkBepInExStatus(game.value.path)
    bepInstalled.value = status.installed
    bepVersion.value = status.version ?? ''

    const rawMods = await scanMods(game.value.path)
    const mods: ModInfo[] = rawMods.map(m => ({
      id: m.id,
      name: m.name,
      modPath: m.mod_path,
      isFolder: m.is_folder,
      status: m.status,
      dlls: m.dlls.map(d => ({
        name: d.name,
        fileName: d.file_name,
        filePath: d.file_path,
        status: d.status,
      })),
      gameId: gameId.value,
    }))
    modStore.setMods(gameId.value, mods)
  } catch (e: any) {
    error.value = String(e)
  } finally {
    loadingMods.value = false
  }
}

async function toggleMod(mod: ModInfo) {
  try {
    if (mod.status === 'enabled') {
      const newPath = await disableMod(mod.modPath, mod.isFolder)
      modStore.updateMod(mod.id, {
        status: 'disabled',
        modPath: mod.isFolder ? mod.modPath : newPath,
        dlls: mod.dlls.map(d => ({ ...d, status: 'disabled' as const })),
      })
    } else {
      const newPath = await enableMod(mod.modPath, mod.isFolder)
      modStore.updateMod(mod.id, {
        status: 'enabled',
        modPath: mod.isFolder ? mod.modPath : newPath,
        dlls: mod.dlls.map(d => ({ ...d, status: 'enabled' as const })),
      })
    }
  } catch (e: any) {
    error.value = String(e)
  }
}

function confirmDelete(mod: ModInfo) { modToDelete.value = mod }

async function doDeleteMod() {
  if (!modToDelete.value) return
  try {
    await deleteMod(modToDelete.value.modPath, modToDelete.value.isFolder)
    modStore.removeMod(modToDelete.value.id)
    modToDelete.value = null
  } catch (e: any) {
    error.value = String(e)
    modToDelete.value = null
  }
}

function openFolder() {
  if (game.value) openPluginsDir(game.value.path)
}

// ── 启动游戏 ──────────────────────────────────────────────────
async function doLaunchGame() {
  if (!game.value) return
  try {
    await launchGame(game.value.path, game.value.exeName)
  } catch (e: any) {
    error.value = String(e)
  }
}

// ── 安装核心逻辑 ──────────────────────────────────────────────
const isDragging = ref(false)
const installing = ref(false)
const installResult = ref<{ name: string; success: boolean; msg: string } | null>(null)

// 冲突弹窗状态
const conflictInfo = ref<{ name: string; sourcePath: string; isUrl: boolean; urlStr?: string } | null>(null)

// URL 安装弹窗
const showUrlDialog = ref(false)
const urlInput = ref('')
const urlInstalling = ref(false)

let unlistenDrop: (() => void) | null = null

async function setupDragDrop() {
  const win = getCurrentWindow()
  unlistenDrop = await win.onDragDropEvent((event) => {
    if (!bepInstalled.value || !game.value) return
    if (event.payload.type === 'over') {
      isDragging.value = true
    } else if (event.payload.type === 'leave') {
      isDragging.value = false
    } else if (event.payload.type === 'drop') {
      isDragging.value = false
      const paths: string[] = (event.payload as any).paths ?? []
      const supported = paths.filter(p =>
        p.toLowerCase().endsWith('.zip') || p.toLowerCase().endsWith('.dll')
      )
      if (supported.length > 0) doInstallMod(supported[0])
    }
  })
}

async function pickAndInstall() {
  const selected = await openDialog({
    title: t.value.mods.pickModFile,
    filters: [{ name: t.value.mods.pickModFilter, extensions: ['zip', 'dll'] }],
    multiple: false,
  }) as string | null
  if (selected) doInstallMod(selected)
}

/** 执行安装（本地文件），strategy 由冲突弹窗传入 */
async function doInstallMod(sourcePath: string, strategy?: ConflictStrategy) {
  if (!game.value || installing.value) return
  installing.value = true
  installResult.value = null
  try {
    const result = await installMod(game.value.path, sourcePath, strategy)

    if (result.conflict) {
      // 需要用户决定
      conflictInfo.value = {
        name: result.mod_name,
        sourcePath,
        isUrl: false,
      }
      return
    }

    installResult.value = { name: result.mod_name, success: true, msg: t.value.mods.installSuccess }
    await refresh()
  } catch (e: any) {
    installResult.value = { name: sourcePath.split(/[\\/]/).pop() ?? '', success: false, msg: String(e) }
  } finally {
    installing.value = false
    if (installResult.value?.success) {
      setTimeout(() => { installResult.value = null }, 2500)
    }
  }
}

/** 执行 URL 安装 */
async function doInstallUrl(strategy?: ConflictStrategy) {
  const url = urlInput.value.trim()
  if (!url || !game.value) return
  urlInstalling.value = true
  showUrlDialog.value = false
  installing.value = true
  installResult.value = null
  try {
    const result = await installModFromUrl(game.value.path, url, strategy)
    if (result.conflict) {
      conflictInfo.value = { name: result.mod_name, sourcePath: '', isUrl: true, urlStr: url }
      return
    }
    installResult.value = { name: result.mod_name, success: true, msg: t.value.mods.installSuccessUrl }
    urlInput.value = ''
    await refresh()
  } catch (e: any) {
    installResult.value = { name: url, success: false, msg: String(e) }
  } finally {
    installing.value = false
    urlInstalling.value = false
    if (installResult.value?.success) {
      setTimeout(() => { installResult.value = null }, 2500)
    }
  }
}

/** 用户在冲突弹窗做出选择后 */
async function onConflictResolved(strategy: ConflictStrategy) {
  const info = conflictInfo.value
  conflictInfo.value = null
  if (!info || strategy === 'cancel') return

  if (info.isUrl && info.urlStr) {
    await doInstallUrl(strategy)
  } else {
    await doInstallMod(info.sourcePath, strategy)
  }
}

onMounted(async () => {
  await refresh()
  await setupDragDrop()
})

onUnmounted(() => { unlistenDrop?.() })
</script>

<template>
  <div class="page">
    <AppTopBar>
      <template #actions>
        <BaseButton variant="ghost" size="sm" :disabled="loadingMods" @click="refresh">
          <Loader v-if="loadingMods" :size="13" class="spin" />
          <RefreshCw v-else :size="13" />
          {{ t.common.refresh }}
        </BaseButton>
        <BaseButton
          v-if="bepInstalled"
          size="sm"
          :disabled="installing"
          @click="pickAndInstall"
        >
          <PackagePlus :size="13" />
          {{ t.mods.installMod }}
        </BaseButton>
        <BaseButton
          v-if="bepInstalled"
          variant="ghost"
          size="sm"
          :disabled="installing"
          @click="showUrlDialog = true"
        >
          <Link :size="13" />
          {{ t.mods.installUrl }}
        </BaseButton>
        <BaseButton variant="ghost" size="sm" @click="openFolder">
          <FolderOpen :size="13" />
          {{ t.mods.openFolder }}
        </BaseButton>
        <BaseButton variant="ghost" size="sm" @click="doLaunchGame">
          <Play :size="13" />
          {{ t.mods.launchGame }}
        </BaseButton>
      </template>
    </AppTopBar>

    <div v-if="!game" class="page__body page__body--center">
      <p class="text-secondary">{{ t.common.gameNotFound }}</p>
    </div>

    <div v-else class="page__body">
      <!-- Error -->
      <div v-if="error" class="error-bar">
        <AlertCircle :size="13" />
        <span>{{ error }}</span>
        <button class="error-bar__close" @click="error = ''">×</button>
      </div>

      <!-- BepInEx status bar -->
      <div class="bep-bar">
        <div class="bep-bar__info">
          <PackageOpen :size="14" class="bep-bar__icon" />
          <span class="bep-bar__name">BepInEx</span>
          <span v-if="bepInstalled" class="bep-bar__status bep-bar__status--ok">
            <span class="bep-bar__dot" />
            {{ bepVersion || t.home.bepInstalled }}
          </span>
          <span v-else class="bep-bar__status bep-bar__status--err">
            <span class="bep-bar__dot" />
            {{ t.home.bepNotInstalled }}
          </span>
        </div>
        <div class="bep-bar__actions">
          <BaseButton
            v-if="bepInstalled"
            variant="ghost"
            size="sm"
            @click="router.push({ name: 'game-config', params: { id: gameId } })"
          >
            <SlidersHorizontal :size="13" />
            {{ t.mods.openConfig }}
          </BaseButton>
          <BaseButton
            v-if="!bepInstalled"
            size="sm"
            @click="router.push({ name: 'bepinex-installer', params: { id: gameId } })"
          >
            <Download :size="13" />
            {{ t.mods.goInstall }}
          </BaseButton>
          <BaseButton
            v-else
            variant="ghost"
            size="sm"
            @click="router.push({ name: 'bepinex-installer', params: { id: gameId } })"
          >
            {{ t.bepinex.title }}
          </BaseButton>
        </div>
      </div>

      <!-- Mod list header -->
      <div class="list-header">
        <input v-model="searchText" class="list-header__search" :placeholder="t.mods.searchPlaceholder" />
        <span class="text-muted text-sm">{{ gameMods.length }} {{ t.mods.modCount }}</span>
      </div>

      <!-- Mod list -->
      <div class="mod-list">
        <div v-if="loadingMods" class="mod-empty">
          <Loader :size="18" class="text-muted spin" />
          <span class="text-secondary text-sm">{{ t.common.loading }}</span>
        </div>

        <div v-else-if="gameMods.length === 0" class="mod-empty">
          <AlertCircle :size="18" class="text-muted" />
          <span class="text-secondary text-sm">
            {{ bepInstalled ? t.mods.noModsHint : t.mods.bepNotInstalledHint }}
          </span>
        </div>

        <template v-else>
          <div
            v-for="mod in gameMods"
            :key="mod.id"
            class="mod-group"
            :class="{ 'mod-group--disabled': mod.status === 'disabled' }"
          >
            <!-- Mod 主行 -->
            <div class="mod-item">
              <!-- 展开箭头（文件夹 Mod 且有 dll 时显示） -->
              <button
                v-if="mod.isFolder && mod.dlls.length > 0"
                class="expand-btn"
                @click="toggleExpand(mod.id)"
              >
                <ChevronDown v-if="expanded.has(mod.id)" :size="13" />
                <ChevronRight v-else :size="13" />
              </button>
              <span v-else class="expand-placeholder" />

              <!-- 图标 -->
              <Folder v-if="mod.isFolder" :size="14" class="mod-item__type-icon" />
              <File   v-else              :size="14" class="mod-item__type-icon" />

              <!-- 启用/禁用 toggle -->
              <BaseToggle
                :model-value="mod.status === 'enabled'"
                @update:model-value="toggleMod(mod)"
              />

              <div class="mod-item__info">
                <div class="mod-item__name truncate">{{ mod.name }}</div>
                <div class="mod-item__meta text-xs text-muted">
                  <span v-if="mod.isFolder">{{ mod.dlls.length }} {{ t.mods.dllCount }}</span>
                  <span v-else>{{ t.mods.looseDll }}</span>
                </div>
              </div>

              <div class="mod-item__actions">
                <button
                  class="icon-btn icon-btn--danger"
                  :title="t.mods.deleteMod"
                  @click="confirmDelete(mod)"
                >
                  <Trash2 :size="13" />
                </button>
              </div>
            </div>

            <!-- 展开的 dll 列表 -->
            <div v-if="mod.isFolder && expanded.has(mod.id)" class="dll-list">
              <div
                v-for="dll in mod.dlls"
                :key="dll.filePath"
                class="dll-item"
                :class="{ 'dll-item--disabled': dll.status === 'disabled' }"
              >
                <File :size="11" class="dll-item__icon" />
                <span class="dll-item__name truncate text-xs">{{ dll.fileName }}</span>
                <span class="dll-status" :class="dll.status === 'enabled' ? 'dll-status--on' : 'dll-status--off'">
                  {{ dll.status === 'enabled' ? t.mods.enable : t.mods.disable }}
                </span>
              </div>
            </div>
          </div>
        </template>
      </div>
    </div>

    <!-- 拖拽覆盖层 -->
    <Transition name="drag-overlay">
      <div v-if="isDragging" class="drag-overlay">
        <div class="drag-overlay__inner">
          <PackagePlus :size="32" />
          <span>{{ t.mods.dragHint }}</span>
          <span class="drag-overlay__sub">{{ t.mods.dragSub }}</span>
        </div>
      </div>
    </Transition>

    <!-- 安装中 overlay -->
    <Transition name="drag-overlay">
      <div v-if="installing" class="drag-overlay drag-overlay--installing">
        <div class="drag-overlay__inner">
          <Loader :size="28" class="spin" />
          <span>{{ t.mods.installing }}</span>
        </div>
      </div>
    </Transition>

    <!-- 安装结果 toast -->
    <Transition name="toast">
      <div
        v-if="installResult"
        class="install-toast"
        :class="installResult.success ? 'install-toast--ok' : 'install-toast--err'"
        @click="installResult = null"
      >
        <Check v-if="installResult.success" :size="14" />
        <AlertCircle v-else :size="14" />
        <div class="install-toast__text">
          <span class="install-toast__name">{{ installResult.name }}</span>
          <span class="install-toast__msg">{{ installResult.msg }}</span>
        </div>
      </div>
    </Transition>

    <!-- Delete confirm modal -->
    <BaseModal v-if="modToDelete" :title="t.mods.deleteTitle" width="340px" @close="modToDelete = null">
      <p class="text-secondary text-sm">
        {{ t.mods.deleteModConfirm }} <strong class="text-primary">{{ modToDelete.name }}</strong>{{ t.mods.deleteModConfirmSuffix }}
        <template v-if="modToDelete.isFolder">
          <br/><span class="text-muted">{{ t.mods.deleteFolderHint }}</span>
        </template>
        <template v-else>
          <br/><span class="text-muted">{{ t.mods.deleteDllHint }}</span>
        </template>
      </p>
      <template #footer>
        <BaseButton variant="ghost" @click="modToDelete = null">{{ t.common.cancel }}</BaseButton>
        <BaseButton variant="danger" @click="doDeleteMod">{{ t.common.delete }}</BaseButton>
      </template>
    </BaseModal>

    <!-- URL 安装弹窗 -->
    <BaseModal v-if="showUrlDialog" :title="t.mods.urlTitle" width="420px" @close="showUrlDialog = false">
      <p class="text-muted text-xs" style="margin-bottom: var(--space-3);">
        {{ t.mods.urlHint }}
      </p>
      <input
        v-model="urlInput"
        class="url-input"
        :placeholder="t.mods.urlPlaceholder"
        @keydown.enter="doInstallUrl()"
      />
      <template #footer>
        <BaseButton variant="ghost" @click="showUrlDialog = false; urlInput = ''">{{ t.common.cancel }}</BaseButton>
        <BaseButton
          :disabled="!urlInput.trim() || urlInstalling"
          :loading="urlInstalling"
          @click="doInstallUrl()"
        >
          <Download :size="13" />
          {{ t.mods.urlDownload }}
        </BaseButton>
      </template>
    </BaseModal>

    <!-- 冲突处理弹窗 -->
    <BaseModal v-if="conflictInfo" :title="t.mods.conflictTitle" width="360px" @close="onConflictResolved('cancel')">
      <p class="text-secondary text-sm">
        {{ t.mods.conflictMsg }}
        <strong class="text-primary">{{ conflictInfo.name }}</strong>
        {{ t.mods.conflictMsgSuffix }}
      </p>
      <template #footer>
        <BaseButton variant="ghost" @click="onConflictResolved('cancel')">{{ t.common.cancel }}</BaseButton>
        <BaseButton variant="ghost" @click="onConflictResolved('rename')">{{ t.mods.conflictRename }}</BaseButton>
        <BaseButton variant="danger" @click="onConflictResolved('overwrite')">{{ t.mods.conflictOverwrite }}</BaseButton>
      </template>
    </BaseModal>
  </div>
</template>

<style scoped>
.page { display: flex; flex-direction: column; height: 100%; position: relative; }

.page__body {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  min-height: 0;
}
.page__body--center { align-items: center; justify-content: center; }

/* Error bar */
.error-bar {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-5);
  background: var(--color-danger-bg);
  color: var(--color-danger);
  font-size: var(--text-sm);
  flex-shrink: 0;
}
.error-bar__close { margin-left: auto; color: inherit; font-size: var(--text-md); }

/* BepInEx bar */
.bep-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-3) var(--space-5);
  border-bottom: 1px solid var(--color-border);
  background: var(--color-surface);
  flex-shrink: 0;
}
.bep-bar__info {
  display: flex;
  align-items: center;
  gap: var(--space-2);
}
.bep-bar__icon { color: var(--color-text-muted); flex-shrink: 0; }
.bep-bar__name {
  font-size: var(--text-sm);
  font-weight: 500;
  color: var(--color-text-primary);
}
.bep-bar__status {
  display: flex;
  align-items: center;
  gap: 5px;
  font-size: var(--text-xs);
  color: var(--color-text-muted);
}
.bep-bar__dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  flex-shrink: 0;
}
.bep-bar__status--ok  .bep-bar__dot { background: var(--color-success); }
.bep-bar__status--ok  { color: var(--color-text-secondary); }
.bep-bar__status--err .bep-bar__dot { background: var(--color-danger); }
.bep-bar__status--err { color: var(--color-danger); }
.bep-bar__actions { display: flex; gap: var(--space-2); }

/* List header */
.list-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-3);
  padding: var(--space-3) var(--space-5);
  border-bottom: 1px solid var(--color-border);
  flex-shrink: 0;
}
.list-header__search {
  flex: 1;
  max-width: 200px;
  padding: 4px 10px;
  border-radius: var(--radius-sm);
}

/* Mod list */
.mod-list { flex: 1; overflow-y: auto; }

.mod-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--space-2);
  height: 160px;
}

/* Mod group */
.mod-group { border-bottom: 1px solid var(--color-border); }
.mod-group--disabled { opacity: 0.55; }

/* Mod 主行 */
.mod-item {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-3) var(--space-4);
  transition: background var(--transition-fast);
}
.mod-item:hover { background: var(--color-surface); }

.expand-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 18px;
  height: 18px;
  color: var(--color-text-muted);
  border-radius: var(--radius-sm);
  flex-shrink: 0;
}
.expand-btn:hover { background: var(--color-surface-2); color: var(--color-text-primary); }
.expand-placeholder { width: 18px; flex-shrink: 0; }

.mod-item__type-icon { color: var(--color-text-muted); flex-shrink: 0; }
.mod-item__info { flex: 1; min-width: 0; }
.mod-item__name { font-size: var(--text-sm); font-weight: 500; color: var(--color-text-primary); }
.mod-item__meta { margin-top: 2px; }

.mod-item__actions {
  display: flex;
  gap: var(--space-1);
  opacity: 0;
  transition: opacity var(--transition-fast);
}
.mod-item:hover .mod-item__actions { opacity: 1; }

.icon-btn {
  display: flex;
  align-items: center;
  padding: 4px;
  border-radius: var(--radius-sm);
  color: var(--color-text-muted);
  transition: background var(--transition-fast), color var(--transition-fast);
}
.icon-btn:hover { background: var(--color-surface-2); color: var(--color-text-primary); }
.icon-btn--danger:hover { background: var(--color-danger-bg); color: var(--color-danger); }

/* Dll 展开列表 */
.dll-list {
  background: var(--color-surface);
  border-top: 1px solid var(--color-border);
  padding: var(--space-1) 0;
}

.dll-item {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: 5px var(--space-4) 5px 52px;
  transition: background var(--transition-fast);
}
.dll-item:hover { background: var(--color-surface-2); }
.dll-item--disabled { opacity: 0.5; }
.dll-item__icon { color: var(--color-text-muted); flex-shrink: 0; }
.dll-item__name { flex: 1; color: var(--color-text-secondary); }
.dll-status {
  font-size: var(--text-xs);
  flex-shrink: 0;
}
.dll-status--on  { color: var(--color-success); }
.dll-status--off { color: var(--color-text-muted); }

/* ── 拖拽覆盖层 ── */
.drag-overlay {
  position: absolute;
  inset: 0;
  z-index: 100;
  background: color-mix(in srgb, var(--color-bg) 85%, transparent);
  display: flex;
  align-items: center;
  justify-content: center;
  backdrop-filter: blur(2px);
}

.drag-overlay--installing {
  background: color-mix(in srgb, var(--color-bg) 90%, transparent);
}

.drag-overlay__inner {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-6) 48px;
  border: 2px dashed var(--color-border-2);
  border-radius: var(--radius-lg);
  color: var(--color-text-primary);
  font-size: var(--text-md);
  font-weight: 500;
  background: var(--color-surface);
}

.drag-overlay__sub {
  font-size: var(--text-xs);
  font-weight: 400;
  color: var(--color-text-muted);
}

/* 拖拽动画 */
.drag-overlay-enter-active,
.drag-overlay-leave-active { transition: opacity 150ms ease; }
.drag-overlay-enter-from,
.drag-overlay-leave-to    { opacity: 0; }

/* ── 安装结果 Toast ── */
.install-toast {
  position: absolute;
  bottom: var(--space-4);
  left: 50%;
  transform: translateX(-50%);
  z-index: 110;
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-4);
  border-radius: var(--radius-md);
  border: 1px solid var(--color-border);
  background: var(--color-surface);
  box-shadow: 0 4px 16px rgba(0,0,0,0.18);
  max-width: 360px;
  white-space: nowrap;
  overflow: hidden;
}

.install-toast--ok  { border-color: var(--color-success); color: var(--color-success); }
.install-toast--err { border-color: var(--color-danger);  color: var(--color-danger);  }

.install-toast__text {
  display: flex;
  flex-direction: column;
  gap: 1px;
  overflow: hidden;
}
.install-toast__name {
  font-size: var(--text-sm);
  font-weight: 500;
  color: var(--color-text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
}
.install-toast__msg {
  font-size: var(--text-xs);
  color: inherit;
}

.toast-enter-active,
.toast-leave-active { transition: opacity 200ms ease, transform 200ms ease; }
.toast-enter-from,
.toast-leave-to     { opacity: 0; transform: translateX(-50%) translateY(8px); }

/* URL 安装输入框 */
.url-input {
  width: 100%;
  padding: 6px 10px;
  border-radius: var(--radius-sm);
  font-size: var(--text-sm);
}

/* Spin */
.spin { animation: spin 1s linear infinite; }
@keyframes spin { to { transform: rotate(360deg); } }
</style>
