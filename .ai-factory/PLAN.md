# Frameless Window with Custom Windows-Style Titlebar

**Date:** 2026-02-26
**Mode:** Fast
**Plan file:** `.ai-factory/PLAN.md`

---

## Settings

| Option | Value |
|--------|-------|
| Tests | No |
| Logging | Verbose (DEBUG) |
| Docs | No |

---

## Overview

Remove the OS-native window frame and replace it with a fully custom Svelte titlebar component styled in the Windows look-and-feel:
- Drag region on the left (app icon + title text)
- Three control buttons on the right: minimize `─`, maximize/restore `□`/`❐`, close `✕`
- Close button turns red (`#c42b1c`) on hover — standard Windows behavior
- Double-click on drag region toggles maximize

Implemented with:
- `"decorations": false` in `tauri.conf.json`
- `data-tauri-drag-region` HTML attribute for the drag area
- `@tauri-apps/api/window` JS API for window controls
- Explicit capability permissions for all window actions

---

## Tasks

### Phase 1 — Tauri Configuration

#### Task 1 — Disable system window decorations
**File:** `src-tauri/tauri.conf.json`

Set `"decorations": false` on the main window entry inside `app.windows[0]`.
This removes the OS frame (titlebar, borders, native buttons).
Keep `"resizable": true` — Tauri still provides resize handles even without decorations.

```json
// In app.windows[0]:
"decorations": false
```

**Logging:** Config-only change — no runtime logging required.

---

#### Task 2 — Add window management permissions
**File:** `src-tauri/capabilities/default.json`

Add the following permissions to the `permissions` array so the frontend JS can call window APIs.
Without these, every window API call silently fails with a permission-denied error at runtime.

Permissions to add:
- `"core:window:allow-minimize"`
- `"core:window:allow-maximize"`
- `"core:window:allow-toggle-maximize"`
- `"core:window:allow-close"`
- `"core:window:allow-is-maximized"`
- `"core:window:allow-start-dragging"`

**Logging:** Config-only change — no runtime logging required.

---

### Phase 2 — Frontend: TitleBar Component

#### Task 3 — Create `TitleBar.svelte` component
**File:** `src/lib/components/TitleBar.svelte` *(new)*

Custom Windows-style titlebar. Visual layout:
```
[⚙ icon]  Process Manager              [─]  [□]  [✕]
```

Implementation:
- Outer `<div class="titlebar" data-tauri-drag-region>` — full-width, `height: 32px`
- Import `getCurrentWindow` from `@tauri-apps/api/window`; store as `const win = getCurrentWindow()`
- `onMount`: call `win.isMaximized()` → set `isMaximized` state; subscribe to `win.onResized(...)` to keep `isMaximized` in sync
- `onDestroy`: unsubscribe from resize listener
- Double-click on drag region: `ondblclick={() => win.toggleMaximize()}`
- Button handlers:
  - Minimize: `win.minimize()` → `console.debug('[titlebar] minimize clicked')`
  - Maximize/Restore: `win.toggleMaximize()` → `console.debug('[titlebar] toggleMaximize clicked, was:', isMaximized)`
  - Close: `win.close()` → `console.debug('[titlebar] close clicked')`

CSS (scoped):
| Property | Value |
|----------|-------|
| Height | `32px` |
| Background | `var(--surface-2)` |
| Border-bottom | `1px solid var(--border)` |
| `user-select` | `none` |
| Button size | `46px × 32px` (Windows standard) |
| Button hover bg | `var(--surface-hover)` |
| Close hover bg | `#c42b1c` |
| Close hover color | `#ffffff` |

Icons: use SVG inline icons for crisp rendering at all scales (not unicode glyphs which vary by font):
- Minimize: horizontal line centered
- Maximize: hollow square
- Restore: two overlapping squares
- Close: diagonal cross (×)

**Verbose logging:**
```typescript
console.debug('[titlebar] mounted, isMaximized:', isMaximized);
console.debug('[titlebar] resize event → isMaximized:', isMaximized);
console.debug('[titlebar] minimize clicked');
console.debug('[titlebar] toggleMaximize clicked, was:', isMaximized);
console.debug('[titlebar] close clicked');
```

---

### Phase 3 — Layout Integration

#### Task 4 — Integrate TitleBar into `+page.svelte` and fix layout
**Files:** `src/routes/+page.svelte`, `src/app.css`

1. Import `TitleBar` and render it at the very top of the markup, above the main content wrapper.
2. Adjust the app layout to account for the `32px` titlebar:
   - The root app wrapper uses `display: flex; flex-direction: column; height: 100vh`
   - TitleBar: `flex: 0 0 32px`
   - Main content area: `flex: 1; min-height: 0; overflow: hidden`
3. Update `html, body` in `app.css` to ensure `height: 100%; overflow: hidden` (already set, verify nothing breaks).
4. Remove any `padding-top` workarounds that existed for the native titlebar.

**Verbose logging:**
```svelte
// in onMount:
console.debug('[page] TitleBar integrated, layout ready');
```

---

## Commit Plan

*4 tasks — single commit after completion:*

```
feat(ui): frameless window with custom Windows-style titlebar

- Set decorations: false in tauri.conf.json
- Add window permissions (minimize/maximize/toggle/close/is-maximized)
- Create TitleBar.svelte with drag region and win32-style SVG buttons
- Integrate TitleBar into page layout with correct flex sizing
```

---

## Affected Files

| File | Change |
|------|--------|
| `src-tauri/tauri.conf.json` | Modified — add `"decorations": false` |
| `src-tauri/capabilities/default.json` | Modified — add 6 window permissions |
| `src/lib/components/TitleBar.svelte` | **New** — custom titlebar component |
| `src/routes/+page.svelte` | Modified — import TitleBar, adjust layout |
| `src/app.css` | Modified — flex layout for root app wrapper |

---

## Notes & Risks

- **Linux / Wayland:** Some compositors (GNOME Shell + Wayland) may not show resize cursors for frameless windows. Tauri's `resizable: true` generally handles this, but test manually.
- **macOS:** `decorations: false` hides the traffic lights. The custom buttons work, but macOS users lose native window chrome. If macOS parity becomes important later, use `titleBarStyle: "overlay"` with conditional rendering of custom buttons only on non-macOS.
- **Double-click on titlebar:** Standard Windows UX — implement `ondblclick` on the drag region to call `toggleMaximize()`.
- **App region vs. drag region:** Do NOT put interactive elements (buttons, inputs) inside the `data-tauri-drag-region` div — they will not receive click events. Keep buttons outside or in a separate non-drag section.
