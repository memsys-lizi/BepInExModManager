<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useGameStore } from '@/store/gameStore'
import { useModStore } from '@/store/modStore'
import AppTopBar from '@/components/layout/AppTopBar.vue'
import BaseButton from '@/components/ui/BaseButton.vue'
import BaseBadge from '@/components/ui/BaseBadge.vue'
import BaseToggle from '@/components/ui/BaseToggle.vue'
import BaseModal from '@/components/ui/BaseModal.vue'
import {
  Download, RefreshCw, Trash2, Settings2, FolderOpen, PackageOpen, AlertCircle, Loader
} from 'lucide-vue-next'
import type { ModInfo, BepInExInfo } from '@/types'
import {
  checkBepInExStatus, scanMods, enableMod, disableMod, deleteMod, openPluginsDir,
} from '@/api'

const route = useRoute()
const router = useRouter()
const gameStore = useGameStore()
const modStore = useModStore()

const gameId = computed(() => route.params.id as string)
const game = computed(() => gameStore.games.find(g => g.id === gameId.value))

const bepinexInfo = ref<BepInExInfo>({ installed: false })
const loadingMods = ref(false)
const modToDelete = ref<ModInfo | null>(null)
const searchText = ref('')
const error = ref('')

const gameMods = computed(() => {
  const allMods = modStore.getModsByGame(gameId.value).value
  if (!searchText.value) return allMods
  const q = searchText.value.toLowerCase()
  return allMods.filter(m => m.name.toLowerCase().includes(q))
})

