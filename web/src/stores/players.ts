import { defineStore } from 'pinia'
import { ref } from 'vue'
import { playersApi } from '../api'
import type { Player } from '../types'

export const usePlayerStore = defineStore('players', () => {
  const players = ref<Player[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function load() {
    loading.value = true
    error.value = null
    try {
      players.value = await playersApi.list()
    } catch (e) {
      error.value = (e as Error).message
    } finally {
      loading.value = false
    }
  }

  function findById(id: number) {
    return players.value.find((p) => p.id === id) ?? null
  }

  return { players, loading, error, load, findById }
})
