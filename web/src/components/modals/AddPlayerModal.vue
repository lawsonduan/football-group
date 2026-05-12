<script setup lang="ts">
import { ref, nextTick, onMounted } from 'vue'
import { usePlayerStore } from '../../stores/players'
import type { PositionInput } from '../../types'
import { DEFAULT_STATS_BY_POS } from '../../utils/stats'
import AppModal from '../AppModal.vue'
import AvatarUploadInput from '../AvatarUploadInput.vue'
import StatsEditor from '../StatsEditor.vue'

const emit = defineEmits<{ close: [] }>()

const playerStore = usePlayerStore()

const name = ref('')
const avatarDataUrl = ref<string | null>(null)
const positions = ref<PositionInput[]>([{ position: 'fw', ...DEFAULT_STATS_BY_POS.fw }])
const inputRef = ref<HTMLInputElement | null>(null)

onMounted(() => nextTick(() => inputRef.value?.focus()))

async function submit() {
  const n = name.value.trim()
  if (!n) { alert('请输入球员姓名'); return }
  if (positions.value.length === 0) { alert('请至少添加一个位置'); return }
  try {
    await playerStore.add(n, positions.value, avatarDataUrl.value)
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
      <AvatarUploadInput @change="avatarDataUrl = $event" />

      <div class="form-group">
        <label>球员姓名</label>
        <input ref="inputRef" v-model="name" placeholder="请输入球员姓名" />
      </div>

      <div class="section-label">⚡ 位置与属性</div>
      <StatsEditor v-model="positions" />

      <div class="modal-actions">
        <button type="button" class="btn btn-warning" @click="emit('close')">取消</button>
        <button type="submit" class="btn btn-success">添加</button>
      </div>
    </form>
  </AppModal>
</template>

<style scoped>
.section-label {
  font-size: 13px;
  font-weight: 600;
  color: var(--gray);
  margin: 10px 0 0;
}
</style>
