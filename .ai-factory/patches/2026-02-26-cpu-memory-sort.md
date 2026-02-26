# CPU/Memory column sorting broken due to column-ID ↔ SortField mismatch

**Date:** 2026-02-26
**Files:** src/lib/components/ProcessTable.svelte
**Severity:** medium

## Problem

Clicking the CPU% or RAM column headers had no effect — the process list did
not sort. The sort indicator (chevron) also never appeared on those columns.

## Root Cause

Column definitions in `settingsStore` use short IDs (`'cpu'`, `'memory'`),
while `SortField` values are `'cpu_percent'` and `'memory_bytes'`.

`ProcessTable.svelte` was casting `col.id as SortField` and passing it
directly to `filterStore.toggleSort()`. The cast silenced the TypeScript
compiler, but at runtime the switch statement in `+page.svelte` received
`'cpu'` / `'memory'` which matched no case and fell through to
`default: return 0` — effectively no-op sorting.

The same mismatch in `isSorted()` prevented the sort indicator from showing.

## Solution

Added an explicit `COL_TO_SORT_FIELD` mapping in `ProcessTable.svelte`:

```ts
const COL_TO_SORT_FIELD: Record<string, SortField> = {
  pid: 'pid', name: 'name', user: 'user',
  cpu: 'cpu_percent',
  memory: 'memory_bytes',
  status: 'status',
};
```

`handleSort` and `isSorted` now go through `colSortField(colId)` instead of
casting. Template uses `col.id` (plain string) everywhere.

## Prevention

- Avoid `as Type` casts to bridge incompatible string unions — add a proper
  mapping instead.
- Add a TypeScript type-check or unit test that verifies every visible column
  can resolve to a valid `SortField` (or `null` for non-sortable columns).

## Tags

`#sort` `#column-mapping` `#svelte` `#typescript` `#cast-bypass`
