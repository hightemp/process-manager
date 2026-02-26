# AGENTS.md

> Project map for AI agents. Keep this file up-to-date as the project evolves.

## Project Overview
A cross-platform (Windows/macOS/Linux) desktop process manager built with Tauri v2 (Rust backend) and Svelte 5 (TypeScript frontend). Designed for high performance: virtualised list, incremental diff updates from Rust, sub-500 ms refresh cycles.

## Tech Stack
- **Language (backend):** Rust (stable)
- **Language (frontend):** TypeScript + Svelte 5
- **Framework:** Tauri v2
- **Build tool:** Vite
- **Process data:** `sysinfo` crate + platform-specific extensions
- **Virtualisation:** Custom windowed list renderer

## Project Structure
```
process-manager/
├── src-tauri/               # Rust backend (Tauri app)
│   ├── src/
│   │   ├── main.rs          # Entry point, Tauri builder setup
│   │   ├── lib.rs           # App state, command registration
│   │   ├── commands/        # Tauri command handlers
│   │   │   ├── mod.rs
│   │   │   ├── processes.rs # list_processes, process_details
│   │   │   └── actions.rs   # kill_process, open_path
│   │   ├── models/          # Data transfer objects
│   │   │   ├── mod.rs
│   │   │   └── process.rs   # ProcessDto, ProcessFilter, SortField
│   │   ├── collector/       # sysinfo + platform-specific collectors
│   │   │   ├── mod.rs
│   │   │   ├── sysinfo_collector.rs
│   │   │   └── platform/    # Windows/macOS/Linux specifics
│   │   ├── updater/         # Background refresh loop, event emitter
│   │   │   └── mod.rs
│   │   └── error.rs         # AppError enum
│   ├── Cargo.toml
│   ├── tauri.conf.json      # Tauri v2 configuration
│   └── capabilities/        # Tauri v2 capability files
│       └── default.json
├── src/                     # Svelte 5 frontend
│   ├── app.html
│   ├── app.css
│   ├── lib/
│   │   ├── api/             # Tauri invoke wrappers
│   │   │   ├── processes.ts
│   │   │   └── actions.ts
│   │   ├── components/
│   │   │   ├── ProcessTable/ # Virtualised process list
│   │   │   ├── FilterBar/    # Search + filter controls
│   │   │   ├── DetailPanel/  # Selected process details
│   │   │   ├── ConfirmModal/ # Kill/terminate confirmation
│   │   │   ├── Sparkline/    # Mini CPU/RAM chart
│   │   │   └── ui/           # Reusable low-level UI primitives (Checkbox, …)
│   │   ├── stores/           # Svelte stores (process list, filters, settings)
│   │   └── utils/            # Formatting helpers, keyboard shortcuts
│   └── routes/
│       └── +page.svelte      # Main process manager page
├── .ai-factory/
│   ├── DESCRIPTION.md        # Project specification
│   └── ARCHITECTURE.md       # Architecture decisions
├── .agents/skills/           # Installed agent skills
│   ├── tauri-v2/
│   └── rust-async-patterns/
├── .mcp.json                 # MCP server configuration
├── AGENTS.md                 # This file
├── README.md                 # Setup, dev, build instructions
├── package.json              # Frontend deps
└── vite.config.ts
```

## Key Entry Points
| File | Purpose |
|------|---------|
| `src-tauri/src/main.rs` | Rust app entry point |
| `src-tauri/src/lib.rs` | Tauri builder, state init, command registration |
| `src-tauri/src/commands/processes.rs` | Core `list_processes` and `process_details` commands |
| `src-tauri/src/updater/mod.rs` | Background refresh loop emitting events |
| `src/routes/+page.svelte` | Main UI page |
| `src/lib/stores/` | Central reactive state (process list, filters) |
| `src-tauri/tauri.conf.json` | Tauri app config (identifier, permissions) |

## Documentation
| Document | Path | Description |
|----------|------|-------------|
| README | README.md | Installation, dev mode, build, OS requirements |
| Project spec | .ai-factory/DESCRIPTION.md | Tech stack, features, NFRs |
| Architecture | .ai-factory/ARCHITECTURE.md | Architecture pattern and module boundaries |

## AI Context Files
| File | Purpose |
|------|---------|
| AGENTS.md | This file — project structure map |
| .ai-factory/DESCRIPTION.md | Project specification and tech stack |
| .ai-factory/ARCHITECTURE.md | Architecture decisions and guidelines |

## Environment Variables
| Variable | Used by | Purpose |
|----------|---------|---------|
| `RUST_LOG` | Rust backend | Log level (e.g. `info`, `debug`) |
| `GITHUB_TOKEN` | MCP github server | GitHub API access |

## Installed Skills
| Skill | Path | Purpose |
|-------|------|---------|
| tauri-v2 | .agents/skills/tauri-v2 | Tauri v2 API patterns and best practices |
| rust-async-patterns | .agents/skills/rust-async-patterns | Rust async/tokio patterns |
