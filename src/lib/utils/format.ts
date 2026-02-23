/** Format bytes to human-readable string. */
export function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(1))} ${sizes[i]}`;
}

/** Format CPU percentage with one decimal. */
export function formatCpu(cpu: number): string {
  return `${cpu.toFixed(1)}%`;
}

/** Format Unix timestamp to locale date-time string. */
export function formatTime(ts: number | null): string {
  if (!ts) return '–';
  return new Date(ts * 1000).toLocaleTimeString();
}

/** Truncate a long path for display, keeping filename. */
export function truncatePath(path: string | null, maxLen = 40): string {
  if (!path) return '–';
  if (path.length <= maxLen) return path;
  const parts = path.replace(/\\/g, '/').split('/');
  const filename = parts[parts.length - 1];
  const prefix = path.slice(0, maxLen - filename.length - 4);
  return `${prefix}…/${filename}`;
}
