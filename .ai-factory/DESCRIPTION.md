# Project: Process Manager

## Overview
A cross-platform (Windows / macOS / Linux) desktop process manager application built with Tauri v2 + Rust backend and Svelte/TypeScript frontend. The primary goal is to provide a fast, responsive interface for monitoring, filtering, and controlling system processes — handling thousands of entries without lag.

## Core Features

### MVP (v0.1)
- Process list with PID, name, user, CPU%, RAM, status, path, parent PID
- String search by name / path / PID
- Filters: user, "mine only", system/non-system, state (running/sleeping), CPU> X%, RAM> Y MB
- Column sorting (ascending/descending) + persist column selection
- Terminate (SIGTERM / graceful) and Kill (SIGKILL / force) with confirmation modal
- Copy PID, Copy path to clipboard
- Open file location in OS file manager
- Auto-refresh every 500 ms with Pause / Resume / Refresh-now controls
- Keyboard shortcuts: `/` → focus search, `Del` → terminate with confirm, `F5` → refresh

### v1 Additions
- Process tree (parent/children hierarchy view)
- Mini CPU/RAM history chart per selected process (last 60 s)
- Pin / favourite processes
- System tray icon + global hotkey to show/hide window

### v1.1 Roadmap
- Snapshot & diff: show what appeared / disappeared, what grew by CPU/RAM
- Alerts/notifications: "process X > 80% CPU for 10 s"
- Export list as CSV / JSON
- Dark / light theme toggle
- Localisation: RU / EN

## Tech Stack
- **Language (backend):** Rust (stable toolchain)
- **Language (frontend):** TypeScript
- **Framework:** Tauri v2
- **UI framework:** Svelte 5 + Vite
- **Process data:** `sysinfo` crate (cross-platform baseline) + platform-specific additions where needed
- **Virtualised list:** `svelte-virtual-list` or custom windowed renderer
- **Charts:** lightweight-charts or custom SVG sparklines

## Architecture Notes
- Backend aggregates all heavy computation (sorting, filtering, diff) before sending to frontend.
- Frontend receives a pre-filtered, pre-sorted array — no JS-side sorting loops over thousands of items.
- Streaming updates via Tauri events (`emit`) with incremental diffs to avoid full re-renders.
- Kill / Terminate commands require explicit user confirmation and show PID + process name in modal.
- "Needs elevated rights" badge for processes owned by other users or root.

## Non-Functional Requirements
- Refresh cycle: 500 ms default, configurable 200 – 5000 ms.
- Startup time: < 1 s on modern hardware.
- List render: smooth scroll on 3,000+ processes (virtualised).
- Logging: structured via `tracing` crate; configurable `RUST_LOG`.
- Error handling: all Tauri commands return `Result<T, AppError>` with structured JSON errors.
- Security: dangerous actions (kill) require explicit user click on confirmation dialog; never auto-execute.
- OS-specific: graceful handling of UAC (Windows), codesign permissions (macOS), polkit (Linux).

## Constraints
- Use Tauri v2 stable.
- No Electron, no Node.js runtime in production.
- `sysinfo` as primary cross-platform process source; add platform APIs only where `sysinfo` falls short.
- Svelte 5 (runes syntax preferred) for UI reactivity.
