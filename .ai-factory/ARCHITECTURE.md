# Architecture: Layered + Command/Event (Tauri)

## Overview
This project uses a **Layered Architecture** adapted for a Tauri desktop application. The Rust backend is divided into clear layers (command handlers → domain services → data collectors), while the Svelte frontend follows a **store-centric reactive** pattern.

The architecture was chosen because:
- The domain logic is straightforward (list/filter/sort/kill processes) — no need for DDD complexity.
- Tauri's command/event model naturally enforces a clean separation between frontend and backend.
- Performance demands (3 000+ process rows, 500 ms refresh) require aggregation on the Rust side before sending to UI.

---

## Layer Map

```
┌─────────────────────────────────────────────────┐
│  Frontend (Svelte 5 / TypeScript)               │
│  ┌──────────┐  ┌──────────┐  ┌──────────────┐  │
│  │  Routes  │  │Components│  │   Stores     │  │
│  │+page.svelte  │ Table    │  │ processStore │  │
│  │          │  │ FilterBar│  │ filterStore  │  │
│  └──────────┘  └──────────┘  └──────────────┘  │
│         ↕  Tauri invoke / Tauri.listen          │
├─────────────────────────────────────────────────┤
│  Backend (Rust / Tauri commands + events)       │
│  ┌──────────────────────────────────────────┐   │
│  │  Commands Layer (src/commands/)           │   │
│  │  list_processes · kill_process · details  │   │
│  └────────────────┬─────────────────────────┘   │
│  ┌────────────────▼─────────────────────────┐   │
│  │  Domain / Service Layer                   │   │
│  │  ProcessFilter · Sorter · DiffEngine      │   │
│  └────────────────┬─────────────────────────┘   │
│  ┌────────────────▼─────────────────────────┐   │
│  │  Collector Layer (src/collector/)         │   │
│  │  sysinfo_collector + platform/ specifics  │   │
│  └──────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────┐   │
│  │  Updater (background Tokio task)          │   │
│  │  Polls collector, diffs, emits events     │   │
│  └──────────────────────────────────────────┘   │
└─────────────────────────────────────────────────┘
```

---

## Module Responsibilities

### Frontend

| Module | Responsibility |
|--------|---------------|
| `routes/+page.svelte` | Page layout; wire stores to components |
| `components/ProcessTable/` | Virtualised row renderer; keyboard nav |
| `components/FilterBar/` | Search input, filter dropdowns, sort state |
| `components/DetailPanel/` | Show selected process details + sparkline |
| `components/ConfirmModal/` | Kill/terminate confirmation gate |
| `stores/processStore.ts` | Receive events, hold canonical process list |
| `stores/filterStore.ts` | Filter state; derive visible list reactively |
| `stores/settingsStore.ts` | Refresh interval, column visibility, theme |
| `lib/api/` | Typed wrappers around `invoke()` and `listen()` |

### Backend

| Module | Responsibility |
|--------|---------------|
| `commands/processes.rs` | `list_processes`, `process_details` — reads from AppState |
| `commands/actions.rs` | `kill_process`, `terminate_process`, `open_path`, `copy_to_clipboard` |
| `collector/sysinfo_collector.rs` | Collect raw process list via `sysinfo` |
| `collector/platform/` | Platform-specific extensions (username on Linux, Windows handle, macOS BSD info) |
| `updater/mod.rs` | Tokio background task: poll → diff → emit `processes:update` event |
| `models/process.rs` | `ProcessDto`, `ProcessFilter`, `SortField`, `KillMode` |
| `error.rs` | `AppError` enum (NotFound, PermissionDenied, InvalidPid, …) |

---

## Data Flow

### Initial Load
```
Frontend page mount
  → invoke("list_processes", { filter, sort })
  → commands/processes.rs reads AppState.snapshot
  → returns Vec<ProcessDto>
  → processStore updates
  → FilterBar + ProcessTable render
```

### Live Updates (streaming)
```
Updater task (every 500 ms)
  → collector::collect()
  → diff against previous snapshot
  → if changed: emit("processes:update", ProcessUpdateEvent)
Frontend
  → listen("processes:update")
  → processStore applies diff patch
  → Svelte reactivity triggers table re-render (virtual rows only)
```

### Kill / Terminate
```
User clicks Terminate
  → ConfirmModal shown (PID, name, "needs elevated rights?" badge)
  → User confirms
  → invoke("kill_process", { pid, mode: "terminate" })
  → commands/actions.rs: platform-specific signal
  → Result<(), AppError> returned
  → on error: toast notification with reason
```

