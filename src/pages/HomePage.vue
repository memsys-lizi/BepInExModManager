<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useGameStore } from '@/store/gameStore'
import AppTopBar from '@/components/layout/AppTopBar.vue'
import BaseButton from '@/components/ui/BaseButton.vue'
import BaseModal from '@/components/ui/BaseModal.vue'
import BaseInput from '@/components/ui/BaseInput.vue'
import { FileCog, Gamepad2, Trash2, FolderOpen } from 'lucide-vue-next'
import { open } from '@tauri-apps/plugin-dialog'
import { validateGameDir, extractExeIcon } from '@/api'
import type { Game } from '@/types'

const router = useRouter()
const gameStore = useGameStore()

const showAddModal = ref(false)
const gameToDelete = ref<Game | null>(null)
const newGame = ref({ name: '', path: '', exeName: '' })
const validationMsg = ref('')
const validationOk = ref<boolean | null>(null)

function resetForm() {
  newGame.value = { name: '', path: '', exeName: '' }
  validationMsg.value = ''
  validationOk.value = null
}

// 选择 exe 文件，自动提取目录、exe名、游戏名
async function pickExe() {
  const selected = await open({
    title: '选择游戏可执行文件',
    filters: [{ name: '可执行文件', extensions: ['exe'] }],
  }) as string | null

  if (!selected) return

  const normalized = selected.replace(/\\/g, '/')
  const parts = normalized.split('/')
  const exe = parts[parts.length - 1]                     // Game.exe
  const dir = parts.slice(0, -1).join('\\')              // D:\...\GameName
  const folderName = parts[parts.length - 2] ?? ''       // GameName

  newGame.value.path    = dir
  newGame.value.exeName = exe
  if (!newGame.value.name) newGame.value.name = folderName

  // 验证目录
  validationMsg.value = '正在验证...'
  validationOk.value = null
  const result = await validateGameDir(dir)
  validationOk.value = result.valid
  validationMsg.value = result.message
}

async function addGame() {
  if (!newGame.value.path) return

  // 尝试从 exe 提取图标（失败不影响添加）
  let iconBase64: string | undefined
  if (newGame.value.exeName) {
    const exeFull = `${newGame.value.path}\\${newGame.value.exeName}`
    iconBase64 = await extractExeIcon(exeFull).catch(() => undefined)
  }

  const game: Game = {
    id: Date.now().toString(),
    name: newGame.value.name || newGame.value.path,
    path: newGame.value.path,
    exeName: newGame.value.exeName,
    addedAt: Date.now(),
    iconBase64,
  }
  gameStore.addGame(game)
  gameStore.setActiveGame(game.id)
  showAddModal.value = false
  resetForm()
  router.push({ name: 'game-detail', params: { id: game.id } })
}

function openAddModal() {
  resetForm()
  showAddModal.value = true
}

function navigateToGame(game: Game) {
  gameStore.setActiveGame(game.id)
  router.push({ name: 'game-detail', params: { id: game.id } })
}

function confirmDelete(game: Game, e: MouseEvent) {
  e.stopPropagation()
  gameToDelete.value = game
}

function doDeleteGame() {
  if (gameToDelete.value) {
    gameStore.removeGame(gameToDelete.value.id)
    gameToDelete.value = null
  }
}
</script>

