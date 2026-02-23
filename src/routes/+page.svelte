<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import type { ProcessDto } from '$lib/types';
  import { processStore, selectionStore } from '$lib/stores/processStore.svelte';
  import { filterStore } from '$lib/stores/filterStore.svelte';
  import { settingsStore } from '$lib/stores/settingsStore.svelte';
  import { listProcesses, onProcessesUpdate } from '$lib/api/processes';
  import { killProcess } from '$lib/api/actions';

  import FilterBar from '$lib/components/FilterBar.svelte';
  import ProcessTable from '$lib/components/ProcessTable.svelte';
  import DetailPanel from '$lib/components/DetailPanel.svelte';
  import ConfirmModal from '$lib/components/ConfirmModal.svelte';
  import Toast from '$lib/components/Toast.svelte';
  import type { Toast as ToastItem } from '$lib/components/Toast.svelte';
  import '../app.css';

  // ────────── State ──────────
  let loading = $state(true);
  let searchInputEl = $state<HTMLInputElement | undefined>();

  // Kill modal state
  let modalProcess = $state<ProcessDto | null>(null);
  let modalMode = $state<'terminate' | 'kill'>('terminate');

  // Toasts
  let toasts = $state<ToastItem[]>([]);
  let toastCounter = 0;

  function addToast(message: string, type: ToastItem['type'] = 'info') {
    const id = ++toastCounter;
    toasts = [...toasts, { id, message, type }];
    setTimeout(() => dismissToast(id), 4000);
  }

  function dismissToast(id: number) {
    toasts = toasts.filter((t) => t.id !== id);
  }

  // ────────── Derived: filtered + sorted list ──────────
  let visibleProcesses = $derived.by(() => {
    // The backend handles filter+sort when we invoke list_processes.
    // The event stream gives us the full snapshot; re-apply client-side
    // derived filtering for instant search responsiveness.
    const filter = filterStore.toFilter();
    const search = filter.search?.toLowerCase() ?? '';
    let list = processStore.list;

    if (search) {
      // [FIX] Only match on path when the search looks like a path
      // (contains / or \). Otherwise, match only on name and PID to
      // avoid flooding results with unrelated subprocesses that share
      // the same executable binary (e.g. all Chromium threads).
      const matchPath = search.includes('/') || search.includes('\\');
      list = list.filter((p) =>
        p.name.toLowerCase().includes(search) ||
        p.pid.toString().includes(search) ||
        (matchPath && (p.path?.toLowerCase().includes(search) ?? false))
      );
      console.debug('[FIX] search filter applied', { search, matchPath, resultCount: list.length });
    }
    if (filter.mine_only) {
      // [FIX] Check that user is known AND belongs to current user
      // (needs_elevation=false means same user, but user must not be null)
      list = list.filter((p) => p.user !== null && !p.needs_elevation);
    }
    if (filter.system_only) {
      list = list.filter((p) => p.pid < 500 || p.user === 'root' || p.user === 'SYSTEM');
    }
    if (filter.non_system_only) {
      list = list.filter((p) => !(p.pid < 500 || p.user === 'root' || p.user === 'SYSTEM'));
    }
    if (filter.cpu_gt !== undefined) {
      // [FIX] Use strict > to match the UI label "CPU >"
      list = list.filter((p) => p.cpu_percent > filter.cpu_gt!);
    }
    if (filter.memory_gt_bytes !== undefined) {
      // [FIX] Use strict > to match the UI label "RAM >"
      list = list.filter((p) => p.memory_bytes > filter.memory_gt_bytes!);
    }

    // Sort
    const { field, direction } = filterStore.toSort();
    const dir = direction === 'asc' ? 1 : -1;
    list = [...list].sort((a, b) => {
      switch (field) {
        case 'pid':          return dir * (a.pid - b.pid);
        case 'name':         return dir * a.name.localeCompare(b.name);
        case 'cpu_percent':  return dir * (a.cpu_percent - b.cpu_percent);
        case 'memory_bytes': return dir * (a.memory_bytes - b.memory_bytes);
        case 'user':         return dir * (a.user ?? '').localeCompare(b.user ?? '');
        case 'status':       return dir * a.status.localeCompare(b.status);
        default:             return 0;
      }
    });

    return list;
  });

  let selectedProcess = $derived(
    selectionStore.selectedPid != null
      ? processStore.processes.get(selectionStore.selectedPid) ?? null
      : null
  );

  // ────────── Lifecycle ──────────
  let unlisten: (() => void) | undefined;

  onMount(async () => {
    document.documentElement.setAttribute('data-theme', settingsStore.theme);

    // Initial load
    try {
      const initial = await listProcesses();
      processStore.setAll(initial);
    } catch (e) {
      addToast('Failed to load processes: ' + String(e), 'error');
    } finally {
      loading = false;
    }

    // Subscribe to incremental updates
    unlisten = await onProcessesUpdate((event) => {
      processStore.applyDiff(event);
      // If selected process was removed, deselect
      if (
        selectionStore.selectedPid != null &&
        event.removed.includes(selectionStore.selectedPid)
      ) {
        selectionStore.select(null);
        addToast('Process disappeared.', 'info');
      }
    });
  });

  onDestroy(() => {
    unlisten?.();
  });

  // ────────── Keyboard shortcuts ──────────
  function handleGlobalKeydown(e: KeyboardEvent) {
    const tag = (e.target as HTMLElement).tagName.toLowerCase();

    // '/' → focus search
    if (e.key === '/' && tag !== 'input' && tag !== 'textarea') {
      e.preventDefault();
      searchInputEl?.focus();
      return;
    }

    // F5 → force refresh
    if (e.key === 'F5') {
      e.preventDefault();
      handleRefreshNow();
      return;
    }

    // Escape → clear search or deselect
    if (e.key === 'Escape') {
      if (filterStore.search) {
        filterStore.search = '';
      } else {
        selectionStore.select(null);
      }
      return;
    }

    // Del → terminate selected process
    if (e.key === 'Delete' && selectionStore.selectedPid != null) {
      e.preventDefault();
      const proc = processStore.processes.get(selectionStore.selectedPid);
      if (proc) openModal(proc, 'terminate');
    }
  }

  async function handleRefreshNow() {
    try {
      const fresh = await listProcesses();
      processStore.setAll(fresh);
    } catch (e) {
      addToast('Refresh failed: ' + String(e), 'error');
    }
  }

  // ────────── Kill modal ──────────
  function openModal(proc: ProcessDto, mode: 'terminate' | 'kill') {
    modalProcess = proc;
    modalMode = mode;
  }

  async function handleConfirmKill(pid: number, mode: 'terminate' | 'kill') {
    modalProcess = null;
    try {
      await killProcess(pid, mode);
      addToast(`Process ${pid} ${mode === 'kill' ? 'killed' : 'terminated'}.`, 'success');
    } catch (e: any) {
      const msg = typeof e === 'string' ? e : (e?.data?.message ?? String(e));
      addToast(`Failed to ${mode} process ${pid}: ${msg}`, 'error');
    }
  }

  function cancelModal() {
    modalProcess = null;
  }

  // ────────── Theme toggle ──────────
  function toggleTheme() {
    settingsStore.theme = settingsStore.theme === 'dark' ? 'light' : 'dark';
    document.documentElement.setAttribute('data-theme', settingsStore.theme);
  }
