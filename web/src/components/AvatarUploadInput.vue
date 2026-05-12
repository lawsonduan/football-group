<script setup lang="ts">
import { ref, computed } from 'vue'
import type { Player } from '../types'

const props = defineProps<{ player?: Player | null }>()
// Emits a base64 data URL (or null to clear)
const emit = defineEmits<{ change: [dataUrl: string | null] }>()

const inputRef = ref<HTMLInputElement | null>(null)
const previewUrl = ref<string | null>(props.player?.avatar ?? null)

function onFileChange(e: Event) {
  const file = (e.target as HTMLInputElement).files?.[0] ?? null
  if (!file) return

  const reader = new FileReader()
  reader.onload = () => {
    const dataUrl = reader.result as string
    previewUrl.value = dataUrl
    emit('change', dataUrl)
  }
  reader.readAsDataURL(file)
}

function clear() {
  previewUrl.value = props.player?.avatar ?? null
  if (inputRef.value) inputRef.value.value = ''
  emit('change', null)
}

const initial = computed(() => props.player?.name?.charAt(0) ?? '?')
</script>

<template>
  <div class="avatar-upload">
    <!-- Preview circle -->
    <div class="avatar-preview" @click="inputRef?.click()">
      <img
        v-if="previewUrl"
        :src="previewUrl"
        alt="avatar"
        class="avatar-preview-img"
      />
      <div v-else class="avatar-preview-default">{{ initial }}</div>
      <div class="avatar-preview-overlay">📷</div>
    </div>

    <div class="avatar-upload-text">
      <button type="button" class="btn btn-primary" style="font-size:13px;padding:6px 14px" @click="inputRef?.click()">
        选择图片
      </button>
      <span style="font-size:12px;color:var(--gray);margin-left:8px">
        JPG / PNG / WebP，最大 5 MB
      </span>
    </div>

    <input
      ref="inputRef"
      type="file"
      accept="image/jpeg,image/png,image/webp,image/gif"
      style="display:none"
      @change="onFileChange"
    />
  </div>
</template>

<style scoped>
.avatar-upload {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 16px;
}

.avatar-preview {
  position: relative;
  width: 72px;
  height: 72px;
  border-radius: 50%;
  cursor: pointer;
  flex-shrink: 0;
  overflow: hidden;
}

.avatar-preview-img,
.avatar-preview-default {
  width: 100%;
  height: 100%;
  border-radius: 50%;
  object-fit: cover;
}

.avatar-preview-default {
  background: linear-gradient(135deg, var(--team-a), var(--team-b));
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-size: 28px;
  font-weight: 700;
  user-select: none;
}

.avatar-preview-overlay {
  position: absolute;
  inset: 0;
  border-radius: 50%;
  background: rgba(0, 0, 0, 0.35);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 22px;
  opacity: 0;
  transition: opacity 0.2s;
}

.avatar-preview:hover .avatar-preview-overlay {
  opacity: 1;
}

.avatar-upload-text {
  display: flex;
  flex-direction: column;
  gap: 6px;
}
</style>
