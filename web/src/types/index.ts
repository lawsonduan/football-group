export type Position = 'fw' | 'mf' | 'df' | 'gk' | 'all' | 'none'

export interface Player {
  id: number
  name: string
  position: Position
  avatar: string | null
}

export interface Game {
  id: number
  game_date: string
  team_a_score: number
  team_b_score: number
  notes: string | null
  created_at: string
}

export interface GamePlayer {
  id: number
  game_id: number
  player_id: number | null
  player_name: string
  team: 'A' | 'B'
  score: number
}

export interface GameDetail extends Game {
  players: GamePlayer[]
}

export interface CreateGamePayload {
  game_date: string
  team_a_score: number
  team_b_score: number
  notes: string | null
  players: {
    player_id: number
    player_name: string
    team: 'A' | 'B'
    score: number
  }[]
}