async function refresh() {
  if (!game.value) return
  loadingMods.value = true
  error.value = ''
  try {
    // 检测 BepInEx
    const status = await checkBepInExStatus(game.value.path)
    bepinexInfo.value = {
      installed: status.installed,
      version: status.version,
      path: status.path,
    }

    // 扫描 Mods
    const rawMods = await scanMods(game.value.path)
    const mods: ModInfo[] = rawMods.map(m => ({
      id: m.id,
      name: m.name,
      fileName: m.file_name,
      filePath: m.file_path,
      status: m.status,
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
      const newPath = await disableMod(mod.filePath)
      mod.filePath = newPath
      mod.fileName = newPath.split(/[\\/]/).pop() ?? mod.fileName
    } else {
      const newPath = await enableMod(mod.filePath)
      mod.filePath = newPath
      mod.fileName = newPath.split(/[\\/]/).pop() ?? mod.fileName
    }
    modStore.toggleMod(mod.id)
  } catch (e: any) {
    error.value = String(e)
  }
}

function openModConfig(mod: ModInfo) {
  router.push({ name: 'mod-config', params: { id: gameId.value, modId: mod.id } })
}

function confirmDelete(mod: ModInfo) {
  modToDelete.value = mod
}

async function doDeleteMod() {
  if (!modToDelete.value) return
  try {
    await deleteMod(modToDelete.value.filePath)
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

onMounted(refresh)
</script>

<template>
  <div class="page">
    <AppTopBar>
      <template #actions>
        <BaseButton variant="ghost" size="sm" :disabled="loadingMods" @click="refresh">
          <Loader v-if="loadingMods" :size="13" class="spin" />
          <RefreshCw v-else :size="13" />
          刷新
        </BaseButton>
        <BaseButton variant="ghost" size="sm" @click="openFolder">
          <FolderOpen :size="13" />
          打开目录
        </BaseButton>
      </template>
    </AppTopBar>

    <div v-if="!game" class="page__body page__body--center">
      <p class="text-secondary">游戏不存在</p>
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
          <PackageOpen :size="14" />
          <span class="text-sm">BepInEx</span>
          <BaseBadge v-if="bepinexInfo.installed" variant="success">
            已安装{{ bepinexInfo.version ? ' ' + bepinexInfo.version : '' }}
          </BaseBadge>
          <BaseBadge v-else variant="danger">未安装</BaseBadge>
        </div>
        <div class="bep-bar__actions">
          <BaseButton
            v-if="!bepinexInfo.installed"
            size="sm"
            @click="router.push({ name: 'bepinex-installer', params: { id: gameId } })"
          >
            <Download :size="13" />
            安装 BepInEx
          </BaseButton>
          <BaseButton
            v-else
            variant="ghost"
            size="sm"
            @click="router.push({ name: 'bepinex-installer', params: { id: gameId } })"
          >
            管理
          </BaseButton>
        </div>
      </div>

      <!-- Mod list header -->
      <div class="list-header">
        <input v-model="searchText" class="list-header__search" placeholder="搜索 Mod..." />
        <span class="text-muted text-sm">{{ gameMods.length }} 个 Mod</span>
      </div>

      <!-- Mod list -->
      <div class="mod-list">
        <div v-if="loadingMods" class="mod-empty">
          <Loader :size="18" class="text-muted spin" />
          <span class="text-secondary text-sm">正在扫描...</span>
        </div>

        <div v-else-if="gameMods.length === 0" class="mod-empty">
          <AlertCircle :size="18" class="text-muted" />
          <span class="text-secondary text-sm">
            {{ bepinexInfo.installed
              ? '暂无 Mod，将 .dll 文件放入 plugins 文件夹后点击刷新'
              : '请先安装 BepInEx' }}
          </span>
        </div>

        <div
          v-for="mod in gameMods"
          :key="mod.id"
          class="mod-item"
          :class="{ 'mod-item--disabled': mod.status === 'disabled' }"
        >
          <BaseToggle
            :model-value="mod.status === 'enabled'"
            @update:model-value="toggleMod(mod)"
          />
          <div class="mod-item__info">
            <div class="mod-item__name truncate">{{ mod.name }}</div>
            <div class="mod-item__meta text-xs text-muted">
              <span v-if="mod.version">v{{ mod.version }}</span>
              <span v-if="mod.author">· {{ mod.author }}</span>
              <span>· {{ mod.fileName }}</span>
            </div>
          </div>
          <div class="mod-item__actions">
            <button class="icon-btn" title="配置" @click="openModConfig(mod)">
              <Settings2 :size="13" />
            </button>
            <button class="icon-btn icon-btn--danger" title="删除" @click="confirmDelete(mod)">
              <Trash2 :size="13" />
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Delete confirm modal -->
    <BaseModal v-if="modToDelete" title="删除 Mod" width="340px" @close="modToDelete = null">
      <p class="text-secondary text-sm">
        确定要删除 <strong class="text-primary">{{ modToDelete.name }}</strong> 吗？此操作将永久删除文件，不可撤销。
      </p>
      <template #footer>
        <BaseButton variant="ghost" @click="modToDelete = null">取消</BaseButton>
        <BaseButton variant="danger" @click="doDeleteMod">删除</BaseButton>
      </template>
    </BaseModal>
  </div>
</template>

<style scoped>
.page { display: flex; flex-direction: column; height: 100%; }

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
.error-bar__close {
  margin-left: auto;
  color: inherit;
  font-size: var(--text-md);
  cursor: pointer;
}

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
.bep-bar__info { display: flex; align-items: center; gap: var(--space-2); }
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

.mod-item {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-3) var(--space-5);
  border-bottom: 1px solid var(--color-border);
  transition: background var(--transition-fast);
}
.mod-item:hover { background: var(--color-surface); }
.mod-item--disabled { opacity: 0.5; }

.mod-item__info { flex: 1; min-width: 0; }
.mod-item__name { font-size: var(--text-sm); font-weight: 500; color: var(--color-text-primary); }
.mod-item__meta { margin-top: 2px; display: flex; gap: 4px; }

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
  cursor: pointer;
  transition: background var(--transition-fast), color var(--transition-fast);
}
.icon-btn:hover { background: var(--color-surface-2); color: var(--color-text-primary); }
.icon-btn--danger:hover { background: var(--color-danger-bg); color: var(--color-danger); }

/* Spin animation */
.spin { animation: spin 1s linear infinite; }
@keyframes spin { to { transform: rotate(360deg); } }
</style>
