<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { useGameStore } from '@/store/gameStore'
import AppTopBar from '@/components/layout/AppTopBar.vue'
import BaseToggle from '@/components/ui/BaseToggle.vue'
import { AlertCircle, Loader, Check, FileText } from 'lucide-vue-next'
import { listConfigFiles, readConfigFile, writeConfigFile, type ConfigSectionRaw } from '@/api'

const route = useRoute()
const gameStore = useGameStore()

const gameId = computed(() => route.params.id as string)
const game = computed(() => gameStore.games.find(g => g.id === gameId.value))

const cfgFiles = ref<string[]>([])
const selectedCfg = ref<string>('')
const sections = ref<ConfigSectionRaw[]>([])

const loading = ref(false)
const saveState = ref<'idle' | 'saving' | 'saved'>('idle')
const error = ref('')

// debounce 自动保存
let saveTimer: ReturnType<typeof setTimeout> | null = null

function scheduleSave() {
  saveState.value = 'idle'
  if (saveTimer) clearTimeout(saveTimer)
  saveTimer = setTimeout(doSave, 800)
}

async function doSave() {
  if (!selectedCfg.value || sections.value.length === 0) return
  saveState.value = 'saving'
  try {
    await writeConfigFile(selectedCfg.value, sections.value)
    saveState.value = 'saved'
    setTimeout(() => { saveState.value = 'idle' }, 1500)
  } catch (e: any) {
    error.value = String(e)
    saveState.value = 'idle'
  }
}

// 监听所有字段变化，触发自动保存
watch(sections, scheduleSave, { deep: true })

async function loadCfgList() {
  if (!game.value) return
  error.value = ''
  try {
    const files = await listConfigFiles(game.value.path)
    cfgFiles.value = files
    selectedCfg.value = files[0] ?? ''
    if (selectedCfg.value) await loadCfg()
  } catch (e: any) {
    error.value = String(e)
  }
}

async function loadCfg() {
  if (!selectedCfg.value) return
  loading.value = true
  error.value = ''
  // 切换文件时取消待保存的定时器
  if (saveTimer) { clearTimeout(saveTimer); saveTimer = null }
  saveState.value = 'idle'
  try {
    sections.value = await readConfigFile(selectedCfg.value)
  } catch (e: any) {
    error.value = String(e)
  } finally {
    loading.value = false
  }
}

function baseName(path: string) {
  return path.split(/[\\/]/).pop() ?? path
}

onMounted(loadCfgList)
</script>

<template>
  <div class="page">
    <AppTopBar>
      <template #actions>
        <!-- 自动保存状态指示 -->
        <span v-if="saveState === 'saving'" class="save-indicator save-indicator--saving">
          <Loader :size="12" class="spin" />
          保存中...
        </span>
        <span v-else-if="saveState === 'saved'" class="save-indicator save-indicator--saved">
          <Check :size="12" />
          已保存
        </span>
      </template>
    </AppTopBar>

    <div class="page__body">
      <!-- Error -->
      <div v-if="error" class="error-bar">
        <AlertCircle :size="13" />
        <span>{{ error }}</span>
        <button class="error-bar__close" @click="error = ''">×</button>
      </div>

      <!-- 无配置文件 -->
      <div v-if="!loading && cfgFiles.length === 0" class="empty-state">
        <FileText :size="28" class="text-muted" />
        <p class="text-secondary text-sm">未找到配置文件</p>
        <p class="text-muted text-xs">运行一次游戏后，BepInEx 会自动生成各 Mod 的配置文件</p>
      </div>

      <template v-else>
        <!-- 文件列表 + 内容区 两栏布局 -->
        <div class="layout">
          <!-- 左侧文件列表 -->
          <aside class="file-list">
            <div
              v-for="f in cfgFiles"
              :key="f"
              class="file-item"
              :class="{ 'file-item--active': f === selectedCfg }"
              @click="selectedCfg = f; loadCfg()"
            >
              <FileText :size="12" />
              <span class="file-item__name truncate">{{ baseName(f) }}</span>
            </div>
          </aside>

          <!-- 右侧配置内容 -->
          <main class="cfg-content">
            <div v-if="loading" class="empty-state">
              <Loader :size="20" class="text-muted spin" />
              <span class="text-secondary text-sm">读取中...</span>
            </div>

            <div v-else-if="sections.length === 0" class="empty-state">
              <span class="text-muted text-sm">配置文件为空</span>
            </div>

            <div v-else class="sections">
              <div
                v-for="section in sections"
                :key="section.name"
                class="config-section"
              >
                <div class="config-section__name">{{ section.name }}</div>

                <div class="config-table">
                  <div
                    v-for="entry in section.entries"
                    :key="entry.key"
                    class="config-row"
                  >
                    <div class="config-row__left">
                      <span class="config-row__key">{{ entry.key }}</span>
                      <span v-if="entry.description" class="config-row__desc text-xs text-muted">
                        {{ entry.description }}
                      </span>
                    </div>

                    <div class="config-row__control">
                      <BaseToggle
                        v-if="entry.entry_type === 'boolean'"
                        :model-value="entry.value.toLowerCase() === 'true'"
                        @update:model-value="(v: boolean | undefined) => entry.value = String(!!v)"
                      />
                      <select
                        v-else-if="entry.entry_type === 'enum' && entry.options"
                        v-model="entry.value"
                        class="config-ctrl"
                      >
                        <option v-for="opt in entry.options" :key="opt" :value="opt">{{ opt }}</option>
                      </select>
                      <input
                        v-else
                        v-model="entry.value"
                        class="config-ctrl"
                        :type="entry.entry_type === 'number' ? 'number' : 'text'"
                      />
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </main>
        </div>
      </template>
    </div>
  </div>
