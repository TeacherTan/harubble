<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { localeState } from '$lib/i18n';
  import BrandLogo from '$lib/components/app/BrandLogo.svelte';
  import SidebarNav from '$lib/components/app/SidebarNav.svelte';
  import { CollapsibleGroup } from '$lib/components/ui/collapsible-group';
  import PlusIcon from '@lucide/svelte/icons/plus';
  import ListMusicIcon from '@lucide/svelte/icons/list-music';
  import StarIcon from '@lucide/svelte/icons/star';
  import TagIcon from '@lucide/svelte/icons/tag';

  import type { AppView } from '$lib/features/shell/store.svelte';
  import type { CollectionSummary } from '$lib/types';

  interface Props {
    isMacOS: boolean;
    currentView: AppView;
    onNavigate: (view: AppView) => void;
    collections: CollectionSummary[];
    selectedCollectionId: string | null;
    isCollectionsLoading: boolean;
    onSelectCollection: (id: string) => void;
    onCreateCollection: () => void;
    onPlayCollection?: (id: string) => void;
    collapsed: boolean;
    contentCollapsed: boolean;
    contentInteractive: boolean;
    layoutCollapsed: boolean;
    sidebarEl?: HTMLElement | null;
    navRegionEl?: HTMLElement | null;
    collectionsRegionEl?: HTMLElement | null;
    collectionsCollapsedEl?: HTMLElement | null;
    bottomLabelEl?: HTMLSpanElement | null;
    logoContainerEl?: HTMLDivElement | null;
    onCharsReady?: (els: HTMLSpanElement[]) => void;
  }

  let {
    isMacOS,
    currentView,
    onNavigate,
    collections,
    selectedCollectionId,
    isCollectionsLoading: _isCollectionsLoading,
    onSelectCollection,
    onCreateCollection,
    onPlayCollection: _onPlayCollection,
    collapsed: _collapsed,
    contentCollapsed,
    contentInteractive,
    layoutCollapsed,
    sidebarEl = $bindable(null),
    navRegionEl = $bindable(null),
    collectionsRegionEl = $bindable(null),
    collectionsCollapsedEl = $bindable(null),
    bottomLabelEl = $bindable(null),
    logoContainerEl = $bindable(null),
    onCharsReady,
  }: Props = $props();

  const officialCollections = $derived.by(() =>
    collections.filter((c) => c.isOfficial)
  );
  const userCollections = $derived.by(() =>
    collections.filter((c) => !c.isOfficial)
  );

  const labels = $derived.by(() => {
    void localeState.current;
    return {
      official: m.sidebar_collections_official(),
      custom: m.sidebar_collections_custom(),
      create: m.sidebar_collections_create(),
      tags: m.shell_nav_tags(),
    };
  });
</script>

