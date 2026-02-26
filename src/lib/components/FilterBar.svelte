<script lang="ts">
  // DEBUG: FilterBar icons replaced with lucide-svelte: Search, X, Play, Pause
  import { Search, X, Play, Pause } from 'lucide-svelte';
  import { filterStore } from '$lib/stores/filterStore.svelte';
  import { settingsStore } from '$lib/stores/settingsStore.svelte';
  import { setPaused, setRefreshInterval } from '$lib/api/processes';
  import { Checkbox } from '$lib/components/ui';

  interface Props {
    totalCount: number;
    visibleCount: number;
    searchInputEl?: HTMLInputElement;
  }

  let { totalCount, visibleCount, searchInputEl = $bindable() }: Props = $props();

  async function togglePause() {
    settingsStore.paused = !settingsStore.paused;
    await setPaused(settingsStore.paused);
  }

  async function changeInterval(e: Event) {
    const val = parseInt((e.target as HTMLSelectElement).value);
    settingsStore.refreshIntervalMs = val;
    await setRefreshInterval(val);
  }
</script>

<div class="filter-bar">
  <!-- Search -->
  <div class="search-wrap">
    <Search size={16} class="search-icon" stroke-width={1.75} />
    <input
      bind:this={searchInputEl}
      class="search-input"
      type="text"
      placeholder="Search name, PID, path... (/)"
      bind:value={filterStore.search}
      autocomplete="off"
      spellcheck={false}
    />
    {#if filterStore.search}
      <button class="clear-btn" onclick={() => { filterStore.search = ''; }} aria-label="Clear search"><X size={13} stroke-width={2} /></button>
    {/if}
  </div>

  <!-- Filter toggles -->
  <div class="toggles">
    <label class="toggle" title="Show only my processes">
      <Checkbox bind:checked={filterStore.mineOnly} />
      Mine only
    </label>
    <label class="toggle" title="Show only system processes (PID < 500 or root user)">
      <Checkbox bind:checked={filterStore.systemOnly} />
      System
    </label>
    <label class="toggle" title="Hide system processes">
      <Checkbox bind:checked={filterStore.nonSystemOnly} />
      Non-system
    </label>
  </div>

  <!-- Threshold filters -->
  <div class="thresholds">
    <label class="threshold-label">
      CPU &gt;
      <input
        class="threshold-input"
        type="number"
        min="0"
        max="100"
        step="5"
        placeholder="0"
        value={filterStore.cpuGt ?? ''}
        oninput={(e) => {
          const v = parseFloat((e.target as HTMLInputElement).value);
          filterStore.cpuGt = isNaN(v) ? undefined : v;
        }}
      />%
    </label>
    <label class="threshold-label">
      RAM &gt;
      <input
        class="threshold-input"
        type="number"
        min="0"
        step="10"
        placeholder="0"
        value={filterStore.memGtMb ?? ''}
        oninput={(e) => {
          const v = parseFloat((e.target as HTMLInputElement).value);
          filterStore.memGtMb = isNaN(v) ? undefined : v;
        }}
      />MB
    </label>
  </div>

  <!-- Spacer -->
  <div class="spacer"></div>

  <!-- Process counts -->
  <div class="counts">
    <span class="count-text">{visibleCount} / {totalCount} processes</span>
  </div>

  <!-- Refresh controls -->
  <div class="refresh-controls">
    <select class="interval-select" onchange={changeInterval} value={settingsStore.refreshIntervalMs}>
      <option value={500}>0.5s</option>
      <option value={1000}>1s</option>
      <option value={2000}>2s</option>
      <option value={5000}>5s</option>
    </select>
    <button
      class={`pause-btn ${settingsStore.paused ? 'paused' : ''}`}
      onclick={togglePause}
      title={settingsStore.paused ? 'Resume auto-refresh (F5)' : 'Pause auto-refresh'}
    >
      {#if settingsStore.paused}
        <Play size={13} stroke-width={2} /> Resume
      {:else}
        <Pause size={13} stroke-width={2} /> Pause
      {/if}
    </button>
  </div>
</div>

<style>
  .filter-bar {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px 12px;
    background: var(--surface-2);
    border-bottom: 1px solid var(--border);
    flex-wrap: wrap;
  }

  .search-wrap {
    position: relative;
    display: flex;
    align-items: center;
    flex: 0 0 260px;
  }

  :global(.search-icon) {
    position: absolute;
    left: 8px;
    pointer-events: none;
    opacity: 0.55;
    color: var(--text-muted);
    display: flex;
  }

  .search-input {
    width: 100%;
    padding: 6px 28px 6px 28px;
    background: var(--surface-1);
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-primary);
    font-size: 0.85rem;
    outline: none;
    transition: border-color 0.15s;
  }

  .search-input:focus {
    border-color: var(--color-accent);
  }

  .clear-btn {
    position: absolute;
    right: 6px;
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 2px 4px;
    display: flex;
    align-items: center;
  }

  .toggles {
    display: flex;
    gap: 8px;
  }

  .toggle {
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 0.8rem;
    color: var(--text-secondary);
    cursor: pointer;
    user-select: none;
    white-space: nowrap;
  }

  .thresholds {
    display: flex;
    gap: 8px;
  }

  .threshold-label {
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 0.8rem;
    color: var(--text-secondary);
    white-space: nowrap;
  }

  .threshold-input {
    width: 52px;
    padding: 3px 5px;
    background: var(--surface-1);
    border: 1px solid var(--border);
    border-radius: 3px;
    color: var(--text-primary);
    font-size: 0.8rem;
    text-align: right;
    outline: none;
    transition: border-color 0.15s;
    /* Remove native spinner arrows */
    -moz-appearance: textfield;
    appearance: textfield;
  }

  .threshold-input::-webkit-inner-spin-button,
  .threshold-input::-webkit-outer-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }

  .threshold-input:hover {
    border-color: var(--text-muted);
  }

  .threshold-input:focus {
    border-color: var(--color-accent);
  }

  .spacer {
    flex: 1;
  }

  .counts {
    font-size: 0.78rem;
    color: var(--text-muted);
    white-space: nowrap;
  }

  .refresh-controls {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .interval-select {
    appearance: none;
    -webkit-appearance: none;
    padding: 4px 22px 4px 8px;
    background-color: var(--surface-1);
    /* Dark theme chevron: --text-muted #636d76 */
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%23636d76' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpolyline points='6 9 12 15 18 9'/%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 6px center;
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-primary);
    font-size: 0.8rem;
    cursor: pointer;
    outline: none;
    transition: border-color 0.15s;
  }

  .interval-select:hover {
    border-color: var(--color-accent);
  }

  .interval-select:focus {
    border-color: var(--color-accent);
  }

  /* Light theme: use --text-muted #9198a1 for chevron */
  :global(:root[data-theme="light"]) .interval-select {
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%239198a1' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpolyline points='6 9 12 15 18 9'/%3E%3C/svg%3E");
  }

  .pause-btn {
    padding: 4px 10px;
    border-radius: 4px;
    border: 1px solid var(--border);
    background: var(--surface-1);
    color: var(--text-secondary);
    font-size: 0.8rem;
    cursor: pointer;
    white-space: nowrap;
    transition: all 0.15s;
    display: flex;
    align-items: center;
    gap: 5px;
  }

  .pause-btn:hover {
    border-color: var(--color-accent);
    color: var(--text-primary);
  }

  .pause-btn.paused {
    background: var(--color-accent);
    color: #fff;
    border-color: var(--color-accent);
  }
</style>