---

## API Contract (Tauri Commands)

### Commands (invoke)

```typescript
// List processes with optional filter + sort applied server-side
invoke<ProcessDto[]>("list_processes", {
  filter?: ProcessFilter,
  sort?: { field: SortField, direction: "asc" | "desc" }
})

// Full details for a single process
invoke<ProcessDetails>("process_details", { pid: number })

// Kill or terminate
invoke<void>("kill_process", { pid: number, mode: "terminate" | "kill" })

// Open directory of process executable in OS file manager
invoke<void>("open_path", { pid: number })

// Copy string to clipboard
invoke<void>("copy_to_clipboard", { text: string })

// Update refresh interval
invoke<void>("set_refresh_interval", { ms: number })
```

### Events (listen)

```typescript
// Emitted by updater on each cycle when any change detected
listen<ProcessUpdateEvent>("processes:update", handler)

// Emitted when a process that was being watched disappears
listen<{ pid: number }>("process:gone", handler)
```

---

## Data Models

### ProcessDto
```typescript
interface ProcessDto {
  pid: number
  name: string
  status: "running" | "sleeping" | "stopped" | "zombie" | "unknown"
  cpu_percent: number          // 0.0 – 100.0 per core
  memory_bytes: number         // RSS in bytes
  user: string | null          // null if unprivileged or not available
  path: string | null          // null if kernel thread or access denied
  parent_pid: number | null    // null for root processes
  start_time: number | null    // Unix timestamp, null if unavailable
  needs_elevation: boolean     // true if owned by different user / root
}
```

### ProcessFilter
```typescript
interface ProcessFilter {
  search?: string              // substring match on name, path, pid
  user?: string                // exact match
  mine_only?: boolean
  system_only?: boolean
  non_system_only?: boolean
  status?: ProcessStatus
  cpu_gt?: number              // percentage threshold
  memory_gt_bytes?: number
}
```

### ProcessUpdateEvent
```typescript
interface ProcessUpdateEvent {
  added: ProcessDto[]
  updated: ProcessDto[]        // changed fields only, keyed by pid
  removed: number[]            // pids that disappeared
  timestamp: number            // ms since epoch
}
```

### AppError (Rust → JSON)
```typescript
type AppError =
  | { type: "NotFound"; pid: number }
  | { type: "PermissionDenied"; pid: number; message: string }
  | { type: "InvalidPid"; pid: number }
  | { type: "OsError"; message: string }
  | { type: "Unsupported"; feature: string }
```

---

## Dependency Rules

- **Frontend MUST NOT** contain sorting/filtering logic over the full process list — delegate to backend via filter/sort params.
- **Commands layer MUST NOT** access `sysinfo` directly — go through `collector/`.
- **Updater MUST NOT** block the Tauri main thread — run in a dedicated Tokio task.
- **`open_path` and `kill_process` MUST** be gated behind `tauri::command` with the user's explicit invocation (never auto-called).

---

## Performance Guidelines

- Use `Arc<Mutex<ProcessSnapshot>>` in AppState — shared between updater task and command handlers.
- Diff algorithm: compare PIDs first (O(n) with hashmap) before field-level comparison.
- Frontend virtual list: render only rows in viewport ± 5 overscroll rows.
- Avoid `JSON.parse` of full 3 000-process arrays on every tick — apply incremental patch.
- Throttle frontend re-renders: batch multiple rapid events within a 16 ms animation frame.

---

## Implementation Roadmap

### MVP (v0.1)
1. Tauri v2 project scaffold (Svelte + Vite)
2. `sysinfo` collector — PID, name, CPU%, RAM, status, path, user
3. `list_processes` command with server-side filter + sort
4. Background updater emitting `processes:update` events
5. Svelte virtualised table + filter bar + search
6. Kill/terminate commands with ConfirmModal
7. Copy PID / Copy path / Open location actions
8. Keyboard shortcuts: `/`, `Del`, `F5`
9. Pause/Resume/Refresh-now controls
10. README + troubleshooting guide

### v1
1. Process tree view (toggle from flat list)
2. CPU/RAM sparkline chart for selected process (60 s ring buffer in Rust)
3. Pin / favourite processes (persistent via Tauri store plugin)
4. System tray icon + global hotkey

### v1.1
1. Snapshot & diff screen
2. Notification alerts (process > threshold for > N seconds)
3. CSV / JSON export
4. Theme toggle (dark/light) + RU/EN localisation
