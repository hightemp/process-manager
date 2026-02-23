# Replace Icons with Lucide — Implementation Plan

**Branch:** `feature/lucide-icons`  
**Created:** 2026-02-23  
**Mode:** Full

---

## Settings

| Option   | Value    |
|----------|----------|
| Testing  | No       |
| Logging  | Verbose  |
| Docs     | No       |

---

## Context

The current UI uses a mix of Unicode emoji (`🔍`, `☠`, `🔒`, `⚠️`), plain Unicode arrows (`▲`, `▼`, `↻`), and basic symbols (`✕`, `▶`, `⏸`, `⚙`) as icons. These look inconsistent and unpresentable across different platforms and fonts.

**Goal:** Replace all emoji/unicode icons with `lucide-svelte` SVG components for a clean, consistent, professional appearance.

### Current Icon Inventory

| Component | Current | Replace With |
|-----------|---------|-------------|
| `FilterBar.svelte` | `🔍` search | `Search` |
| `FilterBar.svelte` | `✕` clear | `X` |
| `FilterBar.svelte` | `▶ Resume` / `⏸ Pause` | `Play` / `Pause` |
| `ProcessTable.svelte` | ` ▲` / ` ▼` sort | `ChevronUp` / `ChevronDown` |
| `ProcessTable.svelte` | `🔒` elevated rights | `Lock` |
| `ProcessTable.svelte` | `✕` terminate | `X` |
| `ProcessTable.svelte` | `☠` kill | `Skull` |
| `+page.svelte` | `⚙` app icon | `Monitor` |
| `+page.svelte` | `↻` refresh | `RefreshCw` |
| `+page.svelte` | `☀` / `🌙` theme | `Sun` / `Moon` |
| `ConfirmModal.svelte` | `⚠️` warning | `AlertTriangle` |
| `DetailPanel.svelte` | `✕` close | `X` |
| `DetailPanel.svelte` | Copy PID button | `Copy` (prefix icon) |
| `DetailPanel.svelte` | Copy Path button | `Copy` (prefix icon) |
| `DetailPanel.svelte` | Open Location button | `FolderOpen` (prefix icon) |
| `Toast.svelte` | `✕` dismiss | `X` |

---

## Tasks

### Phase 1: Setup

#### Task 1 — Install lucide-svelte

**File:** `package.json`

Install the `lucide-svelte` package as a dev/runtime dependency.

```bash
npm install lucide-svelte
```

Verify the package is available and imports work correctly by checking that icons like `Search`, `X`, `Play`, `Pause` are exported from `lucide-svelte`.

**Logging:**
- Log to console: `[Icons] lucide-svelte installed, version: <version>`
- Add `// DEBUG: icon imports verified` comment in the first component that uses it

---

### Phase 2: Component Updates

#### Task 2 — Update FilterBar.svelte

**File:** `src/lib/components/FilterBar.svelte`

Replace:
- `<span class="search-icon">🔍</span>` → `<Search size={16} class="search-icon" />`
- `✕` in clear button → `<X size={14} />`
- `▶ Resume` → `<Play size={14} /> Resume`
- `⏸ Pause` → `<Pause size={14} /> Pause`

**Import at top of script block:**
```ts
import { Search, X, Play, Pause } from 'lucide-svelte';
```

Remove the `.search-icon` CSS rule for `font-size`/`opacity` styling — adjust sizing via `size` prop and `stroke-width` on the SVG component.
Add a flex layout to the pause button label so icon and text align inline: `display: flex; align-items: center; gap: 6px`.

**Logging:** Add `// DEBUG: FilterBar icons replaced with lucide-svelte: Search, X, Play, Pause`

---

#### Task 3 — Update ProcessTable.svelte

**File:** `src/lib/components/ProcessTable.svelte`

Replace:
- Sort indicator text ` ▲` / ` ▼` → render `<ChevronUp size={12} />` / `<ChevronDown size={12} />` inline next to column header text
- `🔒` elevated rights badge → `<Lock size={12} class="lock-icon" />`
- `✕` terminate button → `<X size={14} />`
- `☠` kill button → `<Skull size={14} />`

**Import:**
```ts
import { ChevronUp, ChevronDown, Lock, X, Skull } from 'lucide-svelte';
```

The sort indicator is currently returned as a string suffix from a function. Change the sort column header rendering to conditionally show the icon component instead of a text suffix.

Remove the `lock-icon` CSS `font-size` override (it was for emoji sizing); adjust via `size` prop.

**Logging:** Add `// DEBUG: ProcessTable icons replaced: ChevronUp/Down, Lock, X, Skull`

