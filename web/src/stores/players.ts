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

  async function add(name: string, position: string) {
    const p = await playersApi.create(name, position)
    players.value.push(p)
    return p
  }

  async function update(id: number, name: string, position: string) {
    const p = await playersApi.update(id, name, position)
    const idx = players.value.findIndex((x) => x.id === id)
    if (idx !== -1) players.value[idx] = p
    return p
  }

  async function remove(id: number) {
    await playersApi.delete(id)
    players.value = players.value.filter((p) => p.id !== id)
  }

  function findById(id: number) {
    return players.value.find((p) => p.id === id) ?? null
  }

  return { players, loading, error, load, add, update, remove, findById }
})
