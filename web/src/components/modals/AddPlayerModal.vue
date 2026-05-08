<script setup lang="ts">
import { ref, nextTick, onMounted } from 'vue'
import { usePlayerStore } from '../../stores/players'
import { playersApi } from '../../api'
import type { PlayerStats } from '../../types'
import { DEFAULT_STATS } from '../../utils/stats'
import AppModal from '../AppModal.vue'
import AvatarUploadInput from '../AvatarUploadInput.vue'
import StatsEditor from '../StatsEditor.vue'

const emit = defineEmits<{ close: [] }>()

const playerStore = usePlayerStore()

const name = ref('')
const position = ref('')
const avatarFile = ref<File | null>(null)
const stats = ref<PlayerStats>({ ...DEFAULT_STATS })
const inputRef = ref<HTMLInputElement | null>(null)
const showStats = ref(false)

onMounted(() => nextTick(() => inputRef.value?.focus()))

async function submit() {
  const n = name.value.trim()
  if (!n) { alert('请输入球员姓名'); return }
  try {
    let player = await playerStore.add(n, position.value || 'none', stats.value)
    if (avatarFile.value) {
      player = await playersApi.uploadAvatar(player.id, avatarFile.value)
      const idx = playerStore.players.findIndex((p) => p.id === player.id)
      if (idx !== -1) playerStore.players[idx] = player
    }
    emit('close')
  } catch (e) {
    alert('添加失败：' + (e as Error).message)
  }
}
</script>

<template>
  <AppModal @close="emit('close')">
    <h3>➕ 添加球员</h3>
    <form @submit.prevent="submit">
      <AvatarUploadInput @change="avatarFile = $event" />

      <div class="form-group">
        <label>球员姓名</label>
        <input ref="inputRef" v-model="name" placeholder="请输入球员姓名" />
      </div>
      <div class="form-group">
        <label>位置</label>
        <select v-model="position">
          <option value="">— 选择位置 —</option>
          <option value="fw">前锋</option>
          <option value="mf">中场</option>
          <option value="df">后卫</option>
          <option value="gk">守门员</option>
          <option value="all">多面手</option>
          <option value="none">无</option>
        </select>
      </div>

      <!-- Collapsible stats section -->
      <div class="stats-section">
        <button
          type="button"
          class="stats-toggle"
          @click="showStats = !showStats"
        >
          ⚡ 球员属性 <span class="toggle-arrow">{{ showStats ? '▲' : '▼' }}</span>
        </button>
        <StatsEditor v-if="showStats" v-model="stats" />
      </div>

      <div class="modal-actions">
        <button type="button" class="btn btn-warning" @click="emit('close')">取消</button>
        <button type="submit" class="btn btn-success">添加</button>
      </div>
    </form>
  </AppModal>
</template>

<style scoped>
.stats-section {
  margin: 8px 0;
}

.stats-toggle {
  background: none;
  border: 1px solid #e0e0e0;
  border-radius: 6px;
  padding: 6px 12px;
  cursor: pointer;
  font-size: 13px;
  color: var(--gray);
  width: 100%;
  text-align: left;
  transition: background 0.15s;
}

.stats-toggle:hover {
  background: #f5f5f5;
}

.toggle-arrow {
  float: right;
  font-size: 10px;
}
</style>
