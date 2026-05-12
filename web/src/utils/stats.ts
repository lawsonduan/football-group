import type { Player, PlayerPosition, PositionInput, PlayerStats } from '../types'

export const STAT_KEYS = ['pac', 'sho', 'pas', 'dri', 'def', 'phy'] as const
export type StatKey = (typeof STAT_KEYS)[number]

export const STAT_LABELS: Record<StatKey, string> = {
  pac: 'PAC',
  sho: 'SHO',
  pas: 'PAS',
  dri: 'DRI',
  def: 'DEF',
  phy: 'PHY',
}

export const DEFAULT_STATS: PlayerStats = {
  pac: 50,
  sho: 50,
  pas: 50,
  dri: 50,
  def: 50,
  phy: 50,
}

export const DEFAULT_STATS_BY_POS: Record<string, PlayerStats> = {
  //           pac sho pas dri def phy
  fw: { pac: 75, sho: 75, pas: 60, dri: 70, def: 40, phy: 65 },
  mf: { pac: 65, sho: 60, pas: 75, dri: 65, def: 55, phy: 65 },
  df: { pac: 65, sho: 45, pas: 60, dri: 55, def: 75, phy: 70 },
  gk: { pac: 55, sho: 30, pas: 55, dri: 45, def: 75, phy: 65 },
}

// Position-weighted OVR
const OVR_WEIGHTS: Record<string, number[]> = {
  //     pac sho pas dri def phy
  fw:   [2,   3,  1,  2,  0,  1], // Σ=9
  mf:   [1,   1,  3,  2,  1,  2], // Σ=10
  df:   [1,   0,  1,  1,  3,  2], // Σ=8
  gk:   [1,   0,  1,  0,  3,  2], // Σ=7
}

/** OVR for a specific position record. */
export function calcOvrForPos(pos: Pick<PlayerPosition | PositionInput, StatKey | 'position'>): number {
  const vals: number[] = STAT_KEYS.map((k) => pos[k])
  const w = OVR_WEIGHTS[pos.position] ?? [1, 1, 1, 1, 1, 1]
  const wSum = w.reduce((a, b) => a + b, 0)
  return Math.round(vals.reduce((s, v, i) => s + v * w[i], 0) / wSum)
}

/** OVR for a player — uses their primary (first) position. */
export function calcOvr(player: Player): number {
  const pos = player.positions[0]
  return pos ? calcOvrForPos(pos) : 50
}

/** Color for an individual stat value (0-99). */
export function statColor(v: number): string {
  if (v >= 80) return '#34a853'
  if (v >= 65) return '#0d9e6e'
  if (v >= 50) return '#f9a825'
  return '#ea4335'
}

/** Background color for an OVR badge. */
export function ovrBg(ovr: number): string {
  if (ovr >= 80) return '#c9a227'
  if (ovr >= 70) return '#1a73e8'
  if (ovr >= 60) return '#34a853'
  return '#9e9e9e'
}
