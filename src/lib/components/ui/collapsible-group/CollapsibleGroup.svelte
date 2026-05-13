<script lang="ts">
  import { slide } from 'svelte/transition';
  import ChevronRightIcon from '@lucide/svelte/icons/chevron-right';
  import type { Snippet } from 'svelte';

  interface Props {
    title: string;
    defaultExpanded?: boolean;
    children: Snippet;
    actions?: Snippet;
  }

  let { title, defaultExpanded = false, children, actions }: Props = $props();

  let expanded = $state(defaultExpanded);

  function toggle() {
    expanded = !expanded;
  }

  function handleActionsClick(e: MouseEvent) {
    e.stopPropagation();
  }
</script>

<div class="collapsible-group">
  <button
    type="button"
    class="collapsible-group-header"
    aria-expanded={expanded}
    onclick={toggle}
  >
    <span class="collapsible-group-chevron" class:is-expanded={expanded}>
      <ChevronRightIcon size={12} />
    </span>
    <span class="collapsible-group-title">{title}</span>
    {#if actions}
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <span class="collapsible-group-actions" onclick={handleActionsClick}>
        {@render actions()}
      </span>
    {/if}
  </button>

  {#if expanded}
    <div class="collapsible-group-content" transition:slide={{ duration: 200 }}>
      {@render children()}
    </div>
  {/if}
</div>

<style>
  .collapsible-group {
    display: flex;
    flex-direction: column;
  }

  .collapsible-group-header {
    appearance: none;
    border: none;
    background: none;
    display: flex;
    align-items: center;
    gap: 6px;
    width: 100%;
    padding: 6px 8px;
    border-radius: 6px;
    cursor: pointer;
    transition: background-color 0.15s ease;
  }

  .collapsible-group-header:hover {
    background: rgba(255, 255, 255, 0.06);
  }

  .collapsible-group-chevron {
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-tertiary);
    transition: transform 0.2s ease;
    flex-shrink: 0;
  }

  .collapsible-group-chevron.is-expanded {
    transform: rotate(90deg);
  }

  .collapsible-group-title {
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--text-tertiary);
  }

  .collapsible-group-actions {
    margin-left: auto;
    display: flex;
    align-items: center;
    gap: 2px;
    opacity: 0;
    transition: opacity 0.15s ease;
  }

  .collapsible-group-header:hover .collapsible-group-actions {
    opacity: 1;
  }

  .collapsible-group-content {
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding-top: 2px;
  }
</style>
