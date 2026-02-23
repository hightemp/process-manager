// ────────────────────────────────────────────────────────────────────────────
// Settings store (Svelte 5 runes)
// ────────────────────────────────────────────────────────────────────────────

// Visible columns definition
export type ColumnId = 'pid' | 'name' | 'user' | 'cpu' | 'memory' | 'status' | 'path' | 'parent';

export interface ColumnDef {
  id: ColumnId;
  label: string;
  width: string;
  visible: boolean;
}

function createSettingsStore() {
  let refreshIntervalMs = $state(1000);
  let paused = $state(false);
  let theme = $state<'dark' | 'light'>('dark');

  let columns = $state<ColumnDef[]>([
    { id: 'pid',    label: 'PID',    width: '70px',  visible: true },
    { id: 'name',   label: 'Name',   width: '200px', visible: true },
    { id: 'user',   label: 'User',   width: '100px', visible: true },
    { id: 'cpu',    label: 'CPU%',   width: '80px',  visible: true },
    { id: 'memory', label: 'RAM',    width: '90px',  visible: true },
    { id: 'status', label: 'Status', width: '90px',  visible: true },
    { id: 'path',   label: 'Path',   width: '1fr',   visible: false },
    { id: 'parent', label: 'PPID',   width: '70px',  visible: false },
  ]);

  function toggleColumn(id: ColumnId) {
    columns = columns.map((c) => (c.id === id ? { ...c, visible: !c.visible } : c));
  }

  function visibleColumns() {
    return columns.filter((c) => c.visible);
  }

  return {
    get refreshIntervalMs() { return refreshIntervalMs; },
    set refreshIntervalMs(v: number) { refreshIntervalMs = v; },
    get paused() { return paused; },
    set paused(v: boolean) { paused = v; },
    get theme() { return theme; },
    set theme(v: 'dark' | 'light') { theme = v; },
    get columns() { return columns; },
    toggleColumn,
    visibleColumns,
  };
}

export const settingsStore = createSettingsStore();
