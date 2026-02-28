import { defineStore } from 'pinia'
import { ref, computed, watch } from 'vue'
import type { Game } from '@/types'

const STORAGE_KEY = 'bmm-games'

function loadGames(): Game[] {
  try {
    const raw = localStorage.getItem(STORAGE_KEY)
    return raw ? JSON.parse(raw) : []
  } catch {
    return []
  }
}

function saveGames(games: Game[]) {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(games))
}

export const useGameStore = defineStore('game', () => {
  const games = ref<Game[]>(loadGames())
  const activeGameId = ref<string | null>(games.value[0]?.id ?? null)

  const activeGame = computed(() =>
    games.value.find(g => g.id === activeGameId.value) ?? null
  )

  function addGame(game: Game) {
    games.value.push(game)
  }

  function removeGame(id: string) {
    games.value = games.value.filter(g => g.id !== id)
    if (activeGameId.value === id) {
      activeGameId.value = games.value[0]?.id ?? null
    }
  }

  function setActiveGame(id: string) {
    activeGameId.value = id
  }

  // 自动持久化
  watch(games, (val) => saveGames(val), { deep: true })

  return { games, activeGameId, activeGame, addGame, removeGame, setActiveGame }
})