</script>

<svelte:window onkeydown={handleGlobalKeydown} />

<div class="app">
  <!-- Top bar -->
  <header class="topbar">
    <div class="app-title">
      <span class="app-icon">⚙</span>
      <span>Process Manager</span>
    </div>
    <div class="topbar-actions">
      <button class="icon-btn" onclick={handleRefreshNow} title="Refresh now (F5)">↻</button>
      <button class="icon-btn" onclick={toggleTheme} title="Toggle theme">
        {settingsStore.theme === 'dark' ? '☀' : '🌙'}
      </button>
    </div>
  </header>

  <!-- Filter bar -->
  <FilterBar
    totalCount={processStore.list.length}
    visibleCount={visibleProcesses.length}
    bind:searchInputEl
  />

  <!-- Main content -->
  <div class="main-content">
    {#if loading}
      <div class="loading-state">Loading processes…</div>
    {:else}
      <ProcessTable
        processes={visibleProcesses}
        onKillRequest={openModal}
      />
    {/if}

    <!-- Detail panel -->
    {#if selectedProcess}
      <DetailPanel
        process={selectedProcess}
        onKillRequest={openModal}
        onClose={() => selectionStore.select(null)}
      />
    {/if}
  </div>
</div>

<!-- Kill / Terminate modal -->
<ConfirmModal
  process={modalProcess}
  mode={modalMode}
  onConfirm={handleConfirmKill}
  onCancel={cancelModal}
/>

<!-- Toast notifications -->
<Toast {toasts} onDismiss={dismissToast} />

<style>
  .app {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: var(--surface-1);
    color: var(--text-primary);
    overflow: hidden;
  }

  .topbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 12px;
    height: 40px;
    background: var(--surface-2);
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }

  .app-title {
    display: flex;
    align-items: center;
    gap: 6px;
    font-weight: 600;
    font-size: 0.9rem;
    color: var(--text-primary);
  }

  .app-icon {
    font-size: 1rem;
    opacity: 0.8;
  }

  .topbar-actions {
    display: flex;
    gap: 4px;
  }

  .icon-btn {
    background: none;
    border: 1px solid transparent;
    color: var(--text-muted);
    cursor: pointer;
    padding: 4px 8px;
    border-radius: 4px;
    font-size: 1rem;
    transition: all 0.15s;
  }

  .icon-btn:hover {
    border-color: var(--border);
    color: var(--text-primary);
  }

  .main-content {
    flex: 1;
    display: flex;
    overflow: hidden;
    min-height: 0;
  }

  .loading-state {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
    font-size: 0.9rem;
  }
</style>
