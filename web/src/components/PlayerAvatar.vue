<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import type { Player } from '../types'

const props = defineProps<{
  player: Player
  size?: number
}>()

const sz = computed(() => props.size ?? 40)
const fontSize = computed(() => Math.round(sz.value * 0.42))
const broken = ref(false)

// Reset broken state when player avatar changes
watch(() => props.player.avatar, () => { broken.value = false })

// avatar is now a base64 data URL stored directly in the player object
const src = computed(() =>
  props.player.avatar && !broken.value ? props.player.avatar : null,
)
</script>

<template>
  <img
    v-if="src"
    :src="src"
    :alt="player.name"
    class="player-avatar-img"
    :style="`width:${sz}px;height:${sz}px;font-size:${fontSize}px`"
    @error="broken = true"
  />
  <div
    v-else
    class="player-avatar-default"
    :style="`width:${sz}px;height:${sz}px;font-size:${fontSize}px`"
    :title="player.name"
  >
    {{ player.name.charAt(0) }}
  </div>
</template>

<style scoped>
.player-avatar-img,
.player-avatar-default {
  border-radius: 50%;
  flex-shrink: 0;
  object-fit: cover;
}

.player-avatar-default {
  background: linear-gradient(135deg, var(--team-a), var(--team-b));
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-weight: 700;
  user-select: none;
}
</style>
