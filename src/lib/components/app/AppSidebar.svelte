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
  import PanelLeftIcon from '@lucide/svelte/icons/panel-left';

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
    onToggle: () => void;
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
    collapsed,
    onToggle,
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

<aside class="sidebar" class:collapsed>
  {#if isMacOS}
    <div
      class="sidebar-drag-region"
      data-tauri-drag-region
      aria-hidden="true"
    ></div>
  {/if}

  <div class="sidebar-header" class:macos={isMacOS}>
    <button
      type="button"
      class="collapse-toggle"
      onclick={onToggle}
      aria-label={collapsed ? 'Expand sidebar' : 'Collapse sidebar'}
    >
      <PanelLeftIcon size={16} />
    </button>
  </div>

  <BrandLogo {isMacOS} {collapsed} />

  <div class="sidebar-nav-region">
    <SidebarNav {currentView} {onNavigate} {collapsed} />
  </div>

  {#if !collapsed}
    <div class="sidebar-collections-region">
      <CollapsibleGroup
        title={labels.official}
        icon={StarIcon}
        empty={officialCollections.length === 0}
      >
        <div
          class="collection-list"
          role="listbox"
          aria-label={labels.official}
        >
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
  {/if}

  <div class="sidebar-bottom" class:collapsed>
    <button
      type="button"
      class="bottom-nav-item"
      class:active={currentView === 'tagEditor'}
      onclick={() => onNavigate('tagEditor')}
      aria-current={currentView === 'tagEditor' ? 'page' : undefined}
      title={collapsed ? labels.tags : undefined}
    >
      <TagIcon size={16} aria-hidden="true" />
      <span class="bottom-nav-label">{labels.tags}</span>
    </button>
  </div>
</aside>

<style>
  .sidebar-nav-region {
    flex-shrink: 0;
    padding: 16px 8px 0;
  }

  .sidebar-collections-region {
    flex: 1;
    overflow-y: auto;
    padding: 0 16px;
    display: flex;
    flex-direction: column;
    gap: 2px;
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

  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    padding: 12px 12px 0;
    flex-shrink: 0;
  }

  .sidebar-header.macos {
    padding-top: 40px;
  }

  .collapsed .sidebar-header {
    justify-content: center;
    padding: 12px 0 0;
  }

  .collapsed .sidebar-header.macos {
    padding-top: 40px;
  }

  .collapse-toggle {
    appearance: none;
    border: none;
    background: rgba(255, 255, 255, 0.06);
    color: var(--text-tertiary);
    cursor: pointer;
    width: 28px;
    height: 28px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition:
      background-color 0.15s ease,
      color 0.15s ease;
  }

  .collapse-toggle:hover {
    background: rgba(255, 255, 255, 0.12);
    color: var(--text-primary);
  }

  .sidebar-bottom.collapsed .bottom-nav-item {
    justify-content: center;
    padding: 0;
  }

  .bottom-nav-label {
    overflow: hidden;
    white-space: nowrap;
    transition:
      opacity 200ms ease,
      width 200ms ease;
  }

  .collapsed .bottom-nav-label {
    opacity: 0;
    width: 0;
    pointer-events: none;
  }

  @media (prefers-reduced-motion: reduce) {
    .bottom-nav-label {
      transition: none;
    }
  }
</style>
