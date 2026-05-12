const PREFIX = 'fg_'

function lsKey(name: string) {
  return PREFIX + name
}

export function load<T>(name: string, fallback: T): T {
  try {
    const raw = localStorage.getItem(lsKey(name))
    return raw !== null ? (JSON.parse(raw) as T) : fallback
  } catch {
    return fallback
  }
}

export function save<T>(name: string, value: T): void {
  localStorage.setItem(lsKey(name), JSON.stringify(value))
}

export function remove(name: string): void {
  localStorage.removeItem(lsKey(name))
}

/** Returns true only on the very first call for this key (i.e. key was absent). */
export function isFirstRun(name: string): boolean {
  if (localStorage.getItem(lsKey(name)) !== null) return false
  save(name, true)
  return true
}

/** Monotonically-increasing integer ID for a named sequence. */
export function nextId(sequence: string): number {
  const current = load<number>(sequence + '_seq', 0)
  const next = current + 1
  save(sequence + '_seq', next)
  return next
}
