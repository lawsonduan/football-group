export type Position = 'fw' | 'mf' | 'df' | 'gk'

/** Attribute ratings for one position. */
export interface PlayerStats {
  pac: number
  sho: number
  pas: number
  dri: number
  def: number
  phy: number
}

/** One position record returned by the backend. */
export interface PlayerPosition extends PlayerStats {
  id: number
  player_id: number
  position: Position
  sort_order: number
}

/** Input shape when creating/updating positions (id/sort_order not required). */
export interface PositionInput extends PlayerStats {
  position: Position
}

/** A player with all their position records. */
export interface Player {
  id: number
  name: string
  avatar: string | null
  positions: PlayerPosition[]
}

export interface Report {
  date: string
  word_count: number
  content: string
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
