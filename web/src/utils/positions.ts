export const POSITION_LABELS: Record<string, string> = {
  fw: '前锋',
  mf: '中场',
  df: '后卫',
  gk: '守门员',
}

export const POSITION_OPTIONS = [
  { value: 'fw', label: '前锋' },
  { value: 'mf', label: '中场' },
  { value: 'df', label: '后卫' },
  { value: 'gk', label: '守门员' },
] as const

// Short labels for compact tabs
export const POSITION_SHORT: Record<string, string> = {
  fw: '前锋',
  mf: '中场',
  df: '后卫',
  gk: 'GK',
}

export const POSITION_TAB_OPTIONS = [
  { key: 'all', label: '全部' },
  { key: 'fw', label: '前锋' },
  { key: 'mf', label: '中场' },
  { key: 'df', label: '后卫' },
  { key: 'gk', label: '守门员' },
  { key: 'multi', label: '多位置' },
]

export function posLabel(pos: string): string {
  return POSITION_LABELS[pos] ?? pos
}

export function posClass(pos: string): string {
  return `position-${pos || 'none'}`
}
