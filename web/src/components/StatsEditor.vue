<script setup lang="ts">
import { ref, computed } from 'vue'
import type { PositionInput } from '../types'
import { STAT_KEYS, STAT_LABELS, statColor, calcOvrForPos, ovrBg, DEFAULT_STATS_BY_POS } from '../utils/stats'
import { POSITION_OPTIONS, POSITION_SHORT } from '../utils/positions'

const model = defineModel<PositionInput[]>({ required: true })
const activeIdx = ref(0)

const current = computed(() => model.value[activeIdx.value] ?? null)

function addPosition() {
  const used = new Set(model.value.map((p) => p.position))
  const next = (['fw', 'mf', 'df', 'gk'] as const).find((p) => !used.has(p))
  if (!next) return
  model.value.push({ position: next, ...(DEFAULT_STATS_BY_POS[next] ?? { pac: 50, sho: 50, pas: 50, dri: 50, def: 50, phy: 50 }) })
  activeIdx.value = model.value.length - 1
}

function removePosition(idx: number) {
  if (model.value.length <= 1) return
  model.value.splice(idx, 1)
  if (activeIdx.value >= model.value.length) {
    activeIdx.value = model.value.length - 1
  }
}

function setStat(key: string, value: number) {
  if (current.value) {
    ;(current.value as Record<string, number>)[key] = value
  }
}
</script>

<template>
  <div class="stats-editor">
    <!-- Position tabs -->
    <div class="pos-tabs">
      <button
        v-for="(pos, i) in model"
        :key="i"
        type="button"
        :class="['pos-tab', { active: activeIdx === i }]"
        @click="activeIdx = i"
      >
        {{ POSITION_SHORT[pos.position] ?? pos.position }}
        <span
          v-if="model.length > 1"
          class="pos-remove"
          title="移除该位置"
          @click.stop="removePosition(i)"
        >×</span>
      </button>
      <button
        v-if="model.length < 4"
        type="button"
        class="pos-add-btn"
        @click="addPosition"
      >
        ＋ 添加位置
      </button>
    </div>

    <template v-if="current">
      <!-- Position selector + OVR -->
      <div class="pos-header">
        <select v-model="current.position" class="pos-select">
          <option v-for="opt in POSITION_OPTIONS" :key="opt.value" :value="opt.value">
            {{ opt.label }}
          </option>
        </select>
        <div class="ovr-chip" :style="{ background: ovrBg(calcOvrForPos(current)) }">
          OVR {{ calcOvrForPos(current) }}
        </div>
      </div>

      <!-- Stat sliders -->
      <div class="stats-grid">
        <div v-for="key in STAT_KEYS" :key="key" class="stat-row">
          <div class="stat-header">
            <span class="stat-key-label">{{ STAT_LABELS[key] }}</span>
            <span class="stat-number" :style="{ color: statColor(current[key]) }">
              {{ current[key] }}
            </span>
          </div>
          <input
            type="range"
            min="1"
            max="99"
            step="1"
            :value="current[key]"
            class="stat-slider"
            :style="`--fill: ${statColor(current[key])}`"
            @input="setStat(key, +($event.target as HTMLInputElement).value)"
          />
        </div>
      </div>
    </template>
  </div>
</template>

<style scoped>
.stats-editor {
  padding: 10px 0 4px;
}

/* Position tab row */
.pos-tabs {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  margin-bottom: 10px;
}

.pos-tab {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 10px;
  border: 1.5px solid #d0d0d0;
  border-radius: 20px;
  background: #f5f5f5;
  font-size: 12px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.15s;
  color: #555;
}

.pos-tab.active {
  background: var(--primary);
  border-color: var(--primary);
  color: white;
}

.pos-remove {
  font-size: 14px;
  line-height: 1;
  color: inherit;
  opacity: 0.7;
  padding: 0 0 0 2px;
  cursor: pointer;
}

.pos-remove:hover {
  opacity: 1;
}

.pos-add-btn {
  padding: 4px 10px;
  border: 1.5px dashed #b0b0b0;
  border-radius: 20px;
  background: transparent;
  font-size: 12px;
  color: #888;
  cursor: pointer;
  transition: all 0.15s;
}

.pos-add-btn:hover {
  border-color: var(--primary);
  color: var(--primary);
}

/* Position select + OVR row */
.pos-header {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 10px;
}

.pos-select {
  flex: 1;
  padding: 5px 8px;
  border: 1px solid #d0d0d0;
  border-radius: 6px;
  font-size: 13px;
  background: white;
}

.ovr-chip {
  color: white;
  font-size: 13px;
  font-weight: 800;
  padding: 4px 10px;
  border-radius: 8px;
  letter-spacing: 0.5px;
  white-space: nowrap;
}

/* Stat sliders grid */
.stats-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 10px 20px;
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
