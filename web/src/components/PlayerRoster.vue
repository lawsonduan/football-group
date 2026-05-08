<script setup lang="ts">
import { ref, computed } from 'vue'
import { usePlayerStore } from '../stores/players'
import { useTeamStore } from '../stores/teams'
import { POSITION_TAB_OPTIONS } from '../utils/positions'
import PlayerCard from './PlayerCard.vue'

const emit = defineEmits<{
  openAdd: []
  openEdit: [id: number]
}>()

const playerStore = usePlayerStore()
const teamStore = useTeamStore()

const activeTab = ref('all')

const filteredPlayers = computed(() => {
  if (teamStore.isGrouped) {
    return [...teamStore.teamA, ...teamStore.teamB]
  }
  if (activeTab.value === 'all') return playerStore.players
  if (activeTab.value === 'allpos')
    return playerStore.players.filter((p) => p.position === 'all')
  return playerStore.players.filter((p) => p.position === activeTab.value)
})

const selectedCount = computed(() => teamStore.selectedIdList.length)

const selectedLabel = computed(() =>
  teamStore.isGrouped
    ? `(${selectedCount.value}人已选，已分组锁定)`
    : `(${selectedCount.value}人已选)`,
)

async function handleDelete(id: number) {
  if (!confirm('确定要删除该球员吗？')) return
  try {
    await playerStore.remove(id)
    teamStore.selectedIdList.splice(teamStore.selectedIdList.indexOf(id), 1)
  } catch (e) {
    alert('删除失败：' + (e as Error).message)
  }
}

function handleRandomize() {
  if (teamStore.selectedIdList.length < 2) {
    alert('请至少选择2名球员进行分组！')
    return
  }
  teamStore.randomize(playerStore.players)
}
</script>

<template>
  <div class="card">
    <h2>👥 球员名单 <span style="font-weight: 400; font-size: 1rem">{{ selectedLabel }}</span></h2>

    <div class="tabs">
      <button
        v-for="tab in POSITION_TAB_OPTIONS"
        :key="tab.key"
        :class="['tab', { active: activeTab === tab.key }]"
        @click="activeTab = tab.key"
      >
        {{ tab.label }}
      </button>
    </div>

    <div class="btn-group" style="margin-bottom: 16px">
      <button class="btn btn-primary" @click="teamStore.selectAll(playerStore.players)">全选</button>
      <button class="btn btn-warning" @click="teamStore.clearSelection()">清空</button>
      <button class="btn btn-success" @click="handleRandomize">🎲 随机分组</button>
      <button class="btn btn-danger" @click="teamStore.reset()">重置状态</button>
      <button class="btn btn-primary" @click="emit('openAdd')">➕ 添加球员</button>
    </div>

    <div v-if="playerStore.loading" class="empty-state">加载中...</div>

    <div v-else-if="playerStore.error" class="empty-state" style="color: var(--danger)">
      ⚠️ {{ playerStore.error }}
    </div>

    <div v-else class="player-grid">
      <PlayerCard
        v-for="player in filteredPlayers"
        :key="player.id"
        :player="player"
        :selected="teamStore.selectedIds.has(player.id)"
        :locked="teamStore.isGrouped"
        @toggle="teamStore.toggleSelect"
        @edit="emit('openEdit', $event)"
        @delete="handleDelete"
      />
    </div>
  </div>
</template>
