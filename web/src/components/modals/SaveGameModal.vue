<script setup lang="ts">
import { ref } from 'vue'
import { useTeamStore } from '../../stores/teams'
import { useGameStore } from '../../stores/games'
import AppModal from '../AppModal.vue'

const emit = defineEmits<{ close: [] }>()

const teamStore = useTeamStore()
const gameStore = useGameStore()

const gameDate = ref(new Date().toLocaleDateString('en-CA'))
const teamAScore = ref(0)
const teamBScore = ref(0)
const notes = ref('')

// player scores keyed by player id
const scores = ref<Record<number, number>>(
  Object.fromEntries(
    [...teamStore.teamA, ...teamStore.teamB].map((p) => [p.id, 0]),
  ),
)

async function submit() {
  if (!gameDate.value) { alert('请选择比赛日期'); return }
  try {
    await gameStore.create({
      game_date: gameDate.value,
      team_a_score: teamAScore.value,
      team_b_score: teamBScore.value,
      notes: notes.value.trim() || null,
      players: [
        ...teamStore.teamA.map((p) => ({
          player_id: p.id,
          player_name: p.name,
          team: 'A' as const,
          score: scores.value[p.id] ?? 0,
        })),
        ...teamStore.teamB.map((p) => ({
          player_id: p.id,
          player_name: p.name,
          team: 'B' as const,
          score: scores.value[p.id] ?? 0,
        })),
      ],
    })
    emit('close')
  } catch (e) {
    alert('保存失败：' + (e as Error).message)
  }
}
</script>

<template>
  <AppModal :wide="true" @close="emit('close')">
    <h3>💾 记录比赛</h3>
    <form @submit.prevent="submit">
      <div class="form-group">
        <label>比赛日期</label>
        <input type="date" v-model="gameDate" />
      </div>

      <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 12px">
        <div class="form-group">
          <label style="color: var(--team-a)">A队得分</label>
          <input type="number" v-model.number="teamAScore" min="0" />
        </div>
        <div class="form-group">
          <label style="color: var(--team-b)">B队得分</label>
          <input type="number" v-model.number="teamBScore" min="0" />
        </div>
      </div>

      <div class="form-group">
        <label>备注（可选）</label>
        <input v-model="notes" placeholder="如：第5场, 周末联赛..." />
      </div>

      <!-- Per-player goal counts -->
      <div style="border-top: 1px solid #eee; padding-top: 12px; margin-top: 4px">
        <!-- Team A -->
        <div style="margin-bottom: 12px">
          <div style="font-weight: 600; color: var(--team-a); margin-bottom: 6px">A队 — 个人进球数</div>
          <div v-for="p in teamStore.teamA" :key="p.id" class="player-score-row">
            <span>{{ p.name }}</span>
            <input
              type="number"
              v-model.number="scores[p.id]"
              min="0"
              style="width: 64px; padding: 4px 8px; border: 2px solid #e0e0e0; border-radius: 6px; font-size: 14px; text-align: center"
            />
          </div>
        </div>
        <!-- Team B -->
        <div>
          <div style="font-weight: 600; color: var(--team-b); margin-bottom: 6px">B队 — 个人进球数</div>
          <div v-for="p in teamStore.teamB" :key="p.id" class="player-score-row">
            <span>{{ p.name }}</span>
            <input
              type="number"
              v-model.number="scores[p.id]"
              min="0"
              style="width: 64px; padding: 4px 8px; border: 2px solid #e0e0e0; border-radius: 6px; font-size: 14px; text-align: center"
            />
          </div>
        </div>
      </div>

      <div class="modal-actions" style="margin-top: 16px">
        <button type="button" class="btn btn-warning" @click="emit('close')">取消</button>
        <button type="submit" class="btn btn-success">保存比赛</button>
      </div>
    </form>
  </AppModal>
</template>