---

#### Task 4 — Update +page.svelte (header icons)

**File:** `src/routes/+page.svelte`

Replace:
- `<span class="app-icon">⚙</span>` → `<Monitor size={22} class="app-icon" />`
- `↻` refresh button → `<RefreshCw size={16} />`
- `{settingsStore.theme === 'dark' ? '☀' : '🌙'}` theme button → `{#if settingsStore.theme === 'dark'}<Sun size={16} />{:else}<Moon size={16} />{/if}`

**Import:**
```ts
import { Monitor, RefreshCw, Sun, Moon } from 'lucide-svelte';
```

Remove `.app-icon` `font-size` sizing CSS and replace with appropriate `color` / `vertical-align` via SVG styling if needed.

**Logging:** Add `// DEBUG: +page.svelte header icons replaced: Monitor, RefreshCw, Sun, Moon`

---

#### Task 5 — Update ConfirmModal.svelte

**File:** `src/lib/components/ConfirmModal.svelte`

Replace:
- `⚠️ Force Kill Process` / `⚠️ Terminate...` title prefix → `<AlertTriangle size={18} class="modal-warn-icon" />` placed before the title text
- `⚠️ This process belongs to another user...` inline warning → `<AlertTriangle size={14} class="inline-warn" />` before the warning text

**Import:**
```ts
import { AlertTriangle } from 'lucide-svelte';
```

Add CSS for `.modal-warn-icon` and `.inline-warn` to set color to `var(--color-warning)` and `vertical-align: middle`.

**Logging:** Add `// DEBUG: ConfirmModal icons replaced: AlertTriangle`

---

#### Task 6 — Update DetailPanel.svelte

**File:** `src/lib/components/DetailPanel.svelte`

Replace:
- `✕` close button → `<X size={16} />`
- Add `<Copy size={13} />` prefix icon to **Copy PID** and **Copy Path** buttons
- Add `<FolderOpen size={13} />` prefix icon to **Open Location** button

**Import:**
```ts
import { X, Copy, FolderOpen } from 'lucide-svelte';
```

Ensure buttons with icon + text use `display: flex; align-items: center; gap: 5px` layout.

**Logging:** Add `// DEBUG: DetailPanel icons replaced: X, Copy, FolderOpen`

---

#### Task 7 — Update Toast.svelte

**File:** `src/lib/components/Toast.svelte`

Replace:
- `✕` dismiss button → `<X size={14} />`

**Import:**
```ts
import { X } from 'lucide-svelte';
```

**Logging:** Add `// DEBUG: Toast dismiss icon replaced: X`

---

### Phase 3: Polish

#### Task 8 — CSS cleanup and visual consistency

After all components are updated, do a final pass:

1. **Global icon color** — Lucide SVGs inherit `currentColor` by default. Ensure no hardcoded colors conflict. Check `stroke` and `fill` rendering in both dark and light themes.
2. **Stroke width** — Default Lucide `stroke-width` is 2. For small icons (12–14px), consider `stroke-width={1.75}` for clarity.
3. **Alignment** — Where icons appear alongside text, confirm `vertical-align: middle` or flex row layout is applied consistently.
4. **Remove dead CSS** — Delete CSS rules that were only needed for font-size/emoji sizing (`.search-icon font-size`, `.lock-icon font-size`, `.app-icon font-size`, etc.).
5. **Visual review** — Run `npm run dev` and visually inspect all changed components in both dark and light mode.

**Logging:** Add `// DEBUG: CSS cleanup complete — dead icon CSS rules removed`

---

## Commit Plan

| Checkpoint | Tasks | Commit Message |
|------------|-------|---------------|
| 1 | Task 1 | `chore: install lucide-svelte icon library` |
| 2 | Tasks 2–4 | `feat(ui): replace emoji icons with lucide-svelte in FilterBar, ProcessTable, page header` |
| 3 | Tasks 5–7 | `feat(ui): replace emoji icons with lucide-svelte in ConfirmModal, DetailPanel, Toast` |
| 4 | Task 8 | `style(ui): css cleanup after lucide icon migration` |

---

## Notes

- `lucide-svelte` exports each icon as an individual Svelte component. Tree-shaking ensures only used icons are bundled.
- All icons use `currentColor` for stroke, inheriting text color automatically — no extra CSS needed for theme compatibility.
- `size` prop controls both `width` and `height`. Default is 24; use 12–18 for inline/small contexts.
- If the Skull icon is not available in lucide-svelte, use `Crosshair` or `Bomb` as an alternative for the kill action.
