<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';

  // ── State ─────────────────────────────────────────────────────────────────
  let isMaximized = $state(false);
  let unlistenResize: (() => void) | undefined;

  const win = getCurrentWindow();

  // ── Lifecycle ─────────────────────────────────────────────────────────────
  onMount(async () => {
    isMaximized = await win.isMaximized();
    console.debug('[titlebar] mounted, isMaximized:', isMaximized);

    unlistenResize = await win.onResized(async () => {
      isMaximized = await win.isMaximized();
      console.debug('[titlebar] resize event → isMaximized:', isMaximized);
    });
  });

  onDestroy(() => {
    unlistenResize?.();
    console.debug('[titlebar] destroyed, unsubscribed from resize events');
  });

  // ── Button handlers ───────────────────────────────────────────────────────
  function handleMinimize() {
    console.debug('[titlebar] minimize clicked');
    win.minimize();
  }

  function handleToggleMaximize() {
    console.debug('[titlebar] toggleMaximize clicked, was:', isMaximized);
    win.toggleMaximize();
  }

  function handleClose() {
    console.debug('[titlebar] close clicked');
    win.close();
  }
</script>

<!-- Titlebar root — data-tauri-drag-region makes the whole bar draggable;
     interactive elements (buttons) are placed OUTSIDE the drag div -->
<div class="titlebar">
  <!-- Drag region: icon + title text -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="titlebar-drag-region"
    data-tauri-drag-region
    ondblclick={handleToggleMaximize}
    role="presentation"
  >
    <svg class="app-icon" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
      <rect x="1" y="3" width="14" height="10" rx="1.5" stroke="currentColor" stroke-width="1.5"/>
      <path d="M5 7h6M5 9.5h4" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>
    </svg>
    <span class="titlebar-title">Process Manager</span>
  </div>

  <!-- Window control buttons — NOT inside drag region -->
  <div class="titlebar-controls">
    <!-- Minimize -->
    <button
      class="titlebar-btn"
      onclick={handleMinimize}
      aria-label="Minimize"
      title="Minimize"
    >
      <!-- Horizontal line centered -->
      <svg width="10" height="1" viewBox="0 0 10 1" fill="none" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
        <line x1="0" y1="0.5" x2="10" y2="0.5" stroke="currentColor" stroke-width="1"/>
      </svg>
    </button>

    <!-- Maximize / Restore -->
    <button
      class="titlebar-btn"
      onclick={handleToggleMaximize}
      aria-label={isMaximized ? 'Restore' : 'Maximize'}
      title={isMaximized ? 'Restore' : 'Maximize'}
    >
      {#if isMaximized}
        <!-- Restore icon: two overlapping squares -->
        <svg width="10" height="10" viewBox="0 0 10 10" fill="none" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
          <rect x="2.5" y="0.5" width="7" height="7" rx="0.5" stroke="currentColor" stroke-width="1"/>
          <path d="M0.5 2.5v7a.5.5 0 0 0 .5.5h7" stroke="currentColor" stroke-width="1"/>
        </svg>
      {:else}
        <!-- Maximize icon: hollow square -->
        <svg width="10" height="10" viewBox="0 0 10 10" fill="none" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
          <rect x="0.5" y="0.5" width="9" height="9" rx="0.5" stroke="currentColor" stroke-width="1"/>
        </svg>
      {/if}
    </button>

    <!-- Close -->
    <button
      class="titlebar-btn titlebar-btn--close"
      onclick={handleClose}
      aria-label="Close"
      title="Close"
    >
      <!-- × cross -->
      <svg width="10" height="10" viewBox="0 0 10 10" fill="none" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
        <line x1="1" y1="1" x2="9" y2="9" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>
        <line x1="9" y1="1" x2="1" y2="9" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>
      </svg>
    </button>
  </div>
</div>

<style>
  .titlebar {
    display: flex;
    align-items: center;
    height: 32px;
    flex: 0 0 32px;
    background: var(--surface-2);
    border-bottom: 1px solid var(--border);
    user-select: none;
    /* Prevent titlebar from being hidden by the content layer */
    position: relative;
    z-index: 100;
  }

  /* Drag region fills all available space left of the buttons */
  .titlebar-drag-region {
    display: flex;
    align-items: center;
    flex: 1;
    height: 100%;
    padding: 0 10px;
    gap: 7px;
    overflow: hidden;
    cursor: default;
    /* IMPORTANT: pointer-events must be auto so Tauri can intercept drags */
  }

  .app-icon {
    width: 14px;
    height: 14px;
    flex-shrink: 0;
    color: var(--color-accent);
    opacity: 0.85;
  }

  .titlebar-title {
    font-size: 12px;
    font-weight: 400;
    color: var(--text-secondary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  /* Windows-style control button group */
  .titlebar-controls {
    display: flex;
    align-items: stretch;
    height: 100%;
    flex-shrink: 0;
  }

  .titlebar-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 46px;
    height: 32px;
    padding: 0;
    margin: 0;
    border: none;
    background: transparent;
    color: var(--text-secondary);
    cursor: default;
    transition: background 0.1s ease, color 0.1s ease;
    outline: none;
  }

  .titlebar-btn:hover {
    background: var(--surface-hover);
    color: var(--text-primary);
  }

  .titlebar-btn:active {
    background: var(--surface-3);
  }

  /* Close button: red background on hover (Windows standard) */
  .titlebar-btn--close:hover {
    background: #c42b1c;
    color: #ffffff;
  }

  .titlebar-btn--close:active {
    background: #b01a0f;
    color: #ffffff;
  }

  /* SVG icons inherit text color */
  .titlebar-btn svg {
    display: block;
    pointer-events: none;
  }
</style>
