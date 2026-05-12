<script setup lang="ts">
defineProps<{ wide?: boolean }>()
defineEmits<{ close: [] }>()
</script>

<template>
  <div class="modal-overlay" @click.self="$emit('close')">
    <div :class="['modal', { wide }]">
      <slot />
    </div>
  </div>
</template>
