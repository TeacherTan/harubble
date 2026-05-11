<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { localeState } from '$lib/i18n';
  import { Input } from '$lib/components/ui/input/index.js';
  import { Home, Library, Tag, Search } from '@lucide/svelte';
  import type { AppView } from '$lib/features/shell/store.svelte';

  interface Props {
    currentView: AppView;
    searchQuery: string;
    onNavigate: (view: AppView) => void;
    onSearchQueryChange: (query: string) => void;
  }

  let { currentView, searchQuery, onNavigate, onSearchQueryChange }: Props =
    $props();

  const labels = $derived.by(() => {
    void localeState.current;
    return {
      home: m.shell_nav_home(),
      library: m.shell_nav_library(),
      tags: m.shell_nav_tags(),
      searchPlaceholder: m.sidebar_search_placeholder(),
    };
  });

  const navItems: {
    view: AppView;
    icon: typeof Home;
    labelKey: 'home' | 'library' | 'tags';
  }[] = [
    { view: 'home', icon: Home, labelKey: 'home' },
    { view: 'library', icon: Library, labelKey: 'library' },
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

  <div class="nav-search">
    <Search size={14} class="nav-search-icon" aria-hidden="true" />
    <Input
      value={searchQuery}
      placeholder={labels.searchPlaceholder}
      class="nav-search-input"
      aria-label={labels.searchPlaceholder}
      oninput={(event) => {
        const target = event.currentTarget as HTMLInputElement;
        onSearchQueryChange(target.value);
      }}
      onfocus={() => {
        if (currentView !== 'library') onNavigate('library');
      }}
    />
  </div>
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

  .nav-search {
    position: relative;
    margin-top: 8px;
    padding: 0 4px;
  }

  .nav-search :global(.nav-search-icon) {
    position: absolute;
    top: 50%;
    left: 14px;
    z-index: 1;
    color: var(--text-tertiary);
    pointer-events: none;
    transform: translateY(-50%);
  }

  .nav-search :global(.nav-search-input) {
    height: 32px;
    padding-left: 30px;
    border-radius: 8px;
    font-size: 0.75rem;
    border: 1px solid var(--border);
    background: var(--hover-bg);
  }

  .nav-search :global(.nav-search-input:focus-visible) {
    border-color: rgba(var(--accent-rgb), 0.36);
    box-shadow: 0 0 0 3px rgba(var(--accent-rgb), 0.1);
  }
</style>
