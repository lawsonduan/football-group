<script setup lang="ts">
import { computed } from 'vue'
import { useTeamStore } from '../stores/teams'
import { posLabel, posClass } from '../utils/positions'
import { calcOvr, ovrBg } from '../utils/stats'
import PlayerStatsBadge from './PlayerStatsBadge.vue'
import type { Player } from '../types'

const teamStore = useTeamStore()

function primaryPos(p: Player): string {
  return p.positions[0]?.position ?? ''
}

function stats(team: Player[]) {
  return {
    fw: team.filter((p) => primaryPos(p) === 'fw').length,
    mf: team.filter((p) => primaryPos(p) === 'mf').length,
    df: team.filter((p) => primaryPos(p) === 'df').length,
    gk: team.filter((p) => primaryPos(p) === 'gk').length,
  }
}

const statsA = computed(() => stats(teamStore.teamA))
const statsB = computed(() => stats(teamStore.teamB))

function teamOvr(team: Player[]): number {
  if (!team.length) return 0
  return Math.round(team.reduce((s, p) => s + calcOvr(p), 0) / team.length)
}
</script>

<template>
  <div v-if="teamStore.isGrouped" class="card">
    <h2>🏆 分组结果</h2>

    <div class="btn-group" style="margin-bottom: 16px">
      <button class="btn btn-primary" @click="teamStore.unlock()">🔓 调整分组</button>
      <button class="btn btn-success" @click="teamStore.randomize([...teamStore.teamA, ...teamStore.teamB])">
        🎲 重新随机分组
      </button>
    </div>

    <div class="teams-container">
      <!-- Team A -->
      <div class="team team-a">
        <div class="team-header">
          A队 ({{ teamStore.teamA.length }}人)
          <span class="ovr-tag">OVR {{ teamOvr(teamStore.teamA) }}</span>
        </div>
        <div class="team-content">
          <div class="team-stats">
            <div class="stat-item">前锋: <strong>{{ statsA.fw }}</strong></div>
            <div class="stat-item">中场: <strong>{{ statsA.mf }}</strong></div>
            <div class="stat-item">后卫: <strong>{{ statsA.df }}</strong></div>
            <div class="stat-item">守门员: <strong>{{ statsA.gk }}</strong></div>
          </div>
          <ul class="team-players">
            <li v-for="p in teamStore.teamA" :key="p.id" class="player-row">
              <div class="player-top">
                <div class="player-left">
                  <span class="player-name">{{ p.name }}</span>
                  <span
                    v-if="primaryPos(p)"
                    :class="['position-tag', posClass(primaryPos(p))]"
                  >{{ posLabel(primaryPos(p)) }}</span>
                </div>
                <span class="ovr-badge" :style="{ background: ovrBg(calcOvr(p)) }">
                  {{ calcOvr(p) }}
                </span>
              </div>
              <PlayerStatsBadge v-if="p.positions[0]" :pos="p.positions[0]" />
            </li>
          </ul>
        </div>
      </div>

      <!-- Team B -->
      <div class="team team-b">
        <div class="team-header">
          B队 ({{ teamStore.teamB.length }}人)
          <span class="ovr-tag">OVR {{ teamOvr(teamStore.teamB) }}</span>
        </div>
        <div class="team-content">
          <div class="team-stats">
            <div class="stat-item">前锋: <strong>{{ statsB.fw }}</strong></div>
            <div class="stat-item">中场: <strong>{{ statsB.mf }}</strong></div>
            <div class="stat-item">后卫: <strong>{{ statsB.df }}</strong></div>
            <div class="stat-item">守门员: <strong>{{ statsB.gk }}</strong></div>
          </div>
          <ul class="team-players">
            <li v-for="p in teamStore.teamB" :key="p.id" class="player-row">
              <div class="player-top">
                <div class="player-left">
                  <span class="player-name">{{ p.name }}</span>
                  <span
                    v-if="primaryPos(p)"
                    :class="['position-tag', posClass(primaryPos(p))]"
                  >{{ posLabel(primaryPos(p)) }}</span>
                </div>
                <span class="ovr-badge" :style="{ background: ovrBg(calcOvr(p)) }">
                  {{ calcOvr(p) }}
                </span>
              </div>
              <PlayerStatsBadge v-if="p.positions[0]" :pos="p.positions[0]" />
            </li>
          </ul>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.team-header {
  display: flex;
  align-items: center;
  gap: 8px;
}

.ovr-tag {
  font-size: 12px;
  font-weight: 700;
  background: rgba(255,255,255,0.25);
  padding: 2px 8px;
  border-radius: 10px;
}

/* Per-player row inside the team list */
.player-row {
  display: flex;
  flex-direction: column;
  gap: 6px;
  padding: 10px;
  border-bottom: 1px solid #eee;
}

.player-row:last-child {
  border-bottom: none;
}

.player-top {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 8px;
}

.player-left {
  display: flex;
  align-items: center;
  gap: 6px;
  flex-wrap: wrap;
  min-width: 0;
}

.ovr-badge {
  flex-shrink: 0;
  width: 34px;
  height: 34px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  font-size: 12px;
  font-weight: 800;
  letter-spacing: -0.5px;
}
</style>
