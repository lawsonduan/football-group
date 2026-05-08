import type { Player, PlayerStats, Game, GameDetail, CreateGamePayload } from '../types'

async function request<T>(url: string, options?: RequestInit): Promise<T> {
  const res = await fetch('/api' + url, {
    headers: { 'Content-Type': 'application/json' },
    ...options,
  })
  if (res.status === 204) return null as T
  const data = await res.json()
  if (!res.ok) throw new Error(data.error ?? `HTTP ${res.status}`)
  return data as T
}

export const playersApi = {
  list: () => request<Player[]>('/players'),

  create: (name: string, position: string, stats?: Partial<PlayerStats>) =>
    request<Player>('/players', {
      method: 'POST',
      body: JSON.stringify({ name, position, ...stats }),
    }),

  update: (id: number, name: string, position: string, stats?: Partial<PlayerStats>) =>
    request<Player>(`/players/${id}`, {
      method: 'PUT',
      body: JSON.stringify({ name, position, ...stats }),
    }),

  delete: (id: number) => request<null>(`/players/${id}`, { method: 'DELETE' }),

  /** Upload (or replace) a player's avatar. Returns the updated Player. */
  uploadAvatar: async (id: number, file: File): Promise<Player> => {
    const form = new FormData()
    form.append('avatar', file)
    // No Content-Type header — browser sets it with the correct multipart boundary
    const res = await fetch(`/api/players/${id}/avatar`, { method: 'PUT', body: form })
    const data = await res.json()
    if (!res.ok) throw new Error(data.error ?? `HTTP ${res.status}`)
    return data as Player
  },
}

export const gamesApi = {
  list: () => request<Game[]>('/games'),

  get: (id: number) => request<GameDetail>(`/games/${id}`),

  create: (payload: CreateGamePayload) =>
    request<GameDetail>('/games', {
      method: 'POST',
      body: JSON.stringify(payload),
    }),

  update: (id: number, data: { team_a_score?: number; team_b_score?: number; notes?: string }) =>
    request<Game>(`/games/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    }),
}
