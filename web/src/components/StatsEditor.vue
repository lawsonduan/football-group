<script setup lang="ts">
import type { PlayerStats } from '../types'
import { STAT_KEYS, STAT_LABELS, statColor } from '../utils/stats'

const model = defineModel<PlayerStats>({ required: true })
</script>

<template>
  <div class="stats-editor">
    <div v-for="key in STAT_KEYS" :key="key" class="stat-row">
      <div class="stat-header">
        <span class="stat-key-label">{{ STAT_LABELS[key] }}</span>
        <span class="stat-number" :style="{ color: statColor(model[key]) }">
          {{ model[key] }}
        </span>
      </div>
      <input
        type="range"
        min="1"
        max="99"
        step="1"
        :value="model[key]"
        class="stat-slider"
        :style="`--fill: ${statColor(model[key])}`"
        @input="model[key] = +($event.target as HTMLInputElement).value"
      />
    </div>
  </div>
</template>

<style scoped>
.stats-editor {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 10px 20px;
  padding: 12px 0 4px;
}

.stat-row {
  display: flex;
  flex-direction: column;
  gap: 3px;
}

.stat-header {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
}

.stat-key-label {
  font-size: 12px;
  font-weight: 700;
  color: var(--gray);
  letter-spacing: 0.5px;
}

.stat-number {
  font-size: 15px;
  font-weight: 700;
  min-width: 24px;
  text-align: right;
}

/* Slider styling */
.stat-slider {
  -webkit-appearance: none;
  appearance: none;
  width: 100%;
  height: 4px;
  border-radius: 2px;
  background: #e0e0e0;
  outline: none;
  cursor: pointer;
}

.stat-slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 14px;
  height: 14px;
  border-radius: 50%;
  background: var(--fill, var(--primary));
  cursor: pointer;
  transition: transform 0.15s;
}

.stat-slider::-webkit-slider-thumb:hover {
  transform: scale(1.25);
}

.stat-slider::-moz-range-thumb {
  width: 14px;
  height: 14px;
  border: none;
  border-radius: 50%;
  background: var(--fill, var(--primary));
  cursor: pointer;
}
</style>
