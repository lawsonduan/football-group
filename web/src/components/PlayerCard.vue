<script setup lang="ts">
import type { Player } from '../types'
import { posLabel, posClass } from '../utils/positions'
import { calcOvr, ovrBg } from '../utils/stats'
import PlayerAvatar from './PlayerAvatar.vue'
import PlayerStatsBadge from './PlayerStatsBadge.vue'

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
    <!-- Avatar + OVR badge -->
    <div class="avatar-col">
      <div class="avatar-wrapper">
        <PlayerAvatar :player="player" :size="44" />
        <div v-if="selected" class="avatar-check">✓</div>
      </div>
      <div class="ovr-badge" :style="{ background: ovrBg(calcOvr(player)) }">
        {{ calcOvr(player) }}
      </div>
    </div>

    <!-- Info: top row (name + position + actions), then stats -->
    <div class="info">
      <div class="info-top">
        <div class="info-main">
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

      <PlayerStatsBadge :player="player" />
    </div>
  </div>
</template>

<style scoped>
.avatar-col {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 3px;
  flex-shrink: 0;
}

.avatar-wrapper {
  position: relative;
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

.ovr-badge {
  color: white;
  font-size: 12px;
  font-weight: 800;
  padding: 1px 6px;
  border-radius: 4px;
  letter-spacing: 0.3px;
  min-width: 28px;
  text-align: center;
}

/* info column now owns the actions */
.info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
}

.info-top {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 4px;
}

.info-main {
  flex: 1;
  min-width: 0;
}

.actions {
  display: flex;
  gap: 4px;
  flex-shrink: 0;
}
</style>
