<script setup lang="ts">
import { onMounted } from 'vue'
import { usePlayerStore } from './stores/players'
import { useTeamStore } from './stores/teams'
import PlayerSearch from './components/PlayerSearch.vue'
import PlayerRoster from './components/PlayerRoster.vue'
import TeamResult from './components/TeamResult.vue'
import MatchReports from './components/MatchReports.vue'

const playerStore = usePlayerStore()
const teamStore = useTeamStore()

onMounted(async () => {
  await playerStore.load()
  teamStore.load(new Set(playerStore.players.map((p) => p.id)))
})
</script>

<template>
  <div class="container">
    <header>
      <h1>⚽ 足球比赛分组系统</h1>
      <p>公平竞赛，快乐足球</p>
    </header>

    <PlayerSearch />

    <PlayerRoster />

    <TeamResult />

    <MatchReports />
  </div>
</template>
