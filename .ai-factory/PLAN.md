# Custom Form Element Design

**Date:** 2026-02-26
**Mode:** Fast
**Tests:** No
**Plan file:** `.ai-factory/PLAN.md`

---

## Settings

| Option | Value |
|--------|-------|
| Tests | No |
| Logging | N/A (UI only) |

---

## Overview

Several form elements in `FilterBar.svelte` use default browser rendering:

- **Checkboxes** (`<input type="checkbox">`) — only `accent-color` applied; box shape and tick are OS-native
- **Select** (`<select class="interval-select">`) — has partial CSS but still shows the OS-native dropdown arrow
- **Number inputs** (`<input type="number" class="threshold-input">`) — has basic border/bg but native spin buttons are visible

Goal: replace/restyle all of these with fully customised, design-token-consistent controls that look identical on Linux, macOS and Windows.

---

## Tasks

### Phase 1 — Reusable component

#### [x] Task 1: Create `Checkbox.svelte` UI component

**Files:**
- `src/lib/components/ui/Checkbox.svelte` (new)
- `src/lib/components/ui/index.ts` (new — barrel export)

Build a custom checkbox from a visually-hidden `<input type="checkbox">` + styled `<span>` box:

- Props: `checked: boolean` (`$bindable()`), `disabled?: boolean`
- Visual box: `14×14 px`, `border: 1px solid var(--border)`, `border-radius: 3px`, `background: var(--surface-1)`
- Checked state: `background: var(--color-accent)`, white SVG checkmark via `::after` pseudo-element
- Hover (unchecked): `border-color: var(--color-accent)`
- Focus-visible: `outline: 2px solid var(--color-accent); outline-offset: 2px` (keyboard a11y)
- Transitions: `background 0.12s`, `border-color 0.12s`
- Real `<input>` is `position: absolute; opacity: 0; width: 0; height: 0`

---

### Phase 2 — FilterBar updates

#### [x] Task 2: Replace native checkboxes with `Checkbox.svelte`

**File:** `src/lib/components/FilterBar.svelte`

Import and swap all three `<label class="toggle"><input type="checkbox">` blocks
for the new `<Checkbox bind:checked={...} />` component.
Remove the old `.toggle input { accent-color }` CSS rule.

---

#### [x] Task 3: Fully custom `<select>` styling

**File:** `src/lib/components/FilterBar.svelte`

Apply `appearance: none` and inject a custom chevron SVG via `background-image`
so the dropdown arrow is design-token coloured and consistent cross-OS.

Add `:hover` and `:focus` states that match the search input (border turns accent colour).
Add a light-theme override for the chevron SVG fill colour.

---

#### [x] Task 4: Remove native spin buttons from number inputs

**File:** `src/lib/components/FilterBar.svelte`

Hide the browser-native spinner arrows with:
```css
.threshold-input::-webkit-inner-spin-button,
.threshold-input::-webkit-outer-spin-button { -webkit-appearance: none; margin: 0; }
.threshold-input { -moz-appearance: textfield; appearance: textfield; }
```

Add `:hover` (border → text-muted) and `:focus` (border → accent) states to
match the search input behaviour.

---

## Commit Plan

*4 tasks — single commit after completion:*

```
feat(ui): custom form elements — checkbox, select, number inputs
```

---

## Affected Files

| File | Change |
|------|--------|
| `src/lib/components/ui/Checkbox.svelte` | **New** — reusable custom checkbox |
| `src/lib/components/ui/index.ts` | **New** — barrel export |
| `src/lib/components/FilterBar.svelte` | **Modified** — use Checkbox, restyle select + number inputs |
