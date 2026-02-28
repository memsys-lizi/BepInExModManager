import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { ModInfo } from '@/types'

export const useModStore = defineStore('mod', () => {
  const mods = ref<ModInfo[]>([])

  function getModsByGame(gameId: string) {
    return computed(() => mods.value.filter(m => m.gameId === gameId))
  }

  function setMods(gameId: string, incoming: ModInfo[]) {
    mods.value = [
      ...mods.value.filter(m => m.gameId !== gameId),
      ...incoming,
    ]
  }

  function updateMod(modId: string, patch: Partial<ModInfo>) {
    const mod = mods.value.find(m => m.id === modId)
    if (mod) Object.assign(mod, patch)
  }

  function removeMod(modId: string) {
    mods.value = mods.value.filter(m => m.id !== modId)
  }

  return { mods, getModsByGame, setMods, updateMod, removeMod }
})
