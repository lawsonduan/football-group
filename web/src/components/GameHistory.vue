<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useGameStore } from '../stores/games'
import type { GameDetail } from '../types'

const gameStore = useGameStore()

// Maps game id → loaded detail (null while loading)
const detailCache = ref<Record<number, GameDetail | null>>({})
const openIds = ref<Set<number>>(new Set())

onMounted(() => gameStore.load())

async function toggle(id: number) {
  if (openIds.value.has(id)) {
    openIds.value.delete(id)
    return
  }
  openIds.value.add(id)
  if (detailCache.value[id] === undefined) {
    detailCache.value[id] = null // loading sentinel
    try {
      detailCache.value[id] = await gameStore.getDetail(id)
    } catch {
      detailCache.value[id] = null
    }
  }
}

function winnerLabel(a: number, b: number) {
  if (a > b) return { text: 'A队胜', color: 'var(--team-a)' }
  if (b > a) return { text: 'B队胜', color: 'var(--team-b)' }
  return { text: '平局', color: 'var(--gray)' }
}
</script>

<template>
  <div class="card">
    <h2>📋 比赛记录</h2>

    <div v-if="!gameStore.games.length" class="empty-state">
      暂无比赛记录，完成分组后点击「💾 记录比赛」保存
    </div>

    <div v-else>
      <div
        v-for="g in gameStore.games"
        :key="g.id"
        class="game-history-item"
        @click="toggle(g.id)"
      >
        <!-- Summary row -->
        <div style="display: flex; justify-content: space-between; align-items: center; gap: 12px">
          <div style="min-width: 0">
            <div style="font-weight: 600">📅 {{ g.game_date }}</div>
            <div v-if="g.notes" style="font-size: 13px; color: var(--gray); margin-top: 2px">
              {{ g.notes }}
            </div>
          </div>

          <div style="text-align: center; flex-shrink: 0">
            <div class="game-score-display">
              <span style="color: var(--team-a)">{{ g.team_a_score }}</span>
              <span style="color: var(--gray); font-size: 1rem; margin: 0 6px">:</span>
              <span style="color: var(--team-b)">{{ g.team_b_score }}</span>
            </div>
            <div
              style="font-size: 11px; font-weight: 500"
              :style="{ color: winnerLabel(g.team_a_score, g.team_b_score).color }"
            >
              {{ winnerLabel(g.team_a_score, g.team_b_score).text }}
            </div>
          </div>

          <div style="font-size: 12px; color: var(--gray); flex-shrink: 0">
            {{ openIds.has(g.id) ? '▲' : '▼' }} 详情
          </div>
        </div>

        <!-- Detail panel -->
        <template v-if="openIds.has(g.id)">
          <div v-if="detailCache[g.id] === null" style="text-align: center; padding: 16px; color: var(--gray)">
            加载中...
          </div>
          <div v-else-if="detailCache[g.id]" class="game-detail-grid">
            <!-- Team A -->
            <div>
              <div style="font-weight: 600; color: var(--team-a); margin-bottom: 6px">
                A队 ({{ detailCache[g.id]!.players.filter(p => p.team === 'A').length }}人)
              </div>
              <div
                v-for="p in detailCache[g.id]!.players.filter(p => p.team === 'A')"
                :key="p.id"
                style="display: flex; justify-content: space-between; padding: 4px 8px; background: #f8f9fa; border-radius: 4px; margin-bottom: 3px; font-size: 13px"
              >
                <span>{{ p.player_name }}</span>
                <span v-if="p.score > 0" style="color: var(--success); font-weight: 500">⚽ {{ p.score }}</span>
                <span v-else style="color: #ccc">—</span>
              </div>
            </div>
            <!-- Team B -->
            <div>
              <div style="font-weight: 600; color: var(--team-b); margin-bottom: 6px">
                B队 ({{ detailCache[g.id]!.players.filter(p => p.team === 'B').length }}人)
              </div>
              <div
                v-for="p in detailCache[g.id]!.players.filter(p => p.team === 'B')"
                :key="p.id"
                style="display: flex; justify-content: space-between; padding: 4px 8px; background: #f8f9fa; border-radius: 4px; margin-bottom: 3px; font-size: 13px"
              >
                <span>{{ p.player_name }}</span>
                <span v-if="p.score > 0" style="color: var(--success); font-weight: 500">⚽ {{ p.score }}</span>
                <span v-else style="color: #ccc">—</span>
              </div>
            </div>
          </div>
        </template>
      </div>
    </div>
  </div>
</template>
