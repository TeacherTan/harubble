<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { localeState } from '$lib/i18n';
  import { Home, Library, Tag } from '@lucide/svelte';
  import type { AppView } from '$lib/features/shell/store.svelte';

  interface Props {
    currentView: AppView;
    onNavigate: (view: AppView) => void;
  }

  let { currentView, onNavigate }: Props = $props();

  const labels = $derived.by(() => {
    void localeState.current;
    return {
      home: m.shell_nav_home(),
      library: m.shell_nav_library(),
      tags: m.shell_nav_tags(),
    };
  });

  const navItems: {
    view: AppView;
    icon: typeof Home;
    labelKey: 'home' | 'library' | 'tags';
  }[] = [
    { view: 'home', icon: Home, labelKey: 'home' },
    { view: 'overview', icon: Library, labelKey: 'library' },
    { view: 'tagEditor', icon: Tag, labelKey: 'tags' },
  ];
</script>

<nav class="sidebar-nav" aria-label="Main navigation">
  {#each navItems as item (item.view)}
    <button
      type="button"
      class="nav-item"
      class:active={currentView === item.view}
      onclick={() => onNavigate(item.view)}
      aria-current={currentView === item.view ? 'page' : undefined}
    >
      <item.icon size={16} aria-hidden="true" />
      <span>{labels[item.labelKey]}</span>
    </button>
  {/each}
</nav>

<style>
  .sidebar-nav {
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: 0 8px;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    width: 100%;
    height: 36px;
    padding: 0 0.75rem;
    border: none;
    border-radius: 8px;
    background: none;
    color: var(--text-secondary, rgba(255, 255, 255, 0.6));
    font-family: var(--font-body);
    font-size: 0.8125rem;
    font-weight: 500;
    cursor: pointer;
    transition:
      background var(--motion-fast) ease,
      color var(--motion-fast) ease;
  }

  .nav-item:hover {
    background: var(--hover-bg-elevated);
    color: var(--text-primary);
  }

  .nav-item.active {
    background: var(--surface-state);
    color: var(--text-primary);
    font-weight: 600;
  }
</style>
