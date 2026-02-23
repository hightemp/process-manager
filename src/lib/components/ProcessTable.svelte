<script lang="ts">
  import type { ProcessDto, SortField } from '$lib/types';
  import { filterStore } from '$lib/stores/filterStore.svelte';
  import { selectionStore } from '$lib/stores/processStore.svelte';
  import { settingsStore } from '$lib/stores/settingsStore.svelte';
  import { formatBytes, formatCpu, truncatePath } from '$lib/utils/format';

  interface Props {
    processes: ProcessDto[];
    onKillRequest: (process: ProcessDto, mode: 'terminate' | 'kill') => void;
  }

  let { processes, onKillRequest }: Props = $props();

  // ────────────── Virtualisation ──────────────
  const ROW_HEIGHT = 30; // px per row
  const OVERSCAN = 8;    // extra rows above/below viewport

  let containerEl = $state<HTMLDivElement | undefined>();
  let scrollTop = $state(0);
  let clientHeight = $state(600);

  let startIndex = $derived(Math.max(0, Math.floor(scrollTop / ROW_HEIGHT) - OVERSCAN));
  let endIndex = $derived(
    Math.min(
      processes.length,
      Math.ceil((scrollTop + clientHeight) / ROW_HEIGHT) + OVERSCAN
    )
  );
  let visibleSlice = $derived(processes.slice(startIndex, endIndex));
  let topPad = $derived(startIndex * ROW_HEIGHT);
  let totalHeight = $derived(processes.length * ROW_HEIGHT);

  function handleScroll(e: Event) {
    scrollTop = (e.target as HTMLDivElement).scrollTop;
  }

  function handleContainerResize(entries: ResizeObserverEntry[]) {
    clientHeight = entries[0].contentRect.height;
  }

  let resizeObserver: ResizeObserver | undefined;
  $effect(() => {
    if (containerEl) {
      resizeObserver = new ResizeObserver(handleContainerResize);
      resizeObserver.observe(containerEl);
      clientHeight = containerEl.clientHeight;
    }
    return () => resizeObserver?.disconnect();
  });

  // ────────────── Column handling ──────────────
  let visibleCols = $derived(settingsStore.visibleColumns());

  function handleSort(field: SortField) {
    filterStore.toggleSort(field);
  }

  function sortIcon(field: SortField) {
    if (filterStore.sortField !== field) return '';
    return filterStore.sortDirection === 'asc' ? ' ▲' : ' ▼';
  }

  // ────────────── Row actions ──────────────
  function handleRowClick(p: ProcessDto) {
    selectionStore.select(selectionStore.selectedPid === p.pid ? null : p.pid);
  }

  function handleRowKeydown(e: KeyboardEvent, p: ProcessDto) {
    if (e.key === 'Delete') {
      e.preventDefault();
      onKillRequest(p, 'terminate');
    }
  }

  // ────────────── Cell value renderer ──────────────
  function cellValue(p: ProcessDto, colId: string): string {
    switch (colId) {
      case 'pid': return String(p.pid);
      case 'name': return p.name;
      case 'user': return p.user ?? '–';
      case 'cpu': return formatCpu(p.cpu_percent);
      case 'memory': return formatBytes(p.memory_bytes);
      case 'status': return p.status;
      case 'path': return truncatePath(p.path, 50);
      case 'parent': return p.parent_pid != null ? String(p.parent_pid) : '–';
      default: return '';
    }
  }

  function cpuColorClass(cpu: number): string {
    if (cpu >= 50) return 'cpu-high';
    if (cpu >= 20) return 'cpu-med';
    return '';
  }
</script>

<div
  class="table-wrap"
  onscroll={handleScroll}
  bind:this={containerEl}
  role="grid"
  aria-label="Process list"
