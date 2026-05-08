<script setup lang="ts">
import { ref, computed } from 'vue'
import { usePlayerStore } from '../stores/players'
import { useTeamStore } from '../stores/teams'
import { posLabel, posClass } from '../utils/positions'
import PlayerAvatar from './PlayerAvatar.vue'

const emit = defineEmits<{ edit: [id: number] }>()

const playerStore = usePlayerStore()
const teamStore = useTeamStore()

const query = ref('')
const showResults = ref(false)
const detailPlayer = ref<number | null>(null)

const results = computed(() =>
  query.value.trim()
    ? playerStore.players.filter((p) => p.name.includes(query.value.trim()))
    : [],
)

function selectResult(id: number) {
  detailPlayer.value = id
  query.value = playerStore.findById(id)?.name ?? ''
  showResults.value = false
}

function onBlur() {
  setTimeout(() => { showResults.value = false }, 150)
}

const detail = computed(() =>
  detailPlayer.value ? playerStore.findById(detailPlayer.value) : null,
)
</script>

<template>
  <div class="card">
    <h2>🔍 查询球员</h2>

    <div class="search-box">
      <input
        v-model="query"
        placeholder="输入球员姓名搜索..."
        @input="showResults = true"
        @focus="showResults = true"
        @blur="onBlur"
      />
      <div :class="['search-results', { show: showResults && results.length > 0 }]">
        <div
          v-for="p in results"
          :key="p.id"
          class="search-result-item"
          @mousedown.prevent="selectResult(p.id)"
        >
          <strong>{{ p.name }}</strong> — {{ posLabel(p.position) }}
          {{ teamStore.selectedIds.has(p.id) ? ' ✅' : '' }}
        </div>
        <div v-if="query.trim() && results.length === 0" class="search-result-item">
          未找到球员
        </div>
      </div>
    </div>

    <div v-if="detail" class="player-detail">
      <PlayerAvatar :player="detail" :size="60" />
      <div class="player-info">
        <h3>{{ detail.name }}</h3>
        <p>
          位置:
          <span :class="['position-tag', posClass(detail.position)]">
            {{ posLabel(detail.position) }}
          </span>
          &nbsp;|&nbsp;
          状态: {{ teamStore.selectedIds.has(detail.id) ? '✅ 已报名' : '❌ 未报名' }}
        </p>
        <div style="margin-top: 10px; display: flex; gap: 8px">
          <button class="btn btn-primary" @click="teamStore.toggleSelect(detail.id)">
            {{ teamStore.selectedIds.has(detail.id) ? '取消报名' : '立即报名' }}
          </button>
          <button class="btn btn-warning" @click="emit('edit', detail.id)">编辑</button>
        </div>
      </div>
    </div>
  </div>
</template>
