<script setup lang="ts">
import { ref } from 'vue'
import { usePlayerStore } from '../../stores/players'
import type { PositionInput } from '../../types'
import { DEFAULT_STATS_BY_POS } from '../../utils/stats'
import AppModal from '../AppModal.vue'
import AvatarUploadInput from '../AvatarUploadInput.vue'
import StatsEditor from '../StatsEditor.vue'

const props = defineProps<{ playerId: number }>()
const emit = defineEmits<{ close: [] }>()

const playerStore = usePlayerStore()
const existing = playerStore.findById(props.playerId)

const name = ref(existing?.name ?? '')
// undefined = keep existing avatar; string/null = replace
const avatarDataUrl = ref<string | null | undefined>(undefined)
const positions = ref<PositionInput[]>(
  existing?.positions.length
    ? existing.positions.map((p) => ({
        position: p.position,
        pac: p.pac,
        sho: p.sho,
        pas: p.pas,
        dri: p.dri,
        def: p.def,
        phy: p.phy,
      }))
    : [{ position: 'fw', ...DEFAULT_STATS_BY_POS.fw }],
)

async function submit() {
  const n = name.value.trim()
  if (!n) { alert('请输入球员姓名'); return }
  if (positions.value.length === 0) { alert('请至少保留一个位置'); return }
  try {
    await playerStore.update(props.playerId, n, positions.value, avatarDataUrl.value)
    emit('close')
  } catch (e) {
    alert('编辑失败：' + (e as Error).message)
  }
}
</script>

<template>
  <AppModal @close="emit('close')">
    <h3>✏️ 编辑球员</h3>
    <form @submit.prevent="submit">
      <AvatarUploadInput :player="existing" @change="avatarDataUrl = $event" />

      <div class="form-group">
        <label>球员姓名</label>
        <input v-model="name" placeholder="请输入姓名" />
      </div>

      <div class="section-label">⚡ 位置与属性</div>
      <StatsEditor v-model="positions" />

      <div class="modal-actions">
        <button type="button" class="btn btn-warning" @click="emit('close')">取消</button>
        <button type="submit" class="btn btn-success">保存</button>
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
