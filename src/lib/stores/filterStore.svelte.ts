import type { ProcessFilter, SortSpec, SortField, SortDirection } from '$lib/types';

// ────────────────────────────────────────────────────────────────────────────
// Filter + sort store (Svelte 5 runes)
// ────────────────────────────────────────────────────────────────────────────

function createFilterStore() {
  let search = $state('');
  let mineOnly = $state(false);
  let systemOnly = $state(false);
  let nonSystemOnly = $state(false);
  let cpuGt = $state<number | undefined>(undefined);
  let memGtMb = $state<number | undefined>(undefined);
  let sortField = $state<SortField>('cpu_percent');
  let sortDirection = $state<SortDirection>('desc');

  function toFilter(): ProcessFilter {
    const f: ProcessFilter = {};
    if (search.trim()) f.search = search.trim();
    if (mineOnly) f.mine_only = true;
    if (systemOnly) f.system_only = true;
    if (nonSystemOnly) f.non_system_only = true;
    // [FIX] Allow threshold = 0 to be a valid filter ("show only processes
    // with CPU/RAM > 0"). Previously 0 was silently ignored.
    if (cpuGt !== undefined && cpuGt >= 0) f.cpu_gt = cpuGt;
    if (memGtMb !== undefined && memGtMb >= 0) f.memory_gt_bytes = memGtMb * 1024 * 1024;
    return f;
  }

  function toSort(): SortSpec {
    return { field: sortField, direction: sortDirection };
  }

  function toggleSort(field: SortField) {
    if (sortField === field) {
      sortDirection = sortDirection === 'asc' ? 'desc' : 'asc';
    } else {
      sortField = field;
      sortDirection = field === 'cpu_percent' || field === 'memory_bytes' ? 'desc' : 'asc';
    }
  }

  function reset() {
    search = '';
    mineOnly = false;
    systemOnly = false;
    nonSystemOnly = false;
    cpuGt = undefined;
    memGtMb = undefined;
  }

  return {
    get search() { return search; },
    set search(v: string) { search = v; },
    get mineOnly() { return mineOnly; },
    set mineOnly(v: boolean) { mineOnly = v; },
    get systemOnly() { return systemOnly; },
    set systemOnly(v: boolean) { systemOnly = v; },
    get nonSystemOnly() { return nonSystemOnly; },
    set nonSystemOnly(v: boolean) { nonSystemOnly = v; },
    get cpuGt() { return cpuGt; },
    set cpuGt(v: number | undefined) { cpuGt = v; },
    get memGtMb() { return memGtMb; },
    set memGtMb(v: number | undefined) { memGtMb = v; },
    get sortField() { return sortField; },
    get sortDirection() { return sortDirection; },
    toFilter,
    toSort,
    toggleSort,
    reset,
  };
}

export const filterStore = createFilterStore();
