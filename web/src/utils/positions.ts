export const POSITION_LABELS: Record<string, string> = {
  fw: '前锋',
  mf: '中场',
  df: '后卫',
  gk: '守门员',
  all: '多面手',
  none: '无',
}

export const POSITION_TAB_OPTIONS = [
  { key: 'all', label: '全部' },
  { key: 'fw', label: '前锋' },
  { key: 'mf', label: '中场' },
  { key: 'df', label: '后卫' },
  { key: 'gk', label: '守门员' },
  { key: 'allpos', label: '多面手' },
]

export function posLabel(pos: string): string {
  return POSITION_LABELS[pos] ?? '无'
}

export function posClass(pos: string): string {
  return `position-${pos || 'none'}`
}
