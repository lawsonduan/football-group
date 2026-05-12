import type { Player, PlayerPosition, Game, GameDetail, CreateGamePayload, Report } from '../types'
import { load, save, nextId } from '../lib/storage'

// ─────────────────────────────────────────────────
// Players — read-only, sourced from players.csv
// ─────────────────────────────────────────────────

async function fetchPlayersFromCsv(): Promise<Player[]> {
  const res = await fetch(import.meta.env.BASE_URL + 'players.csv')
  if (!res.ok) return []
  const text = await res.text()

  const players: Player[] = []
  const nameIndex = new Map<string, number>()

  for (let i = 1; i < text.split('\n').length; i++) {
    const line = text.split('\n')[i].trim()
    if (!line) continue

    const f = line.split(',')
    if (f.length < 8) continue

    const name     = f[0].trim()
    const position = f[1].trim() as PlayerPosition['position']
    const pac = parseInt(f[2]) || 50
    const sho = parseInt(f[3]) || 50
    const pas = parseInt(f[4]) || 50
    const dri = parseInt(f[5]) || 50
    const def = parseInt(f[6]) || 50
    const phy = parseInt(f[7]) || 50

    if (nameIndex.has(name)) {
      const idx = nameIndex.get(name)!
      const p = players[idx]
      p.positions.push({
        id: p.positions.length,
        player_id: p.id,
        position, pac, sho, pas, dri, def, phy,
        sort_order: p.positions.length,
      })
    } else {
      const id = nameIndex.size + 1
      nameIndex.set(name, players.length)
      players.push({
        id,
        name,
        avatar: null,
        positions: [{ id: 0, player_id: id, position, pac, sho, pas, dri, def, phy, sort_order: 0 }],
      })
    }
  }

  return players
}

export const playersApi = {
  list: fetchPlayersFromCsv,
}

// ─────────────────────────────────────────────────
// Games — persisted in localStorage
// ─────────────────────────────────────────────────

function getGames(): GameDetail[] {
  return load<GameDetail[]>('games', [])
}
function setGames(g: GameDetail[]): void {
  save('games', g)
}

export const gamesApi = {
  list: async (): Promise<Game[]> =>
    getGames().map(({ players: _p, ...g }) => g),

  get: async (id: number): Promise<GameDetail> => {
    const game = getGames().find((g) => g.id === id)
    if (!game) throw new Error(`Game ${id} not found`)
    return game
  },

  create: async (payload: CreateGamePayload): Promise<GameDetail> => {
    const games = getGames()
    const id = nextId('game')
    const game: GameDetail = {
      id,
      game_date: payload.game_date,
      team_a_score: payload.team_a_score,
      team_b_score: payload.team_b_score,
      notes: payload.notes,
      created_at: new Date().toISOString(),
      players: payload.players.map((p) => ({
        id: nextId('game_player'),
        game_id: id,
        player_id: p.player_id,
        player_name: p.player_name,
        team: p.team,
        score: p.score,
      })),
    }
    games.unshift(game)
    setGames(games)
    return game
  },

  update: async (
    id: number,
    data: { team_a_score?: number; team_b_score?: number; notes?: string },
  ): Promise<Game> => {
    const games = getGames()
    const idx = games.findIndex((g) => g.id === id)
    if (idx === -1) throw new Error(`Game ${id} not found`)
    games[idx] = { ...games[idx], ...data }
    setGames(games)
    const { players: _p, ...game } = games[idx]
    return game
  },
}

// ─────────────────────────────────────────────────
// Reports — read-only, sourced from reports.csv
// ─────────────────────────────────────────────────

function splitCsvRow(line: string): string[] {
  const fields: string[] = []
  let cur = ''
  let inQuote = false
  for (let i = 0; i < line.length; i++) {
    const ch = line[i]
    if (ch === '"') {
      if (inQuote && line[i + 1] === '"') { cur += '"'; i++ }
      else inQuote = !inQuote
    } else if (ch === ',' && !inQuote) {
      fields.push(cur); cur = ''
    } else {
      cur += ch
    }
  }
  fields.push(cur)
  return fields
}

export const reportsApi = {
  list: async (): Promise<Report[]> => {
    try {
      const res = await fetch(import.meta.env.BASE_URL + 'reports.csv')
      if (!res.ok) return []
      const text = await res.text()
      const reports: Report[] = []

      for (const line of text.split('\n').slice(1)) {
        if (!line.trim()) continue
        const f = splitCsvRow(line)
        if (f.length < 3) continue

        const datetime  = f[0].trim()
        const wordCount = parseInt(f[1].trim()) || 0
        const content   = f[2].trim()
        if (!content) continue

        const date = datetime.split(/[ T]/)[0] || datetime
        reports.push({ date, word_count: wordCount, content })
      }

      return reports.sort((a, b) => b.date.localeCompare(a.date))
    } catch {
      return []
    }
  },
}
