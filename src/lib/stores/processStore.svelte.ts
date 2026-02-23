import type { ProcessDto, ProcessUpdateEvent, SortSpec, SortField, SortDirection } from '$lib/types';

// ────────────────────────────────────────────────────────────────────────────
// Process store (Svelte 5 runes)
// ────────────────────────────────────────────────────────────────────────────

function createProcessStore() {
  let processes = $state<Map<number, ProcessDto>>(new Map());
  let lastUpdated = $state<number>(0);

  function setAll(list: ProcessDto[]) {
    const m = new Map<number, ProcessDto>();
    for (const p of list) m.set(p.pid, p);
    processes = m;
    lastUpdated = Date.now();
  }

  function applyDiff(event: ProcessUpdateEvent) {
    const m = new Map(processes);
    for (const p of event.added) m.set(p.pid, p);
    for (const p of event.updated) m.set(p.pid, p);
    for (const pid of event.removed) m.delete(pid);
    processes = m;
    lastUpdated = event.timestamp_ms;
  }

  return {
    get processes() { return processes; },
    get lastUpdated() { return lastUpdated; },
    get list() { return Array.from(processes.values()); },
    setAll,
    applyDiff,
  };
}

export const processStore = createProcessStore();

// ────────────────────────────────────────────────────────────────────────────
// Selection store
// ────────────────────────────────────────────────────────────────────────────

function createSelectionStore() {
  let selectedPid = $state<number | null>(null);

  return {
    get selectedPid() { return selectedPid; },
    select(pid: number | null) { selectedPid = pid; },
  };
}

export const selectionStore = createSelectionStore();
