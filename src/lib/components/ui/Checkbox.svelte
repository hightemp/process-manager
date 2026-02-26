<script lang="ts">
  interface Props {
    checked: boolean;
    disabled?: boolean;
  }

  let { checked = $bindable(), disabled = false }: Props = $props();
</script>

<span class="checkbox-wrap" class:disabled>
  <input
    type="checkbox"
    class="checkbox-input"
    bind:checked
    {disabled}
  />
  <span class="checkbox-box" class:checked aria-hidden="true"></span>
</span>

<style>
  .checkbox-wrap {
    position: relative;
    display: inline-flex;
    align-items: center;
    flex-shrink: 0;
    width: 14px;
    height: 14px;
    cursor: pointer;
  }

  .checkbox-wrap.disabled {
    opacity: 0.45;
    cursor: not-allowed;
  }

  /* Visually hidden native input — still accessible via keyboard / AT */
  .checkbox-input {
    position: absolute;
    opacity: 0;
    width: 0;
    height: 0;
    margin: 0;
    pointer-events: none;
  }

  /* Styled box */
  .checkbox-box {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 14px;
    height: 14px;
    border: 1px solid var(--border);
    border-radius: 3px;
    background: var(--surface-1);
    transition: background 0.12s, border-color 0.12s;
    flex-shrink: 0;
  }

  /* Checkmark via ::after */
  .checkbox-box::after {
    content: '';
    display: block;
    width: 8px;
    height: 5px;
    border-left: 1.5px solid #fff;
    border-bottom: 1.5px solid #fff;
    transform: rotate(-45deg) translateY(-1px);
    opacity: 0;
    transition: opacity 0.1s;
  }

  /* Checked state */
  .checkbox-box.checked {
    background: var(--color-accent);
    border-color: var(--color-accent);
  }

  .checkbox-box.checked::after {
    opacity: 1;
  }

  /* Hover on the outer wrap (when not disabled) */
  .checkbox-wrap:not(.disabled):hover .checkbox-box:not(.checked) {
    border-color: var(--color-accent);
  }

  /* Focus-visible — shown when navigated via keyboard */
  .checkbox-input:focus-visible ~ .checkbox-box {
    outline: 2px solid var(--color-accent);
    outline-offset: 2px;
  }
</style>
