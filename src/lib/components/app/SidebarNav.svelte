<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { localeState } from '$lib/i18n';
  import { Input } from '$lib/components/ui/input/index.js';
  import { Button } from '$lib/components/ui/button/index.js';
  import { Home, Library, Tag, Search } from '@lucide/svelte';
  import type { AppView } from '$lib/features/shell/store.svelte';
  import type { LibrarySearchScope } from '$lib/types';

  interface Props {
    currentView: AppView;
    searchQuery: string;
    searchScope: LibrarySearchScope;
    onNavigate: (view: AppView) => void;
    onSearchQueryChange: (query: string) => void;
    onSearchScopeChange: (scope: LibrarySearchScope) => void;
  }

  let {
    currentView,
    searchQuery,
    searchScope,
    onNavigate,
    onSearchQueryChange,
    onSearchScopeChange,
  }: Props = $props();

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

  const scopeOptions = $derived.by(() => {
    void localeState.current;
    return [
      {
        value: 'all' as LibrarySearchScope,
        // "ALL" 是固定品牌文案，不走 i18n
        label: 'ALL',
      },
      {
        value: 'albums' as LibrarySearchScope,
        label: m.library_search_scope_albums(),
      },
      {
        value: 'songs' as LibrarySearchScope,
        label: m.library_search_scope_songs(),
      },
    ];
  });

  const activeScopeLabel = $derived.by(
    () =>
      scopeOptions.find((option) => option.value === searchScope)?.label ??
      'ALL'
  );

  function cycleSearchScope() {
    const currentIndex = scopeOptions.findIndex(
      (option) => option.value === searchScope
    );
    const nextIndex = (currentIndex + 1) % scopeOptions.length;
    onSearchScopeChange(scopeOptions[nextIndex]?.value ?? 'all');
  }
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
    <div class="nav-search-field">
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
      <Button
        variant="outline"
        size="icon"
        class="nav-search-scope-button active:!translate-y-0"
        data-scope={searchScope}
        aria-label={m.library_search_scope_aria({ scope: activeScopeLabel })}
        title={m.library_search_scope_title({ scope: activeScopeLabel })}
        onclick={cycleSearchScope}
      >
        {activeScopeLabel}
      </Button>
    </div>
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
    margin-top: 8px;
    padding: 0 4px;
  }

  .nav-search-field {
    position: relative;
  }

  .nav-search-field :global(.nav-search-icon) {
    position: absolute;
    top: 50%;
    left: 10px;
    z-index: 1;
    color: var(--text-tertiary);
    pointer-events: none;
    transform: translateY(-50%);
  }

  .nav-search-field :global(.nav-search-input) {
    height: 32px;
    padding-left: 30px;
    padding-right: 40px;
    border-radius: 8px;
    font-size: 0.75rem;
    border: 1px solid var(--border);
    background: var(--hover-bg);
  }

  .nav-search-field :global(.nav-search-input:focus-visible) {
    border-color: rgba(var(--accent-rgb), 0.36);
    box-shadow: 0 0 0 3px rgba(var(--accent-rgb), 0.1);
  }

  .nav-search-field :global(.nav-search-scope-button) {
    --scope-bg: var(--accent);
    --scope-bg-hover: var(--accent-hover);
    interpolate-size: allow-keywords;
    position: absolute;
    top: 3px;
    right: 3px;
    width: auto;
    min-width: 26px;
    height: 26px;
    padding: 0 6px;
    border: 1px solid color-mix(in srgb, var(--scope-bg) 72%, white 28%);
    border-radius: 6px;
    background: var(--scope-bg);
    box-shadow:
      inset 0 1px 0 rgba(255, 255, 255, 0.22),
      0 4px 10px color-mix(in srgb, var(--scope-bg) 24%, transparent);
    color: var(--accent-readable-foreground);
    font-family: var(--font-wide);
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 0.02em;
    line-height: 1;
    white-space: nowrap;
    transition:
      width 0.25s ease,
      transform 0.15s ease,
      background-color 0.2s ease,
      border-color 0.2s ease,
      box-shadow 0.2s ease;
  }

  .nav-search-field :global(.nav-search-scope-button[data-scope='albums']) {
    --scope-bg: oklch(from var(--accent) l c calc(h + 22));
    --scope-bg-hover: oklch(from var(--accent-hover) l c calc(h + 22));
  }

  .nav-search-field :global(.nav-search-scope-button[data-scope='songs']) {
    --scope-bg: oklch(from var(--accent) l c calc(h - 28));
    --scope-bg-hover: oklch(from var(--accent-hover) l c calc(h - 28));
  }

  .nav-search-field :global(.nav-search-scope-button:hover) {
    border-color: color-mix(in srgb, var(--scope-bg-hover) 78%, white 22%);
    background: var(--scope-bg-hover);
    color: var(--accent-hover-readable-foreground);
  }

  .nav-search-field :global(.nav-search-scope-button[data-scope='all']) {
    overflow: hidden;
    isolation: isolate;
  }

  .nav-search-field
    :global(.nav-search-scope-button[data-scope='all']::before) {
    content: '';
    position: absolute;
    top: 50%;
    left: 50%;
    width: 800px;
    height: 800px;
    background: linear-gradient(
      to right,
      #ff2080,
      #c830ff,
      #5050ff,
      #00a0ff,
      #00d4a0,
      #60e840,
      #e8d020,
      #ff8020,
      #ff2080
    );
    background-size: 25% 100%;
    z-index: -2;
    pointer-events: none;
    opacity: 0;
    transform: translate(-50%, -50%) rotate(135deg);
    animation: scope-rainbow-slide 2.4s linear infinite;
    transition: opacity 0.3s ease;
  }

  .nav-search-field :global(.nav-search-scope-button[data-scope='all']::after) {
    content: '';
    position: absolute;
    inset: 0;
    z-index: -1;
    pointer-events: none;
    opacity: 0;
    background: radial-gradient(
        circle,
        rgba(255, 255, 255, 0.32) 0.7px,
        transparent 0.7px
      )
      0 0 / 3.5px 3.5px;
    transition: opacity 0.3s ease;
  }

  .nav-search-field :global(.nav-search-scope-button[data-scope='all']:hover) {
    border-color: rgba(255, 255, 255, 0.48);
    background: transparent;
    color: #fff;
    text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
    -webkit-text-stroke: 2px rgba(0, 0, 0, 0.5);
    paint-order: stroke fill;
    box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.32);
  }

  .nav-search-field
    :global(.nav-search-scope-button[data-scope='all']:hover::before) {
    opacity: 1;
  }

  .nav-search-field
    :global(.nav-search-scope-button[data-scope='all']:hover::after) {
    opacity: 1;
  }

  .nav-search-field :global(.nav-search-scope-button:active) {
    transform: scaleX(0.92);
    box-shadow:
      inset 0 1px 2px rgba(15, 23, 42, 0.08),
      0 2px 6px rgba(15, 23, 42, 0.06);
  }

  @keyframes scope-rainbow-slide {
    from {
      transform: translate(-50%, -50%) rotate(135deg) translateX(0);
    }
    to {
      transform: translate(-50%, -50%) rotate(135deg) translateX(-25%);
    }
  }
</style>
