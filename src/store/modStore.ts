import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { ModInfo } from '@/types'

export const useModStore = defineStore('mod', () => {
  const mods = ref<ModInfo[]>([])
  const loading = ref(false)

  function getModsByGame(gameId: string) {
    return computed(() => mods.value.filter(m => m.gameId === gameId))
  }

  function setMods(gameId: string, incoming: ModInfo[]) {
    mods.value = [
      ...mods.value.filter(m => m.gameId !== gameId),
      ...incoming,
    ]
  }

  function toggleMod(modId: string) {
    const mod = mods.value.find(m => m.id === modId)
    if (mod) {
      mod.status = mod.status === 'enabled' ? 'disabled' : 'enabled'
    }
  }

  function removeMod(modId: string) {
    mods.value = mods.value.filter(m => m.id !== modId)
  }

  return { mods, loading, getModsByGame, setMods, toggleMod, removeMod }
})