<template>
  <div class="page">
    <AppTopBar />

    <div class="page__body">
      <!-- Empty state -->
      <div v-if="gameStore.games.length === 0" class="empty-state">
        <Gamepad2 :size="40" class="empty-state__icon" />
        <h2 class="empty-state__title">尚未添加任何游戏</h2>
        <p class="empty-state__desc">添加游戏后，即可安装 BepInEx 并管理 Mod</p>
        <BaseButton @click="openAddModal">添加游戏</BaseButton>
      </div>

      <!-- Game grid -->
      <div v-else class="game-grid">
        <button
          v-for="game in gameStore.games"
          :key="game.id"
          class="game-card"
          @click="navigateToGame(game)"
        >
          <div class="game-card__header">
            <img
              v-if="game.iconBase64"
              :src="game.iconBase64"
              class="game-card__icon-img"
              alt=""
              draggable="false"
            />
            <Gamepad2 v-else :size="20" class="game-card__icon" />
            <button
              class="game-card__del"
              title="移除游戏"
              @click="confirmDelete(game, $event)"
            >
              <Trash2 :size="12" />
            </button>
          </div>
          <div class="game-card__info">
            <div class="game-card__name truncate">{{ game.name }}</div>
            <div class="game-card__path truncate text-muted text-xs">{{ game.path }}</div>
          </div>
        </button>

        <!-- Add card -->
        <button class="game-card game-card--add" @click="openAddModal">
          <span class="game-card__add-icon">+</span>
          <span class="text-sm text-muted">添加游戏</span>
        </button>
      </div>
    </div>

    <!-- Add game modal -->
    <BaseModal v-if="showAddModal" title="添加游戏" @close="showAddModal = false">
      <div class="form">
        <div class="form__field">
          <label class="form__label">选择游戏可执行文件 *</label>
          <div class="form__row">
            <BaseInput
              v-model="newGame.exeName"
              :placeholder="newGame.path ? newGame.exeName : '点击浏览选择 .exe 文件'"
              readonly
            />
            <BaseButton variant="ghost" size="sm" @click="pickExe">
              <FileCog :size="13" />
              浏览
            </BaseButton>
          </div>
          <!-- 目录路径（只读展示） -->
          <div v-if="newGame.path" class="form__hint">
            <FolderOpen :size="11" />
            <span class="truncate">{{ newGame.path }}</span>
          </div>
          <!-- 验证结果 -->
          <div
            v-if="validationMsg"
            class="form__validation"
            :class="validationOk ? 'form__validation--ok' : 'form__validation--warn'"
          >
            {{ validationMsg }}
          </div>
        </div>

        <div class="form__field">
          <label class="form__label">游戏名称</label>
          <BaseInput v-model="newGame.name" placeholder="留空则自动从目录名获取" />
        </div>
      </div>
      <template #footer>
        <BaseButton variant="ghost" @click="showAddModal = false">取消</BaseButton>
        <BaseButton :disabled="!newGame.path" @click="addGame">添加</BaseButton>
      </template>
    </BaseModal>

    <!-- Delete confirm modal -->
    <BaseModal
      v-if="gameToDelete"
      title="移除游戏"
      width="340px"
      @close="gameToDelete = null"
    >
      <p class="text-secondary text-sm">
        确定要从列表中移除 <strong class="text-primary">{{ gameToDelete.name }}</strong> 吗？<br/>
        <span class="text-muted">不会删除任何游戏文件。</span>
      </p>
      <template #footer>
        <BaseButton variant="ghost" @click="gameToDelete = null">取消</BaseButton>
        <BaseButton variant="danger" @click="doDeleteGame">移除</BaseButton>
      </template>
    </BaseModal>
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
}

/* Empty state */
.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--space-3);
  text-align: center;
}
.empty-state__icon  { color: var(--color-text-muted); }
.empty-state__title { font-size: var(--text-xl); }
.empty-state__desc  { color: var(--color-text-secondary); font-size: var(--text-sm); margin-bottom: var(--space-2); }

/* Game grid */
.game-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(190px, 1fr));
  gap: var(--space-3);
  align-content: start;
}

.game-card {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  padding: var(--space-3) var(--space-4);
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  text-align: left;
  transition: border-color var(--transition-fast), background var(--transition-fast);
}
.game-card:hover { border-color: var(--color-border-2); background: var(--color-surface-2); }

.game-card__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}
.game-card__icon { color: var(--color-text-muted); }
.game-card__icon-img { width: 20px; height: 20px; object-fit: contain; border-radius: 3px; }
.game-card__del {
  display: flex;
  align-items: center;
  padding: 3px;
  border-radius: var(--radius-sm);
  color: var(--color-text-muted);
  opacity: 0;
  transition: opacity var(--transition-fast), background var(--transition-fast), color var(--transition-fast);
}
.game-card:hover .game-card__del { opacity: 1; }
.game-card__del:hover { background: var(--color-danger-bg); color: var(--color-danger); }

.game-card__info { min-width: 0; }
.game-card__name { font-size: var(--text-sm); font-weight: 500; color: var(--color-text-primary); }
.game-card__path { margin-top: 2px; }

.game-card--add {
  align-items: center;
  justify-content: center;
  border-style: dashed;
  border-color: var(--color-border-2);
  min-height: 80px;
  gap: var(--space-1);
}
.game-card--add:hover { border-color: var(--color-text-muted); }
.game-card__add-icon { font-size: 20px; color: var(--color-text-muted); }

/* Form */
.form { display: flex; flex-direction: column; gap: var(--space-4); }
.form__field { display: flex; flex-direction: column; gap: var(--space-1); }
.form__label { font-size: var(--text-sm); color: var(--color-text-secondary); }
.form__row { display: flex; gap: var(--space-2); align-items: center; }

.form__hint {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: var(--text-xs);
  color: var(--color-text-muted);
  min-width: 0;
}
.form__hint .truncate { flex: 1; }

.form__validation {
  font-size: var(--text-xs);
  padding: 3px 6px;
  border-radius: var(--radius-sm);
}
.form__validation--ok   { color: var(--color-success); background: var(--color-success-bg); }
.form__validation--warn { color: var(--color-warning); background: var(--color-warning-bg); }
</style>
