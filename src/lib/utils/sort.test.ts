import { describe, it, expect } from 'vitest';
import { COL_TO_SORT_FIELD, colSortField } from './sort';

describe('COL_TO_SORT_FIELD', () => {
  it('maps cpu column to cpu_percent SortField', () => {
    expect(COL_TO_SORT_FIELD['cpu']).toBe('cpu_percent');
  });

  it('maps memory column to memory_bytes SortField', () => {
    expect(COL_TO_SORT_FIELD['memory']).toBe('memory_bytes');
  });

  it('maps other columns to their own names', () => {
    expect(COL_TO_SORT_FIELD['pid']).toBe('pid');
    expect(COL_TO_SORT_FIELD['name']).toBe('name');
    expect(COL_TO_SORT_FIELD['user']).toBe('user');
    expect(COL_TO_SORT_FIELD['status']).toBe('status');
  });
});

describe('colSortField', () => {
  it('returns cpu_percent for cpu column', () => {
    expect(colSortField('cpu')).toBe('cpu_percent');
  });

  it('returns memory_bytes for memory column', () => {
    expect(colSortField('memory')).toBe('memory_bytes');
  });

  it('returns null for non-sortable columns (path, parent, actions)', () => {
    expect(colSortField('path')).toBeNull();
    expect(colSortField('parent')).toBeNull();
    expect(colSortField('actions')).toBeNull();
    expect(colSortField('')).toBeNull();
  });

  it('returns correct field for all sortable columns', () => {
    const sortable: Array<[string, string]> = [
      ['pid',    'pid'],
      ['name',   'name'],
      ['user',   'user'],
      ['cpu',    'cpu_percent'],
      ['memory', 'memory_bytes'],
      ['status', 'status'],
    ];
    for (const [colId, expected] of sortable) {
      expect(colSortField(colId)).toBe(expected);
    }
  });
});
