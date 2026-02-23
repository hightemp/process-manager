<script lang="ts">
  import type { ProcessDto } from '$lib/types';
  import { formatBytes, formatCpu, formatTime, truncatePath } from '$lib/utils/format';
  import { killProcess, openPath, copyToClipboard } from '$lib/api/actions';

  interface Props {
    process: ProcessDto | null;
    onKillRequest: (process: ProcessDto, mode: 'terminate' | 'kill') => void;
    onClose: () => void;
  }

  let { process, onKillRequest, onClose }: Props = $props();

  async function handleCopyPid() {
    if (process) await copyToClipboard(String(process.pid));
  }

  async function handleCopyPath() {
    if (process?.path) await copyToClipboard(process.path);
  }

  async function handleOpenPath() {
    if (process) await openPath(process.pid);
  }
</script>

{#if process}
  <div class="detail-panel">
    <div class="panel-header">
      <div class="title-row">
        <span class="proc-name">{process.name}</span>
        {#if process.needs_elevation}
          <span class="badge-elevation" title="Requires elevated privileges">🔒 Needs rights</span>
        {/if}
      </div>
      <button class="close-btn" onclick={onClose} aria-label="Close detail panel">✕</button>
    </div>

    <div class="panel-body">
      <!-- Metrics -->
      <div class="section">
        <div class="metric-grid">
          <div class="metric">
            <div class="metric-value">{formatCpu(process.cpu_percent)}</div>
            <div class="metric-label">CPU</div>
          </div>
          <div class="metric">
            <div class="metric-value">{formatBytes(process.memory_bytes)}</div>
            <div class="metric-label">RAM</div>
          </div>
          <div class="metric">
            <div class="metric-value status-badge {process.status}">{process.status}</div>
            <div class="metric-label">Status</div>
          </div>
        </div>
      </div>

      <!-- Details -->
      <div class="section">
        <table class="detail-table">
          <tbody>
            <tr>
              <td class="key">PID</td>
              <td class="val monospace">{process.pid}</td>
            </tr>
            {#if process.parent_pid != null}
              <tr>
                <td class="key">Parent PID</td>
                <td class="val monospace">{process.parent_pid}</td>
              </tr>
            {/if}
            {#if process.user}
              <tr>
                <td class="key">User</td>
                <td class="val">{process.user}</td>
              </tr>
            {/if}
            {#if process.path}
              <tr>
                <td class="key">Path</td>
                <td class="val monospace small" title={process.path}>
                  {truncatePath(process.path, 50)}
                </td>
              </tr>
            {/if}
            {#if process.start_time}
              <tr>
                <td class="key">Started</td>
                <td class="val">{formatTime(process.start_time)}</td>
              </tr>
            {/if}
            {#if process.cmd.length > 0}
              <tr>
                <td class="key">Command</td>
                <td class="val monospace small">{process.cmd.slice(0, 5).join(' ')}</td>
              </tr>
            {/if}
          </tbody>
        </table>
      </div>

      <!-- Actions -->
      <div class="section actions-section">
        <div class="action-group">
          <button class="btn-sm btn-copy" onclick={handleCopyPid}>Copy PID</button>
          {#if process.path}
            <button class="btn-sm btn-copy" onclick={handleCopyPath}>Copy Path</button>
            <button class="btn-sm btn-open" onclick={handleOpenPath}>Open Location</button>
          {/if}
        </div>
        <div class="action-group">
          <button
            class="btn-sm btn-terminate"
            onclick={() => onKillRequest(process!, 'terminate')}
            title="Send SIGTERM (graceful)"
          >
            Terminate
          </button>
          <button
            class="btn-sm btn-kill"
            onclick={() => onKillRequest(process!, 'kill')}
            title="Send SIGKILL (force)"
          >
            Force Kill
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .detail-panel {
    width: 280px;
    min-width: 260px;
    background: var(--surface-2);
    border-left: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .panel-header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    padding: 12px;
    border-bottom: 1px solid var(--border);
    gap: 8px;
  }

  .title-row {
    display: flex;
    flex-direction: column;
    gap: 4px;
    min-width: 0;
  }

  .proc-name {
    font-weight: 600;
    font-size: 0.95rem;
    color: var(--text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .badge-elevation {
    font-size: 0.7rem;
    color: var(--color-warning);
    background: rgba(255, 170, 0, 0.12);
    border-radius: 3px;
    padding: 2px 5px;
    width: fit-content;
  }

  .close-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 2px 6px;
    font-size: 0.9rem;
    flex-shrink: 0;
  }

  .panel-body {
    flex: 1;
    overflow-y: auto;
    padding: 8px 0;
  }

  .section {
    padding: 8px 12px;
    border-bottom: 1px solid var(--border);
  }

  .section:last-child {
    border-bottom: none;
  }

  .metric-grid {
    display: flex;
    gap: 12px;
  }

  .metric {
    flex: 1;
    text-align: center;
    background: var(--surface-1);
    border-radius: 6px;
    padding: 8px 4px;
  }

  .metric-value {
    font-size: 1rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .metric-label {
    font-size: 0.7rem;
    color: var(--text-muted);
    margin-top: 2px;
  }

  .status-badge {
    font-size: 0.75rem;
    padding: 2px 6px;
    border-radius: 4px;
  }

  .status-badge.running { background: rgba(60, 200, 100, 0.2); color: #3cc864; }
  .status-badge.sleeping { background: rgba(100, 150, 255, 0.2); color: #6496ff; }
  .status-badge.stopped { background: rgba(255, 150, 0, 0.2); color: #ffa030; }
  .status-badge.zombie { background: rgba(255, 80, 80, 0.2); color: #ff5050; }
  .status-badge.unknown { background: rgba(150, 150, 150, 0.2); color: #aaa; }

  .detail-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.8rem;
  }

  .detail-table tr:not(:last-child) td {
    padding-bottom: 4px;
  }

  .key {
    color: var(--text-muted);
    padding-right: 8px;
    white-space: nowrap;
    vertical-align: top;
    width: 70px;
  }

  .val {
    color: var(--text-secondary);
    word-break: break-all;
  }

  .monospace {
    font-family: 'Menlo', 'Consolas', monospace;
    font-size: 0.78rem;
  }

  .small {
    font-size: 0.72rem;
  }

  .actions-section {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .action-group {
    display: flex;
    gap: 6px;
    flex-wrap: wrap;
  }

  .btn-sm {
    padding: 5px 10px;
    border-radius: 4px;
    border: 1px solid var(--border);
    font-size: 0.78rem;
    cursor: pointer;
    transition: all 0.15s;
  }

  .btn-copy {
    background: var(--surface-1);
    color: var(--text-secondary);
  }

  .btn-open {
    background: var(--surface-1);
    color: var(--color-accent);
  }

  .btn-terminate {
    background: rgba(255, 170, 0, 0.15);
    border-color: rgba(255, 170, 0, 0.4);
    color: var(--color-warning);
  }

  .btn-terminate:hover {
    background: rgba(255, 170, 0, 0.25);
  }

  .btn-kill {
    background: rgba(255, 60, 60, 0.15);
    border-color: rgba(255, 60, 60, 0.4);
    color: var(--color-danger);
  }

  .btn-kill:hover {
    background: rgba(255, 60, 60, 0.25);
  }
</style>