<aside class="sidebar" class:collapsed={contentCollapsed} bind:this={sidebarEl}>
  {#if isMacOS}
    <div
      class="sidebar-drag-region"
      data-tauri-drag-region
      aria-hidden="true"
    ></div>
  {/if}

  <BrandLogo
    {isMacOS}
    {layoutCollapsed}
    bind:containerEl={logoContainerEl}
    {onCharsReady}
  />

  <div class="sidebar-nav-region" bind:this={navRegionEl}>
    <SidebarNav {currentView} {onNavigate} collapsed={contentCollapsed} />
  </div>

  <div
    class="sidebar-collections-collapsed"
    class:hidden={!contentCollapsed}
    bind:this={collectionsCollapsedEl}
  >
    <button
      type="button"
      class="collapsed-collection-btn"
      title={labels.official}
      aria-label={labels.official}
    >
      <StarIcon size={16} aria-hidden="true" />
    </button>
    <button
      type="button"
      class="collapsed-collection-btn"
      title={labels.custom}
      aria-label={labels.custom}
      onclick={onCreateCollection}
    >
      <ListMusicIcon size={16} aria-hidden="true" />
    </button>
  </div>

  <div
    class="sidebar-collections-region"
    class:hidden={contentCollapsed}
    style:pointer-events={contentInteractive ? undefined : 'none'}
    bind:this={collectionsRegionEl}
  >
    <CollapsibleGroup
      title={labels.official}
      icon={StarIcon}
      empty={officialCollections.length === 0}
    >
      <div class="collection-list" role="listbox" aria-label={labels.official}>
        {#each officialCollections as collection (collection.id)}
          <button
            type="button"
            class="collection-item"
            class:active={selectedCollectionId === collection.id}
            role="option"
            aria-selected={selectedCollectionId === collection.id}
            onclick={() => onSelectCollection(collection.id)}
          >
            <ListMusicIcon size={16} aria-hidden="true" />
            <span>{collection.name}</span>
          </button>
        {/each}
      </div>
    </CollapsibleGroup>

    <CollapsibleGroup
      title={labels.custom}
      icon={ListMusicIcon}
      empty={userCollections.length === 0}
    >
      {#snippet actions()}
        <button
          type="button"
          class="section-action-btn"
          title={labels.create}
          aria-label={labels.create}
          onclick={onCreateCollection}
        >
          <PlusIcon size={14} />
        </button>
      {/snippet}
      <div class="collection-list" role="listbox" aria-label={labels.custom}>
        {#each userCollections as collection (collection.id)}
          <button
            type="button"
            class="collection-item"
            class:active={selectedCollectionId === collection.id}
            role="option"
            aria-selected={selectedCollectionId === collection.id}
            onclick={() => onSelectCollection(collection.id)}
          >
            <ListMusicIcon size={16} aria-hidden="true" />
            <span>{collection.name}</span>
          </button>
        {/each}
      </div>
    </CollapsibleGroup>
  </div>

  <div class="sidebar-bottom" class:collapsed={contentCollapsed}>
    <button
      type="button"
      class="bottom-nav-item"
      class:active={currentView === 'tagEditor'}
      onclick={() => onNavigate('tagEditor')}
      aria-current={currentView === 'tagEditor' ? 'page' : undefined}
      title={contentCollapsed ? labels.tags : undefined}
    >
      <TagIcon size={16} aria-hidden="true" />
      <span
        class="bottom-nav-label"
        class:hidden={contentCollapsed}
        bind:this={bottomLabelEl}>{labels.tags}</span
      >
    </button>
  </div>
</aside>

<style>
  .sidebar-nav-region {
    flex-shrink: 0;
    padding: 16px 8px 0;
  }

  .sidebar.collapsed .sidebar-nav-region {
    padding: 16px 0 0;
  }

  .sidebar-collections-region {
    flex: 1;
    overflow-y: auto;
    padding: 0 16px;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .sidebar-collections-region.hidden {
    pointer-events: none;
    overflow: hidden;
    flex: 0;
    height: 0;
    padding: 0;
  }

  .sidebar-collections-collapsed {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2px;
    padding: 8px 4px;
  }

  .sidebar-collections-collapsed.hidden {
    pointer-events: none;
    overflow: hidden;
    height: 0;
    padding: 0;
  }

  .collapsed-collection-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    border: none;
    border-radius: 8px;
    background: none;
    color: var(--text-secondary, rgba(255, 255, 255, 0.6));
    cursor: pointer;
    transition:
      background var(--motion-fast) ease,
      color var(--motion-fast) ease;
  }

  .collapsed-collection-btn:hover {
    background: var(--hover-bg-elevated);
    color: var(--text-primary);
  }

  .collection-list {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .collection-item {
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
    text-align: left;
    transition:
      background var(--motion-fast) ease,
      color var(--motion-fast) ease;
  }

  .collection-item:hover {
    background: var(--hover-bg-elevated);
    color: var(--text-primary);
  }

  .collection-item.active {
    background: var(--surface-state);
    color: var(--text-primary);
    font-weight: 600;
  }

  .collection-item span {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .section-action-btn {
    appearance: none;
    border: none;
    background: none;
    color: var(--text-tertiary);
    cursor: pointer;
    padding: 4px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition:
      background-color 0.15s ease,
      color 0.15s ease;
  }

  .section-action-btn:hover {
    background: rgba(255, 255, 255, 0.08);
    color: var(--text-primary);
  }

  .sidebar-bottom {
    flex-shrink: 0;
    margin-top: auto;
    padding: 8px 16px 12px;
    border-top: 1px solid rgba(255, 255, 255, 0.06);
  }

  .bottom-nav-item {
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

  .bottom-nav-item:hover {
    background: var(--hover-bg-elevated);
    color: var(--text-primary);
  }

  .bottom-nav-item.active {
    background: var(--surface-state);
    color: var(--text-primary);
    font-weight: 600;
  }

  .sidebar-bottom.collapsed .bottom-nav-item {
    justify-content: center;
    padding: 0;
  }

  .bottom-nav-label {
    overflow: hidden;
    white-space: nowrap;
  }

  .bottom-nav-label.hidden {
    max-width: 0;
    opacity: 0;
  }
</style>
