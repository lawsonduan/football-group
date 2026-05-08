import { defineStore } from 'pinia'
import { ref } from 'vue'
import { gamesApi } from '../api'
import type { Game, GameDetail, CreateGamePayload } from '../types'

export const useGameStore = defineStore('games', () => {
  const games = ref<Game[]>([])

  async function load() {
    games.value = await gamesApi.list()
  }

  async function create(payload: CreateGamePayload) {
    await gamesApi.create(payload)
    await load()
  }

  async function getDetail(id: number): Promise<GameDetail> {
    return gamesApi.get(id)
  }

  return { games, load, create, getDetail }
})
