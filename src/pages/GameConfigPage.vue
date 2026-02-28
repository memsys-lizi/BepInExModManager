<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { useGameStore } from '@/store/gameStore'
import { useI18n } from '@/i18n'
import AppTopBar from '@/components/layout/AppTopBar.vue'
import BaseToggle from '@/components/ui/BaseToggle.vue'
import { AlertCircle, Loader, Check, FileText } from 'lucide-vue-next'
import { listConfigFiles, readConfigFile, writeConfigFile, type ConfigSectionRaw } from '@/api'

const route = useRoute()
const gameStore = useGameStore()
const { t } = useI18n()

const gameId = computed(() => route.params.id as string)
const game = computed(() => gameStore.games.find(g => g.id === gameId.value))

const cfgFiles = ref<string[]>([])
const selectedCfg = ref<string>('')
const sections = ref<ConfigSectionRaw[]>([])

const loading = ref(false)
const saveState = ref<'idle' | 'saving' | 'saved'>('idle')
const error = ref('')

// 当前文件是否是 BepInEx.cfg
const isBepInExCfg = computed(() =>
  baseName(selectedCfg.value).toLowerCase() === 'bepinex.cfg'
)

// 获取字段说明：优先用 section.key 组合键，不存在则退回到只用 key
function getFieldDesc(sectionName: string, key: string): string {
  const fd = t.value.config.fieldDesc as Record<string, string>
  // 针对 Logging.Console / Logging.Disk 下的 Enabled / LogLevels 特殊处理
  const sectionKey = `${sectionName}.${key}`
  return fd[sectionKey] ?? fd[key] ?? ''
}

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
        <span v-if="saveState === 'saving'" class="save-indicator save-indicator--saving">
          <Loader :size="12" class="spin" />
          {{ t.common.saving }}
        </span>
        <span v-else-if="saveState === 'saved'" class="save-indicator save-indicator--saved">
          <Check :size="12" />
          {{ t.common.saved }}
        </span>
      </template>
    </AppTopBar>

    <div class="page__body">
      <div v-if="error" class="error-bar">
        <AlertCircle :size="13" />
        <span>{{ error }}</span>
        <button class="error-bar__close" @click="error = ''">×</button>
      </div>

      <div v-if="!loading && cfgFiles.length === 0" class="empty-state">
        <FileText :size="28" class="text-muted" />
        <p class="text-secondary text-sm">{{ t.config.noFiles }}</p>
        <p class="text-muted text-xs">{{ t.config.noFilesHint }}</p>
      </div>

      <template v-else>
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
              <span class="text-secondary text-sm">{{ t.config.reading }}</span>
            </div>

            <div v-else-if="sections.length === 0" class="empty-state">
              <span class="text-muted text-sm">{{ t.config.empty }}</span>
            </div>

            <div v-else class="sections">
              <!-- BepInEx.cfg 提示横幅 -->
              <div v-if="isBepInExCfg" class="bep-cfg-banner">
                <span class="bep-cfg-banner__title">BepInEx 核心配置</span>
                <span class="bep-cfg-banner__desc">
                  修改前请了解各项含义，悬停字段名可查看说明。大多数情况保持默认即可。
                </span>
              </div>

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
                    :title="getFieldDesc(section.name, entry.key) || undefined"
                  >
                    <div class="config-row__left">
                      <span class="config-row__key">{{ entry.key }}</span>
                      <!-- 字段说明：BepInEx.cfg 用 i18n 翻译，其他 cfg 用原始描述 -->
                      <span
                        v-if="isBepInExCfg && getFieldDesc(section.name, entry.key)"
                        class="config-row__desc text-xs text-muted"
                      >
                        {{ getFieldDesc(section.name, entry.key) }}
                      </span>
                      <span
                        v-else-if="!isBepInExCfg && entry.description"
                        class="config-row__desc text-xs text-muted"
                      >
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
                        class="config-ctrl config-ctrl--select"
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
.error-bar__close { margin-left: auto; font-size: var(--text-md); color: inherit; }

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

/* BepInEx.cfg 提示横幅 */
.bep-cfg-banner {
  display: flex;
  flex-direction: column;
  gap: 3px;
  padding: var(--space-3) var(--space-4);
  background: var(--color-surface-2);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  border-left: 3px solid var(--color-accent);
}
.bep-cfg-banner__title {
  font-size: var(--text-sm);
  font-weight: 600;
  color: var(--color-text-primary);
}
.bep-cfg-banner__desc {
  font-size: var(--text-xs);
  color: var(--color-text-muted);
  line-height: 1.5;
}

.sections { display: flex; flex-direction: column; gap: var(--space-4); }

.config-section__name {
  font-size: var(--text-xs);
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
  transition: background var(--transition-fast);
}
.config-row:last-child { border-bottom: none; }
.config-row:hover { background: var(--color-surface-2); }

.config-row__left {
  display: flex;
  flex-direction: column;
  gap: 3px;
  flex: 1;
  min-width: 0;
}
.config-row__key  {
  font-size: var(--text-sm);
  font-family: var(--font-mono);
  color: var(--color-text-primary);
}
.config-row__desc {
  line-height: 1.5;
  max-width: 380px;
}
.config-row__control { flex-shrink: 0; }

.config-ctrl {
  padding: 4px 8px;
  border-radius: var(--radius-sm);
  font-size: var(--text-sm);
  width: 140px;
  border: 1px solid var(--color-border);
  background: var(--color-surface-2);
  color: var(--color-text-primary);
  transition: border-color var(--transition-fast);
}
.config-ctrl:focus {
  outline: none;
  border-color: var(--color-border-2);
}
.config-ctrl--select {
}

.spin { animation: spin 1s linear infinite; }
@keyframes spin { to { transform: rotate(360deg); } }
</style>
