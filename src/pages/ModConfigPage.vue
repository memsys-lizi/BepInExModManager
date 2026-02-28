<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { useGameStore } from '@/store/gameStore'
import { useModStore } from '@/store/modStore'
import AppTopBar from '@/components/layout/AppTopBar.vue'
import BaseButton from '@/components/ui/BaseButton.vue'
import BaseToggle from '@/components/ui/BaseToggle.vue'
import { Save, AlertCircle, Loader } from 'lucide-vue-next'
import { listConfigFiles, readConfigFile, writeConfigFile, type ConfigSectionRaw } from '@/api'

const route = useRoute()
const gameStore = useGameStore()
const modStore = useModStore()

const gameId = computed(() => route.params.id as string)
const modId = computed(() => route.params.modId as string)

const game = computed(() => gameStore.games.find(g => g.id === gameId.value))
const mod = computed(() => modStore.mods.find(m => m.id === modId.value))

// 当前打开的 cfg 文件路径
const cfgFiles = ref<string[]>([])
const selectedCfg = ref<string>('')
const sections = ref<ConfigSectionRaw[]>([])

const loading = ref(false)
const saving = ref(false)
const saved = ref(false)
const error = ref('')

async function loadCfgList() {
  if (!game.value) return
  const files = await listConfigFiles(game.value.path)
  cfgFiles.value = files

  // 尝试匹配当前 mod 名称
  if (mod.value) {
    const match = files.find(f =>
      f.toLowerCase().includes(mod.value!.name.toLowerCase())
    )
    selectedCfg.value = match ?? files[0] ?? ''
  } else {
    selectedCfg.value = files[0] ?? ''
  }

  if (selectedCfg.value) await loadCfg()
}

async function loadCfg() {
  if (!selectedCfg.value) return
  loading.value = true
  error.value = ''
  try {
    sections.value = await readConfigFile(selectedCfg.value)
  } catch (e: any) {
    error.value = String(e)
  } finally {
    loading.value = false
  }
}

async function saveConfig() {
  if (!selectedCfg.value) return
  saving.value = true
  error.value = ''
  try {
    await writeConfigFile(selectedCfg.value, sections.value)
    saved.value = true
    setTimeout(() => { saved.value = false }, 2000)
  } catch (e: any) {
    error.value = String(e)
  } finally {
    saving.value = false
  }
}

// 文件名简写
function baseName(path: string) {
  return path.split(/[\\/]/).pop() ?? path
}

onMounted(loadCfgList)
</script>

<template>
  <div class="page">
    <AppTopBar>
      <template #actions>
        <BaseButton size="sm" :loading="saving" @click="saveConfig">
          <Save :size="13" />
          {{ saved ? '已保存' : '保存' }}
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

      <!-- Cfg file selector -->
      <div v-if="cfgFiles.length > 1" class="file-select-bar">
        <span class="text-muted text-sm">配置文件：</span>
        <select v-model="selectedCfg" class="file-select" @change="loadCfg">
          <option v-for="f in cfgFiles" :key="f" :value="f">{{ baseName(f) }}</option>
        </select>
      </div>

      <!-- No config -->
      <div v-if="!loading && sections.length === 0" class="empty-state">
        <AlertCircle :size="20" class="text-muted" />
        <span class="text-secondary text-sm">
          {{ cfgFiles.length === 0
            ? '未找到配置文件（需先运行一次游戏生成）'
            : '配置文件为空' }}
        </span>
      </div>

      <!-- Loading -->
      <div v-else-if="loading" class="empty-state">
        <Loader :size="20" class="text-muted spin" />
        <span class="text-secondary text-sm">读取中...</span>
      </div>

      <!-- Config sections -->
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
  </div>
</template>

<style scoped>
.page { display: flex; flex-direction: column; height: 100%; }

.page__body {
  flex: 1;
  overflow-y: auto;
  padding: var(--space-4) var(--space-5);
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
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
}
.error-bar__close { margin-left: auto; cursor: pointer; font-size: var(--text-md); color: inherit; }

.file-select-bar {
  display: flex;
  align-items: center;
  gap: var(--space-2);
}
.file-select {
  flex: 1;
  max-width: 300px;
  padding: 4px 8px;
  border-radius: var(--radius-sm);
  font-size: var(--text-sm);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--space-2);
  padding: var(--space-6);
}

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
.config-row__key { font-size: var(--text-sm); font-family: var(--font-mono); color: var(--color-text-primary); }
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
