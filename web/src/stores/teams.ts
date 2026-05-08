import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Player } from '../types'

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

    const byPos = (pos: string) => sel.filter((p) => p.position === pos)
    const fw = byPos('fw'), mf = byPos('mf'), df = byPos('df')
    const gk = byPos('gk'), all = byPos('all')
    const none = sel.filter((p) => p.position === 'none' || !p.position)

    const a: Player[] = [], b: Player[] = []
    const distribute = (list: Player[], preferA = true) => {
      shuffle(list).forEach((p, i) => {
        ;(preferA ? i % 2 === 0 : i % 2 === 1) ? a.push(p) : b.push(p)
      })
    }

    if (gk.length) distribute(gk, true)
    distribute(fw, true)
    distribute(mf, false)
    distribute(df, true)
    distribute(all, false)
    distribute(none, true)

    while (Math.abs(a.length - b.length) > 1) {
      a.length > b.length + 1 ? b.push(a.pop()!) : a.push(b.pop()!)
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
