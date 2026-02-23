// Shared TypeScript types mirroring Rust DTOs

export type ProcessStatus = 'running' | 'sleeping' | 'stopped' | 'zombie' | 'unknown';

export interface ProcessDto {
  pid: number;
  name: string;
  status: ProcessStatus;
  cpu_percent: number;
  memory_bytes: number;
  user: string | null;
  path: string | null;
  parent_pid: number | null;
  start_time: number | null;
  needs_elevation: boolean;
  cmd: string[];
}

export interface ProcessDetails {
  dto: ProcessDto;
  threads: number | null;
  virtual_memory_bytes: number | null;
  disk_read_bytes: number | null;
  disk_written_bytes: number | null;
  open_files_count: number | null;
  environment: string[] | null;
}

export type SortField = 'pid' | 'name' | 'cpu_percent' | 'memory_bytes' | 'user' | 'status' | 'start_time';
export type SortDirection = 'asc' | 'desc';

export interface SortSpec {
  field: SortField;
  direction: SortDirection;
}

export interface ProcessFilter {
  search?: string;
  user?: string;
  mine_only?: boolean;
  system_only?: boolean;
  non_system_only?: boolean;
  status?: ProcessStatus;
  cpu_gt?: number;
  memory_gt_bytes?: number;
}

export type KillMode = 'terminate' | 'kill';

export interface ProcessUpdateEvent {
  added: ProcessDto[];
  updated: ProcessDto[];
  removed: number[];
  timestamp_ms: number;
}

export type AppError =
  | { type: 'NotFound'; data: { pid: number } }
  | { type: 'PermissionDenied'; data: { pid: number; message: string } }
  | { type: 'InvalidPid'; data: { pid: number } }
  | { type: 'OsError'; data: { message: string } }
  | { type: 'Unsupported'; data: { feature: string } };
