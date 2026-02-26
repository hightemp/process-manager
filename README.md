# Process Manager

A fast, cross-platform (Windows / macOS / Linux) desktop process manager built with **Tauri v2** + **Rust** backend and **Svelte 5** frontend.

[![Rust](https://img.shields.io/badge/Rust-stable-orange)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-v2-blue)](https://tauri.app)
[![Svelte](https://img.shields.io/badge/Svelte-5-FF3E00)](https://svelte.dev)
![Experimental](https://img.shields.io/badge/status-experimental-orange)
![Vibe Coded](https://img.shields.io/badge/vibe-coded-blueviolet)

---

## Features

- **Process list** — PID, name, user, CPU%, RAM, status, path, parent PID
- **Live updates** — incremental diff from Rust every 1 s (no full re-renders)
- **Search & filters** — substring search, "mine only", system/non-system, CPU > X%, RAM > Y MB
- **Sortable columns** — click any column header, persists direction
- **Actions** — Terminate (SIGTERM), Force Kill (SIGKILL), Copy PID, Copy path, Open file location
- **Safety** — kill actions require confirmation modal with PID + name; "Needs rights" badge on restricted processes
- **Keyboard shortcuts** — `/` focus search, `Del` terminate, `F5` refresh, `Esc` clear/deselect
- **Pause/Resume** — stop auto-refresh while you investigate
- **Virtualised list** — renders only visible rows; smooth scroll on 3,000+ processes

---

## Requirements

### All platforms
- [Node.js](https://nodejs.org/) 18+
- [Rust stable](https://rustup.rs/) 1.70+

### Linux
```bash
# Ubuntu/Debian
sudo apt install libwebkit2gtk-4.1-dev libxdo-dev libayatana-appindicator3-dev librsvg2-dev build-essential curl file libssl-dev libgtk-3-dev

# Fedora
sudo dnf install webkit2gtk4.1-devel libxdo-devel libayatana-appindicator-gtk3-devel librsvg2-devel openssl-devel
```

### macOS
```bash
xcode-select --install
```

### Windows
- [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (included with Windows 11)

---

## Development

```bash
# Install frontend dependencies
npm install

# Start development server (Tauri + Vite HMR)
npm run tauri dev
```

The Tauri window opens automatically. Hot-module reload works for the Svelte frontend. Rust changes require a full restart.

---

## Build

```bash
# Production build (creates platform bundle in src-tauri/target/release/bundle/)
npm run tauri build
```

Output locations:
- **Linux** — `.deb`, `.rpm`, `.AppImage`
- **macOS** — `.dmg`, `.app`
- **Windows** — `.msi`, `.exe` (NSIS)

---

## Testing

```bash
# Rust unit tests
cd src-tauri && cargo test

# Frontend type check
npm run check
```

---

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `/` | Focus search bar |
| `F5` | Force refresh process list |
| `Esc` | Clear search / deselect process |
| `Del` | Terminate selected process (with confirmation) |
| `Enter` | Confirm modal |
| `Esc` (modal) | Cancel modal |

---

## Configuration

| Setting | Default | Description |
|---------|---------|-------------|
| Refresh interval | 1000 ms | Configurable via dropdown: 500 ms – 5 s |
| `RUST_LOG` env var | `info` | Set to `debug` or `trace` for verbose backend logs |

---

## Troubleshooting

### Windows — Access Denied killing processes
Processes owned by SYSTEM or elevated services require admin rights. Run Process Manager as Administrator.

**UAC prompt:** Right-click the executable → "Run as administrator".

### macOS — Cannot kill processes
macOS SIP prevents even root from killing certain system processes. For user processes you may need **Full Disk Access** in System Settings → Privacy & Security.

### Linux — Permission denied (polkit)
Some distributions restrict signal delivery to processes owned by other users. Run with `sudo` or configure polkit to allow your user.

```bash
# Quick test with sudo
sudo RUST_LOG=info ./process-manager
```

---

## Project Structure

```
process-manager/
├── src-tauri/src/
│   ├── main.rs               Entry point
│   ├── lib.rs                Tauri builder, plugin registration
│   ├── state.rs              Shared AppState (Arc<Mutex<T>>)
│   ├── error.rs              AppError enum
│   ├── models/process.rs     ProcessDto, filter/sort helpers + unit tests
│   ├── collector/            sysinfo-based process collector
│   ├── updater/              Background refresh loop, event emitter
│   └── commands/             Tauri command handlers (list, kill, open)
└── src/
    ├── routes/+page.svelte   Main application page
    ├── lib/api/              Typed invoke/listen wrappers
    ├── lib/stores/           Svelte 5 rune-based reactive stores
    ├── lib/components/       UI components (table, filters, modals)
    └── lib/utils/            Formatting helpers
```

---

## Releases

Pre-built binaries for all supported platforms are attached to each [GitHub Release](../../releases).

### Supported platforms

| Platform | Architecture | Installer formats |
|----------|-------------|-------------------|
| Linux | x86_64 | `.deb`, `.rpm`, `.AppImage` |
| Linux | aarch64 | `.deb`, `.rpm`, `.AppImage` |
| Windows | x86_64 | `.msi`, `.exe` (NSIS) |
| Windows | aarch64 | `.msi`, `.exe` (NSIS) |
| macOS | Apple Silicon (aarch64) | `.dmg`, `.app` |

> **macOS Intel (x86_64) is not supported.** The `macos-13` GitHub Actions runner
> (the last Intel runner) is EOL and is no longer available.

### Creating a new release

```bash
# Tag the commit you want to release
git tag v0.2.0
git push origin v0.2.0
```

The [release workflow](.github/workflows/release.yml) triggers automatically,
builds all platform bundles in parallel, and creates a **draft** GitHub Release
with the compiled installers attached as assets.

Once all platform jobs finish, open the draft release on GitHub, review the
assets, edit the release notes if needed, and click **Publish release**.

---

## Roadmap

See [.ai-factory/ARCHITECTURE.md](.ai-factory/ARCHITECTURE.md#implementation-roadmap) for the full v1 / v1.1 plan.

![](https://asdertasd.site/counter/process-manager)