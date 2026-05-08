<script setup lang="ts">
import { computed } from 'vue'
import { useTeamStore } from '../stores/teams'
import { posLabel, posClass } from '../utils/positions'
import type { Player } from '../types'

const emit = defineEmits<{ openSaveGame: [] }>()

const teamStore = useTeamStore()

function stats(team: Player[]) {
  return {
    fw:   team.filter((p) => p.position === 'fw').length,
    mf:   team.filter((p) => p.position === 'mf').length,
    df:   team.filter((p) => p.position === 'df').length,
    gk:   team.filter((p) => p.position === 'gk').length,
    all:  team.filter((p) => p.position === 'all').length,
    none: team.filter((p) => p.position === 'none' || !p.position).length,
  }
}

const statsA = computed(() => stats(teamStore.teamA))
const statsB = computed(() => stats(teamStore.teamB))
</script>

<template>
  <div v-if="teamStore.isGrouped" class="card">
    <h2>🏆 分组结果</h2>

    <div class="btn-group" style="margin-bottom: 16px">
      <button class="btn btn-primary" @click="teamStore.unlock()">🔓 调整分组</button>
      <button class="btn btn-success" @click="teamStore.randomize([...teamStore.teamA, ...teamStore.teamB])">
        🎲 重新随机分组
      </button>
      <button class="btn btn-warning" @click="emit('openSaveGame')">💾 记录比赛</button>
    </div>

    <div class="teams-container">
      <!-- Team A -->
      <div class="team team-a">
        <div class="team-header">A队 ({{ teamStore.teamA.length }}人)</div>
        <div class="team-content">
          <div class="team-stats">
            <div class="stat-item">前锋: <strong>{{ statsA.fw }}</strong></div>
            <div class="stat-item">中场: <strong>{{ statsA.mf }}</strong></div>
            <div class="stat-item">后卫: <strong>{{ statsA.df }}</strong></div>
            <div class="stat-item">守门员: <strong>{{ statsA.gk }}</strong></div>
            <div class="stat-item">多面手: <strong>{{ statsA.all }}</strong></div>
            <div class="stat-item">无位置: <strong>{{ statsA.none }}</strong></div>
          </div>
          <ul class="team-players">
            <li v-for="p in teamStore.teamA" :key="p.id">
              <span class="player-name">{{ p.name }}</span>
              <span :class="['position-tag', posClass(p.position)]">{{ posLabel(p.position) }}</span>
            </li>
          </ul>
        </div>
      </div>

      <!-- Team B -->
      <div class="team team-b">
        <div class="team-header">B队 ({{ teamStore.teamB.length }}人)</div>
        <div class="team-content">
          <div class="team-stats">
            <div class="stat-item">前锋: <strong>{{ statsB.fw }}</strong></div>
            <div class="stat-item">中场: <strong>{{ statsB.mf }}</strong></div>
            <div class="stat-item">后卫: <strong>{{ statsB.df }}</strong></div>
            <div class="stat-item">守门员: <strong>{{ statsB.gk }}</strong></div>
            <div class="stat-item">多面手: <strong>{{ statsB.all }}</strong></div>
            <div class="stat-item">无位置: <strong>{{ statsB.none }}</strong></div>
          </div>
          <ul class="team-players">
            <li v-for="p in teamStore.teamB" :key="p.id">
              <span class="player-name">{{ p.name }}</span>
              <span :class="['position-tag', posClass(p.position)]">{{ posLabel(p.position) }}</span>
            </li>
          </ul>
        </div>
      </div>
    </div>
  </div>
</template>
