import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type {
  ProcessDto,
  ProcessDetails,
  ProcessFilter,
  SortSpec,
  ProcessUpdateEvent,
} from '$lib/types';

export async function listProcesses(
  filter?: ProcessFilter,
  sort?: SortSpec
): Promise<ProcessDto[]> {
  return invoke<ProcessDto[]>('list_processes', { filter, sort });
}

export async function processDetails(pid: number): Promise<ProcessDetails> {
  return invoke<ProcessDetails>('process_details', { pid });
}

export async function setRefreshInterval(ms: number): Promise<void> {
  return invoke('set_refresh_interval', { ms });
}

export async function setPaused(paused: boolean): Promise<void> {
  return invoke('set_paused', { paused });
}

export async function onProcessesUpdate(
  handler: (event: ProcessUpdateEvent) => void
): Promise<UnlistenFn> {
  return listen<ProcessUpdateEvent>('processes:update', (e) => handler(e.payload));
}
