<script setup lang="ts">
import { ref, computed } from 'vue'
import type { Player } from '../types'
import { posLabel, posClass } from '../utils/positions'
import { calcOvrForPos, ovrBg } from '../utils/stats'
import PlayerAvatar from './PlayerAvatar.vue'
import PlayerStatsBadge from './PlayerStatsBadge.vue'

const props = defineProps<{
  player: Player
  selected: boolean
  locked?: boolean
}>()

const emit = defineEmits<{
  toggle: [id: number]
}>()

const activeIdx = ref(0)
const activePos = computed(() => props.player.positions[activeIdx.value] ?? null)
const ovr = computed(() => (activePos.value ? calcOvrForPos(activePos.value) : 50))
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
      <div class="ovr-badge" :style="{ background: ovrBg(ovr) }">
        {{ ovr }}
      </div>
    </div>

    <!-- Info column -->
    <div class="info">
      <!-- Top row: name -->
      <div class="info-top">
        <div class="name">{{ player.name }}</div>
      </div>

      <!-- Position pills (clickable to switch stats) -->
      <div class="pos-pills" @click.stop>
        <button
          v-for="(pos, i) in player.positions"
          :key="i"
          type="button"
          :class="['position-tag', posClass(pos.position), { active: activeIdx === i }]"
          @click="activeIdx = i"
        >
          {{ posLabel(pos.position) }}
        </button>
      </div>

      <!-- Stats for currently selected position -->
      <PlayerStatsBadge v-if="activePos" :pos="activePos" />
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

.info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.info-top {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 4px;
}

.name {
  font-weight: 700;
  font-size: 14px;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.pos-pills {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}

/* Position tags are buttons here — override default button styles */
.pos-pills .position-tag {
  border: none;
  cursor: pointer;
  font-family: inherit;
  opacity: 0.6;
  transition: opacity 0.15s, transform 0.1s;
}

.pos-pills .position-tag.active {
  opacity: 1;
  transform: scale(1.05);
}
</style>
