import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Player } from '../types'
import { calcOvr } from '../utils/stats'

function shuffle<T>(arr: T[]): T[] {
  const a = [...arr]
  for (let i = a.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1))
    ;[a[i], a[j]] = [a[j], a[i]]
  }
  return a
}

export const useTeamStore = defineStore('teams', () => {
  // Using an array so Vue reactivity tracks mutations correctly.
  // Expose a Set-shaped computed for O(1) lookups.
  const selectedIdList = ref<number[]>([])
  const teamA = ref<Player[]>([])
  const teamB = ref<Player[]>([])
  const isGrouped = ref(false)

  const selectedIds = computed(() => new Set(selectedIdList.value))

  function load(existingIds: Set<number>) {
    const saved = localStorage.getItem('football_selected')
    if (saved) {
      const all: number[] = JSON.parse(saved)
      selectedIdList.value = all.filter((id) => existingIds.has(id))
    }
    const a = localStorage.getItem('football_teamA')
    const b = localStorage.getItem('football_teamB')
    const grouped = localStorage.getItem('football_isGrouped')
    if (a && b) {
      teamA.value = JSON.parse(a)
      teamB.value = JSON.parse(b)
      isGrouped.value = grouped === 'true'
    }
  }

  function persistSelection() {
    localStorage.setItem('football_selected', JSON.stringify(selectedIdList.value))
  }

  function persistTeams() {
    localStorage.setItem('football_teamA', JSON.stringify(teamA.value))
    localStorage.setItem('football_teamB', JSON.stringify(teamB.value))
    localStorage.setItem('football_isGrouped', JSON.stringify(isGrouped.value))
  }

  function toggleSelect(id: number) {
    const idx = selectedIdList.value.indexOf(id)
    if (idx === -1) selectedIdList.value.push(id)
    else selectedIdList.value.splice(idx, 1)
    persistSelection()
  }

  function selectAll(players: Player[]) {
    const existing = new Set(selectedIdList.value)
    players.forEach((p) => { if (!existing.has(p.id)) selectedIdList.value.push(p.id) })
    persistSelection()
  }

  function clearSelection() {
    selectedIdList.value = []
    persistSelection()
  }

  function randomize(players: Player[]) {
    const sel = players.filter((p) => selectedIds.value.has(p.id))
    if (sel.length === 0) return

    const primaryPos = (p: Player) => p.positions[0]?.position ?? ''

    // Sort by OVR descending; small random jitter (±2) so similar-rated players
    // swap order occasionally, giving different splits on repeated runs.
    const sortByOvr = (list: Player[]): Player[] =>
      [...list].sort((x, y) => calcOvr(y) - calcOvr(x) + (Math.random() - 0.5) * 4)

    // Snake draft: A, B, B, A, A, B, B, A …
    // For players sorted best→worst this mathematically minimises the OVR gap.
    const snakeDraft = (sorted: Player[], a: Player[], b: Player[]) => {
      sorted.forEach((p, i) => {
        ;(i % 4 === 0 || i % 4 === 3) ? a.push(p) : b.push(p)
      })
    }

    const byPos = (pos: string) => sortByOvr(sel.filter((p) => primaryPos(p) === pos))
    const fw   = byPos('fw')
    const mf   = byPos('mf')
    const df   = byPos('df')
    const gk   = byPos('gk')
    const none = sortByOvr(sel.filter((p) => !primaryPos(p)))

    const a: Player[] = []
    const b: Player[] = []

    // GK special case: exactly 2 → one per team (randomly which gets the better GK)
    if (gk.length === 2) {
      Math.random() < 0.5
        ? (a.push(gk[0]), b.push(gk[1]))
        : (a.push(gk[1]), b.push(gk[0]))
    } else {
      snakeDraft(gk, a, b)
    }

    snakeDraft(fw, a, b)
    snakeDraft(mf, a, b)
    snakeDraft(df, a, b)
    snakeDraft(none, a, b)

    // Balance head-count (edge cases with odd totals)
    while (Math.abs(a.length - b.length) > 1) {
      a.length > b.length + 1 ? b.push(a.pop()!) : a.push(b.pop()!)
    }

    // OVR swap optimisation: try exchanging same-position players to shrink the
    // total OVR gap. Keeps position balance intact.
    let aSum = a.reduce((s, p) => s + calcOvr(p), 0)
    let bSum = b.reduce((s, p) => s + calcOvr(p), 0)
    let improved = true
    while (improved) {
      improved = false
      outer: for (let i = 0; i < a.length; i++) {
        for (let j = 0; j < b.length; j++) {
          if (primaryPos(a[i]) !== primaryPos(b[j])) continue
          const oA = calcOvr(a[i]), oB = calcOvr(b[j])
          const newASum = aSum - oA + oB
          const newBSum = bSum - oB + oA
          if (Math.abs(newASum - newBSum) < Math.abs(aSum - bSum)) {
            ;[a[i], b[j]] = [b[j], a[i]]
            aSum = newASum
            bSum = newBSum
            improved = true
            break outer
          }
        }
      }
    }

    teamA.value = shuffle(a)
    teamB.value = shuffle(b)
    isGrouped.value = true
    persistTeams()
  }

  function unlock() {
    isGrouped.value = false
    persistTeams()
  }

  function reset() {
    selectedIdList.value = []
    teamA.value = []
    teamB.value = []
    isGrouped.value = false
    persistSelection()
    persistTeams()
  }

  return {
    selectedIdList,
    selectedIds,
    teamA,
    teamB,
    isGrouped,
    load,
    toggleSelect,
    selectAll,
    clearSelection,
    randomize,
    unlock,
    reset,
  }
})
