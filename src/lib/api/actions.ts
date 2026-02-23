import { invoke } from '@tauri-apps/api/core';
import type { AppError, KillMode } from '$lib/types';

export async function killProcess(pid: number, mode: KillMode): Promise<void> {
  return invoke('kill_process', { pid, mode });
}

export async function openPath(pid: number): Promise<void> {
  return invoke('open_path', { pid });
}

export async function copyToClipboard(text: string): Promise<void> {
  return invoke('copy_to_clipboard', { text });
}