>
  <!-- Sticky header -->
  <div class="thead" role="row">
    {#each visibleCols as col}
      <div
        class="th {col.id}"
        style="width: {col.width}"
        role="columnheader"
        aria-sort={filterStore.sortField === col.id ? (filterStore.sortDirection === 'asc' ? 'ascending' : 'descending') : 'none'}
        onclick={() => handleSort(col.id as SortField)}
        onkeydown={(e) => e.key === 'Enter' && handleSort(col.id as SortField)}
        tabindex="0"
      >
        {col.label}{sortIcon(col.id as SortField)}
      </div>
    {/each}
    <!-- Actions column -->
    <div class="th actions-col" role="columnheader">Actions</div>
  </div>

  <!-- Virtualised body -->
  <div class="tbody" style="height: {totalHeight}px; position: relative">
    <div style="transform: translateY({topPad}px)">
      {#each visibleSlice as proc (proc.pid)}
        <!-- svelte-ignore a11y_interactive_supports_focus -->
        <div
          class="tr {selectionStore.selectedPid === proc.pid ? 'selected' : ''} {proc.needs_elevation ? 'needs-elevation' : ''}"
          role="row"
          aria-selected={selectionStore.selectedPid === proc.pid}
          onclick={() => handleRowClick(proc)}
          onkeydown={(e) => handleRowKeydown(e, proc)}
          tabindex="0"
        >
          {#each visibleCols as col}
            <div
              class="td {col.id} {col.id === 'cpu' ? cpuColorClass(proc.cpu_percent) : ''} {col.id === 'status' ? 'status-cell status-' + proc.status : ''}"
              style="width: {col.width}"
              role="gridcell"
            >
              {#if col.id === 'name'}
                <span class="proc-name-cell">
                  {proc.name}
                  {#if proc.needs_elevation}
                    <span class="lock-icon" title="Needs elevated rights">🔒</span>
                  {/if}
                </span>
              {:else}
                {cellValue(proc, col.id)}
              {/if}
            </div>
          {/each}
          <!-- Inline Actions -->
          <div class="td actions-col" role="gridcell">
            <button
              class="action-btn btn-term"
              title="Terminate (SIGTERM)"
              onclick={(e) => { e.stopPropagation(); onKillRequest(proc, 'terminate'); }}
            >✕</button>
            <button
              class="action-btn btn-kill"
              title="Force Kill (SIGKILL)"
              onclick={(e) => { e.stopPropagation(); onKillRequest(proc, 'kill'); }}
            >☠</button>
          </div>
        </div>
      {/each}
    </div>
  </div>

  {#if processes.length === 0}
    <div class="empty-state">No processes match the current filters.</div>
  {/if}
</div>

<style>
  .table-wrap {
    flex: 1;
    overflow-y: auto;
    overflow-x: auto;
    background: var(--surface-1);
    font-size: 0.82rem;
    position: relative;
  }

  .thead {
    display: flex;
    align-items: center;
    position: sticky;
    top: 0;
    z-index: 10;
    background: var(--surface-2);
    border-bottom: 1px solid var(--border);
    height: 32px;
  }

  .th {
    padding: 0 8px;
    height: 100%;
    display: flex;
    align-items: center;
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--text-muted);
    cursor: pointer;
    user-select: none;
    white-space: nowrap;
    flex-shrink: 0;
    transition: color 0.15s;
  }

  .th:hover {
    color: var(--text-primary);
  }

  .tbody {
    width: 100%;
  }

  .tr {
    display: flex;
    align-items: center;
    height: 30px;
    border-bottom: 1px solid var(--border-subtle);
    cursor: pointer;
    transition: background 0.08s;
  }

  .tr:hover {
    background: var(--surface-hover);
  }

  .tr.selected {
    background: var(--color-accent-subtle);
  }

  .tr.needs-elevation {
    border-left: 2px solid var(--color-warning);
  }

  .td {
    padding: 0 8px;
    height: 100%;
    display: flex;
    align-items: center;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    flex-shrink: 0;
    color: var(--text-secondary);
  }

  /* Column-specific widths */
  .pid    { width: 70px; color: var(--text-muted); font-family: monospace; font-size: 0.78rem; }
  .name   { width: 200px; }
  .user   { width: 100px; }
  .cpu    { width: 80px; font-variant-numeric: tabular-nums; }
  .memory { width: 90px; font-variant-numeric: tabular-nums; }
  .status { width: 90px; }
  .path   { flex: 1; min-width: 80px; font-family: monospace; font-size: 0.75rem; }
  .parent { width: 70px; font-family: monospace; font-size: 0.78rem; }
  .actions-col { width: 60px; gap: 4px; }

  .cpu-high { color: #ff5555; font-weight: 600; }
  .cpu-med  { color: #ffaa00; }

  .status-cell.status-running  { color: #3cc864; }
  .status-cell.status-sleeping { color: #6496ff; }
  .status-cell.status-stopped  { color: #ffa030; }
  .status-cell.status-zombie   { color: #ff5050; }

  .proc-name-cell {
    display: flex;
    align-items: center;
    gap: 4px;
    overflow: hidden;
  }

  .lock-icon {
    font-size: 0.7rem;
    opacity: 0.7;
    flex-shrink: 0;
  }

  .action-btn {
    background: none;
    border: none;
    cursor: pointer;
    padding: 2px 5px;
    border-radius: 3px;
    font-size: 0.85rem;
    opacity: 0;
    transition: opacity 0.15s, background 0.15s;
  }

  .tr:hover .action-btn {
    opacity: 0.7;
  }

  .action-btn:hover {
    opacity: 1 !important;
  }

  .btn-term:hover { background: rgba(255, 170, 0, 0.2); color: var(--color-warning); }
  .btn-kill:hover { background: rgba(255, 60, 60, 0.2); color: var(--color-danger); }

  .empty-state {
    padding: 40px;
    text-align: center;
    color: var(--text-muted);
    font-size: 0.88rem;
  }
</style>
