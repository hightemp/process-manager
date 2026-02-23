<script lang="ts">
  // DEBUG: ConfirmModal icons replaced: TriangleAlert
  import { TriangleAlert } from 'lucide-svelte';
  import type { ProcessDto, KillMode } from '$lib/types';

  interface Props {
    process: ProcessDto | null;
    mode: KillMode;
    onConfirm: (pid: number, mode: KillMode) => void;
    onCancel: () => void;
  }

  let { process, mode, onConfirm, onCancel }: Props = $props();

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') onCancel();
    if (e.key === 'Enter') {
      if (process) onConfirm(process.pid, mode);
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if process}
  <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
    <div class="modal-backdrop" onclick={onCancel} role="presentation">
    <div
      class="modal"
      role="dialog"
      tabindex="-1"
      aria-modal="true"
      aria-labelledby="modal-title"
      onclick={(e) => e.stopPropagation()}
    >
      <h2 id="modal-title" class="modal-title">
        {#if mode === 'kill'}
          <TriangleAlert size={18} class="modal-warn-icon" stroke-width={2} /> Force Kill Process
        {:else}
          Terminate Process
        {/if}
      </h2>

      <div class="process-info">
        <div class="info-row">
          <span class="label">PID:</span>
          <span class="value monospace">{process.pid}</span>
        </div>
        <div class="info-row">
          <span class="label">Name:</span>
          <span class="value">{process.name}</span>
        </div>
        {#if process.user}
          <div class="info-row">
            <span class="label">User:</span>
            <span class="value">{process.user}</span>
          </div>
        {/if}
        {#if process.needs_elevation}
          <div class="elevation-warning">
            <TriangleAlert size={13} class="inline-warn" stroke-width={2} /> This process belongs to another user. You may need elevated privileges.
          </div>
        {/if}
      </div>

      {#if mode === 'kill'}
        <p class="warning-text">
          This will <strong>immediately terminate</strong> the process without giving it
          a chance to save data. This action cannot be undone.
        </p>
      {:else}
        <p class="desc-text">
          A graceful termination signal will be sent to the process. It may take a moment to stop.
        </p>
      {/if}

      <div class="modal-actions">
        <button class="btn-cancel" onclick={onCancel}>Cancel</button>
        <button
          class={mode === 'kill' ? 'btn-danger' : 'btn-warning'}
          onclick={() => onConfirm(process!.pid, mode)}
        >
          {mode === 'kill' ? 'Force Kill' : 'Terminate'}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal {
    background: var(--surface-2);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 24px;
    min-width: 360px;
    max-width: 480px;
    width: 90%;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
  }

  .modal-title {
    margin: 0 0 16px;
    font-size: 1.1rem;
    color: var(--text-primary);
    display: flex;
    align-items: center;
    gap: 8px;
  }

  :global(.modal-warn-icon) {
    color: var(--color-danger);
    flex-shrink: 0;
  }

  :global(.inline-warn) {
    color: var(--color-warning);
    flex-shrink: 0;
  }

  .process-info {
    background: var(--surface-1);
    border-radius: 6px;
    padding: 12px;
    margin-bottom: 12px;
  }

  .info-row {
    display: flex;
    gap: 8px;
    margin-bottom: 4px;
    font-size: 0.875rem;
  }

  .label {
    color: var(--text-muted);
    min-width: 50px;
  }

  .value {
    color: var(--text-primary);
  }

  .monospace {
    font-family: monospace;
  }

  .elevation-warning {
    margin-top: 8px;
    color: var(--color-warning);
    font-size: 0.8rem;
    display: flex;
    align-items: baseline;
    gap: 5px;
  }

  .warning-text {
    color: var(--color-danger);
    font-size: 0.875rem;
    margin: 0 0 16px;
  }

  .desc-text {
    color: var(--text-secondary);
    font-size: 0.875rem;
    margin: 0 0 16px;
  }

  .modal-actions {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
  }

  button {
    padding: 8px 16px;
    border-radius: 4px;
    border: none;
    cursor: pointer;
    font-size: 0.875rem;
    font-weight: 500;
    transition: opacity 0.15s;
  }

  button:hover {
    opacity: 0.85;
  }

  .btn-cancel {
    background: var(--surface-3);
    color: var(--text-secondary);
  }

  .btn-warning {
    background: var(--color-warning);
    color: #000;
  }

  .btn-danger {
    background: var(--color-danger);
    color: #fff;
  }
</style>
