import type { Player, PlayerStats } from '../types'

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

// Position-weighted OVR — mirrors the backend default_stats logic
const OVR_WEIGHTS: Record<string, number[]> = {
  //          pac sho pas dri def phy
  fw:   [2,   3,  1,  2,  0,  1], // Σ=9
  mf:   [1,   1,  3,  2,  1,  2], // Σ=10
  df:   [1,   0,  1,  1,  3,  2], // Σ=8
  gk:   [1,   0,  1,  0,  3,  2], // Σ=7
  all:  [1,   1,  2,  2,  1,  2], // Σ=9
}

export function calcOvr(p: Pick<Player, StatKey | 'position'>): number {
  const vals: number[] = STAT_KEYS.map((k) => p[k])
  const w = OVR_WEIGHTS[p.position] ?? [1, 1, 1, 1, 1, 1]
  const wSum = w.reduce((a, b) => a + b, 0)
  return Math.round(vals.reduce((s, v, i) => s + v * w[i], 0) / wSum)
}

/** Color for individual stat value (0-99). */
export function statColor(v: number): string {
  if (v >= 80) return '#34a853' // green
  if (v >= 65) return '#0d9e6e' // teal
  if (v >= 50) return '#f9a825' // amber
  return '#ea4335'               // red
}

/** Background color for OVR badge. */
export function ovrBg(ovr: number): string {
  if (ovr >= 80) return '#c9a227' // gold
  if (ovr >= 70) return '#1a73e8' // blue
  if (ovr >= 60) return '#34a853' // green
  return '#9e9e9e'                 // grey
}
