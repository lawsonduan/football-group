<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { usePlayerStore } from './stores/players'
import { useTeamStore } from './stores/teams'
import { useGameStore } from './stores/games'
import PlayerSearch from './components/PlayerSearch.vue'
import PlayerRoster from './components/PlayerRoster.vue'
import TeamResult from './components/TeamResult.vue'
import GameHistory from './components/GameHistory.vue'
import AddPlayerModal from './components/modals/AddPlayerModal.vue'
import EditPlayerModal from './components/modals/EditPlayerModal.vue'
import SaveGameModal from './components/modals/SaveGameModal.vue'

const playerStore = usePlayerStore()
const teamStore = useTeamStore()
const gameStore = useGameStore()

const addPlayerOpen = ref(false)
const editPlayerId = ref<number | null>(null)
const saveGameOpen = ref(false)

onMounted(async () => {
  await playerStore.load()
  teamStore.load(new Set(playerStore.players.map((p) => p.id)))
  await gameStore.load()
})
</script>

<template>
  <div class="container">
    <header>
      <h1>⚽ 足球比赛分组系统</h1>
      <p>公平竞赛，快乐足球</p>
    </header>

    <PlayerSearch @edit="editPlayerId = $event" />

    <PlayerRoster
      @open-add="addPlayerOpen = true"
      @open-edit="editPlayerId = $event"
    />

    <TeamResult @open-save-game="saveGameOpen = true" />

    <GameHistory />
  </div>

  <!-- Modals are teleported to <body> so they sit above all content -->
  <Teleport to="body">
    <AddPlayerModal v-if="addPlayerOpen" @close="addPlayerOpen = false" />
    <EditPlayerModal
      v-if="editPlayerId !== null"
      :player-id="editPlayerId"
      @close="editPlayerId = null"
    />
    <SaveGameModal v-if="saveGameOpen" @close="saveGameOpen = false" />
  </Teleport>
</template>
