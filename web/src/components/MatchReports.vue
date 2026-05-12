<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { reportsApi } from '../api'
import type { Report } from '../types'

const reports = ref<Report[]>([])
const loading = ref(true)
const error = ref<string | null>(null)
const expanded = ref<Set<number>>(new Set())

onMounted(async () => {
  try {
    reports.value = await reportsApi.list()
  } catch (e) {
    error.value = (e as Error).message
  } finally {
    loading.value = false
  }
})

function toggle(idx: number) {
  if (expanded.value.has(idx)) {
    expanded.value.delete(idx)
  } else {
    expanded.value.add(idx)
  }
}

function formatDate(d: string): string {
  // YYYY-MM-DD → YYYY年M月D日
  const parts = d.split('-')
  if (parts.length !== 3) return d
  return `${parts[0]}年${parseInt(parts[1])}月${parseInt(parts[2])}日`
}
</script>

<template>
  <div class="card">
    <h2>📝 小作文 <span class="count" v-if="!loading">({{ reports.length }} 篇)</span></h2>

    <div v-if="loading" class="empty-state">加载中…</div>
    <div v-else-if="error" class="empty-state" style="color: var(--danger)">⚠️ {{ error }}</div>
    <div v-else-if="reports.length === 0" class="empty-state">暂无记录</div>

    <div v-else class="report-list">
      <div
        v-for="(r, i) in reports"
        :key="i"
        class="report-card"
      >
        <!-- Header row -->
        <div class="report-header" @click="toggle(i)">
          <div class="report-meta">
            <span class="report-date">{{ formatDate(r.date) }}</span>
            <span class="report-words">{{ r.word_count }} 字</span>
          </div>
          <button class="expand-btn" type="button" :aria-label="expanded.has(i) ? '收起' : '展开'">
            {{ expanded.has(i) ? '▲' : '▼' }}
          </button>
        </div>

        <!-- Preview / full content -->
        <div :class="['report-body', { expanded: expanded.has(i) }]">
          <p class="report-content">{{ r.content }}</p>
        </div>

        <button
          v-if="!expanded.has(i)"
          class="read-more"
          type="button"
          @click="toggle(i)"
        >展开全文</button>
        <button
          v-else
          class="read-more"
          type="button"
          @click="toggle(i)"
        >收起</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.count {
  font-weight: 400;
  font-size: 1rem;
  color: var(--gray);
}

.report-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.report-card {
  border: 1px solid #e8e8e8;
  border-radius: 10px;
  overflow: hidden;
  background: #fafafa;
  transition: box-shadow 0.15s;
}

.report-card:hover {
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.08);
}

.report-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  cursor: pointer;
  background: white;
  border-bottom: 1px solid #f0f0f0;
  user-select: none;
}

.report-meta {
  display: flex;
  align-items: center;
  gap: 12px;
}

.report-date {
  font-weight: 700;
  font-size: 15px;
  color: #333;
}

.report-words {
  font-size: 12px;
  color: #aaa;
  background: #f0f0f0;
  padding: 2px 7px;
  border-radius: 10px;
}

.expand-btn {
  background: none;
  border: none;
  font-size: 11px;
  color: #aaa;
  cursor: pointer;
  padding: 2px 4px;
}

/* Content area */
.report-body {
  padding: 12px 16px 0;
  max-height: 4.8em; /* ~3 lines */
  overflow: hidden;
  position: relative;
  transition: max-height 0.3s ease;
}

.report-body.expanded {
  max-height: 2000px;
}

/* Fade-out gradient when collapsed */
.report-body:not(.expanded)::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 2em;
  background: linear-gradient(transparent, #fafafa);
  pointer-events: none;
}

.report-content {
  white-space: pre-wrap;
  line-height: 1.7;
  font-size: 14px;
  color: #444;
  margin: 0 0 10px;
}

.read-more {
  display: block;
  width: 100%;
  padding: 8px;
  border: none;
  background: transparent;
  color: var(--primary);
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  text-align: center;
  border-top: 1px solid #f0f0f0;
  transition: background 0.15s;
}

.read-more:hover {
  background: #f5f8ff;
}
</style>