</template>

<style scoped>
.page { display: flex; flex-direction: column; height: 100%; }

.page__body {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.error-bar {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-4);
  background: var(--color-danger-bg);
  color: var(--color-danger);
  font-size: var(--text-sm);
  flex-shrink: 0;
}
.error-bar__close { margin-left: auto; cursor: pointer; font-size: var(--text-md); color: inherit; }

/* 自动保存状态 */
.save-indicator {
  display: flex;
  align-items: center;
  gap: 5px;
  font-size: var(--text-xs);
  padding: 3px 8px;
  border-radius: var(--radius-sm);
}
.save-indicator--saving { color: var(--color-text-muted); }
.save-indicator--saved  { color: var(--color-success); background: var(--color-success-bg); }

/* Empty */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--space-2);
  height: 100%;
  padding: var(--space-6);
  text-align: center;
}

/* 两栏布局 */
.layout {
  flex: 1;
  display: flex;
  overflow: hidden;
}

/* 左侧文件列表 */
.file-list {
  width: 180px;
  flex-shrink: 0;
  border-right: 1px solid var(--color-border);
  overflow-y: auto;
  padding: var(--space-2) 0;
}

.file-item {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-3);
  cursor: pointer;
  font-size: var(--text-xs);
  color: var(--color-text-secondary);
  transition: background var(--transition-fast), color var(--transition-fast);
  min-width: 0;
}
.file-item:hover { background: var(--color-surface-2); color: var(--color-text-primary); }
.file-item--active {
  background: var(--color-accent-dim);
  color: var(--color-text-primary);
  font-weight: 500;
}
.file-item__name { flex: 1; min-width: 0; }

/* 右侧内容 */
.cfg-content {
  flex: 1;
  overflow-y: auto;
  padding: var(--space-4) var(--space-5);
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.sections { display: flex; flex-direction: column; gap: var(--space-4); }

.config-section__name {
  font-size: var(--text-sm);
  font-weight: 600;
  color: var(--color-text-muted);
  text-transform: uppercase;
  letter-spacing: 0.06em;
  margin-bottom: var(--space-2);
}

.config-table {
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  overflow: hidden;
}

.config-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-3) var(--space-4);
  border-bottom: 1px solid var(--color-border);
  gap: var(--space-4);
}
.config-row:last-child { border-bottom: none; }

.config-row__left {
  display: flex;
  flex-direction: column;
  gap: 2px;
  flex: 1;
  min-width: 0;
}
.config-row__key  { font-size: var(--text-sm); font-family: var(--font-mono); color: var(--color-text-primary); }
.config-row__desc { line-height: 1.4; }
.config-row__control { flex-shrink: 0; }

.config-ctrl {
  padding: 4px 8px;
  border-radius: var(--radius-sm);
  font-size: var(--text-sm);
  width: 140px;
}

.spin { animation: spin 1s linear infinite; }
@keyframes spin { to { transform: rotate(360deg); } }
</style>
