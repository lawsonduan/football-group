<script setup lang="ts">
import { computed } from 'vue'
import type { Player } from '../types'
import { STAT_KEYS, STAT_LABELS, statColor } from '../utils/stats'

const props = defineProps<{ player: Player }>()

// Render as two rows of three: [PAC SHO PAS] / [DRI DEF PHY]
const rows = computed(() => [
  STAT_KEYS.slice(0, 3),
  STAT_KEYS.slice(3, 6),
])
</script>

<template>
  <div class="stats-badge">
    <div v-for="(row, ri) in rows" :key="ri" class="stats-row">
      <div v-for="key in row" :key="key" class="stat-cell">
        <span class="stat-lbl">{{ STAT_LABELS[key] }}</span>
        <span class="stat-val" :style="{ color: statColor(player[key]) }">
          {{ player[key] }}
        </span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.stats-badge {
  display: flex;
  flex-direction: column;
  gap: 2px;
  margin-top: 5px;
}

.stats-row {
  display: flex;
  gap: 8px;
}

.stat-cell {
  display: flex;
  flex-direction: column;
  align-items: center;
  min-width: 26px;
}

.stat-lbl {
  font-size: 8.5px;
  color: #9e9e9e;
  font-weight: 700;
  letter-spacing: 0.4px;
  line-height: 1;
}

.stat-val {
  font-size: 12px;
  font-weight: 700;
  line-height: 1.3;
}
</style>
