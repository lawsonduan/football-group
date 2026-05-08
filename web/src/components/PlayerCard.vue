<script setup lang="ts">
import type { Player } from '../types'
import { posLabel, posClass } from '../utils/positions'
import PlayerAvatar from './PlayerAvatar.vue'

defineProps<{
  player: Player
  selected: boolean
  locked?: boolean
}>()

const emit = defineEmits<{
  toggle: [id: number]
  edit: [id: number]
  delete: [id: number]
}>()
</script>

<template>
  <div
    :class="['player-card', { selected }]"
    :style="locked ? 'cursor: default' : ''"
    @click="!locked && emit('toggle', player.id)"
  >
    <!-- Avatar doubles as the selection indicator -->
    <div class="avatar-wrapper">
      <PlayerAvatar :player="player" :size="40" />
      <div v-if="selected" class="avatar-check">✓</div>
    </div>

    <div class="info">
      <div class="name">{{ player.name }}</div>
      <div class="position">
        <span :class="['position-tag', posClass(player.position)]">
          {{ posLabel(player.position) }}
        </span>
      </div>
    </div>

    <div v-if="!locked" class="actions" @click.stop>
      <button class="btn-icon btn-edit" title="编辑" @click="emit('edit', player.id)">✏️</button>
      <button class="btn-icon btn-delete" title="删除" @click="emit('delete', player.id)">🗑️</button>
    </div>
  </div>
</template>

<style scoped>
.avatar-wrapper {
  position: relative;
  flex-shrink: 0;
}

.avatar-check {
  position: absolute;
  inset: 0;
  border-radius: 50%;
  background: rgba(52, 168, 83, 0.85);
  color: white;
  font-size: 18px;
  font-weight: 700;
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
