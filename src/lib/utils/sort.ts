import type { SortField } from '$lib/types';

/**
 * Maps column IDs (used in ColumnDef) to their corresponding SortField values.
 * Column IDs are short display names; SortField values match ProcessDto property names.
 */
export const COL_TO_SORT_FIELD: Record<string, SortField> = {
  pid:    'pid',
  name:   'name',
  user:   'user',
  cpu:    'cpu_percent',
  memory: 'memory_bytes',
  status: 'status',
};

/**
 * Returns the SortField for a column ID, or null if the column is not sortable.
 */
export function colSortField(colId: string): SortField | null {
  return COL_TO_SORT_FIELD[colId] ?? null;
}
