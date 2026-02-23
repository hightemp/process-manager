<script lang="ts">
  // DEBUG: Toast dismiss icon replaced: X
  import { X } from 'lucide-svelte';

  export interface Toast {
    id: number;
    message: string;
    type: 'error' | 'success' | 'info';
  }

  interface Props {
    toasts: Toast[];
    onDismiss: (id: number) => void;
  }

  let { toasts, onDismiss }: Props = $props();
</script>

<div class="toast-container" aria-live="polite">
  {#each toasts as toast (toast.id)}
    <div class="toast toast-{toast.type}" role="alert">
      <span class="toast-msg">{toast.message}</span>
      <button class="toast-close" onclick={() => onDismiss(toast.id)} aria-label="Dismiss"><X size={14} stroke-width={2} /></button>
    </div>
  {/each}
</div>

<style>
  .toast-container {
    position: fixed;
    bottom: 16px;
    right: 16px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    z-index: 2000;
    max-width: 360px;
  }

  .toast {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    padding: 10px 14px;
    border-radius: 6px;
    font-size: 0.85rem;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
    animation: slide-in 0.2s ease-out;
  }

  @keyframes slide-in {
    from { transform: translateX(100%); opacity: 0; }
    to   { transform: translateX(0);   opacity: 1; }
  }

  .toast-error   { background: #3a1515; border: 1px solid #c04040; color: #ff8080; }
  .toast-success { background: #153a20; border: 1px solid #3c9840; color: #70d880; }
  .toast-info    { background: #1a2240; border: 1px solid #3060c0; color: #80a8f0; }

  .toast-msg { flex: 1; }

  .toast-close {
    background: none;
    border: none;
    cursor: pointer;
    color: inherit;
    opacity: 0.6;
    padding: 0 2px;
    display: flex;
    align-items: center;
  }

  .toast-close:hover { opacity: 1; }
</style>
