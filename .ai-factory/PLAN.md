# GitHub Actions: Multi-Platform Release Workflow

**Date:** 2026-02-26  
**Type:** chore / CI  
**Plan file:** `.ai-factory/PLAN.md`

---

## Settings

| Option | Value |
|--------|-------|
| Tests | No |
| Logging | N/A (CI only) |
| Docs | Update README |

---

## Context

The project already has three workflows:
- `.github/workflows/build.yml` — builds on push to `main` (Linux/macOS/Windows x86_64)
- `.github/workflows/lint.yml` — linting
- `.github/workflows/tests.yml` — tests

**Goal:** add a dedicated release workflow triggered by `v*` tag pushes that builds Tauri bundles for all target platforms and attaches them to a GitHub Release (draft).

**Target platforms:**
| Platform | Runner | Rust target |
|----------|--------|-------------|
| Linux x86_64 | `ubuntu-latest` | `x86_64-unknown-linux-gnu` |
| Linux aarch64 | `ubuntu-24.04-arm` | `aarch64-unknown-linux-gnu` |
| Windows x86_64 | `windows-latest` | `x86_64-pc-windows-msvc` |
| Windows aarch64 | `windows-latest` (cross) | `aarch64-pc-windows-msvc` |
| macOS Apple Silicon | `macos-14` | `aarch64-apple-darwin` |

> macOS Intel (x86_64) excluded intentionally — macOS 13 (last Intel runner) is EOL and not supported.

---

## Tasks

### Phase 1 — Release Workflow

#### [x] Task 1 — Create `.github/workflows/release.yml`

**File:** `.github/workflows/release.yml`

Create a new GitHub Actions workflow with the following specification:

**Trigger:**
```yaml
on:
  push:
    tags:
      - 'v*'
  workflow_dispatch:
```

**Permissions:** `contents: write` (required to create GitHub Releases and upload assets).

**Job matrix** — one job per platform (see table above). Use `fail-fast: false` so one platform failure doesn't cancel others.

**Steps per job:**

1. `actions/checkout@v4`
2. **Linux only** — install `libgtk-3-dev`, `libwebkit2gtk-4.1-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`, `patchelf` via `apt-get`. Use `if: startsWith(matrix.os, 'ubuntu')`.
3. `actions/setup-node@v4` with `node-version: 22`, `cache: npm`
4. `npm ci`
5. `dtolnay/rust-toolchain@stable` with `targets: ${{ matrix.rust_target }}` to pre-install the correct target toolchain.
6. `Swatinem/rust-cache@v2` with `workspaces: src-tauri -> target` — cache keyed by platform + target.
7. `tauri-apps/tauri-action@v0` with:
   - `tagName: ${{ github.ref_name }}`
   - `releaseName: 'Process Manager ${{ github.ref_name }}'`
   - `releaseBody: 'See the assets below to download and install Process Manager.'`
   - `releaseDraft: true` — publish as draft so maintainer can review before making it public
   - `prerelease: false`
   - `args: ${{ matrix.args }}` — each matrix entry provides its own `--target <rust-target>`

**Matrix entries:**
```yaml
matrix:
  include:
    - platform: linux-x86_64
      os: ubuntu-latest
      rust_target: x86_64-unknown-linux-gnu
      args: "--target x86_64-unknown-linux-gnu"

    - platform: linux-aarch64
      os: ubuntu-24.04-arm
      rust_target: aarch64-unknown-linux-gnu
      args: "--target aarch64-unknown-linux-gnu"

    - platform: windows-x86_64
      os: windows-latest
      rust_target: x86_64-pc-windows-msvc
      args: "--target x86_64-pc-windows-msvc"

    - platform: windows-aarch64
      os: windows-latest
      rust_target: aarch64-pc-windows-msvc
      args: "--target aarch64-pc-windows-msvc"

    - platform: macos-aarch64
      os: macos-14
      rust_target: aarch64-apple-darwin
      args: "--target aarch64-apple-darwin"
```

**Logging / debug hints to add as comments in the YAML:**
- Comment above the Linux dep step explaining why specific libs are needed (Tauri WebKit2GTK)
- Comment above the `rust_target` field explaining cross-compilation for Windows aarch64

**Expected output:** `.github/workflows/release.yml` — fully functional YAML file.

---

#### [x] Task 2 — Update `README.md` with release instructions

**File:** `README.md`

Add (or update) a `## Releases` section that explains:
- How to create a new release: `git tag v0.x.x && git push origin v0.x.x`
- What the workflow builds and attaches to the GitHub Release
- How to find and download releases from the GitHub Releases page
- A note that macOS Intel is not supported (macOS 13 / x86_64 runner is EOL)

**Log requirement:** None (documentation only).

---

## Commit Plan

*Less than 5 tasks — single commit after completion:*

```
git commit -m "ci: add multi-platform GitHub Actions release workflow"
```

---

## Notes & Edge Cases

- **Windows aarch64 cross-compile:** `tauri-apps/tauri-action` with `--target aarch64-pc-windows-msvc` cross-compiles on `windows-latest` (x86_64). The NSIS/WiX installer bundles WebView2 separately so cross-compilation is supported. If the build fails, consider switching to a native `windows-11-arm` runner (currently in beta).
- **Linux aarch64 native runner:** `ubuntu-24.04-arm` is a GitHub-hosted ARM64 runner. All webkit/GTK packages must install for arm64 architecture — the standard apt packages resolve correctly on ARM.
- **`releaseDraft: true`:** Releases are created as drafts to avoid accidental public pre-release artefacts. Publish manually from the GitHub Releases UI after reviewing all assets.
- **`GITHUB_TOKEN` secret:** Automatically available in all GitHub Actions — no extra secret configuration needed.
- **`tauri-apps/tauri-action@v0`:** Uses the floating `v0` tag; pin to a specific version (e.g. `v0.5.17`) for reproducibility if needed.
