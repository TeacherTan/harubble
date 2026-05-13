<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { localeState } from '$lib/i18n';
  import BrandLogo from '$lib/components/app/BrandLogo.svelte';
  import SidebarNav from '$lib/components/app/SidebarNav.svelte';
  import { CollapsibleGroup } from '$lib/components/ui/collapsible-group';
  import PlusIcon from '@lucide/svelte/icons/plus';
  import ListMusicIcon from '@lucide/svelte/icons/list-music';
  import StarIcon from '@lucide/svelte/icons/star';

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
  }

  let {
    isMacOS,
    currentView,
    onNavigate,
    collections,
    selectedCollectionId,
    isCollectionsLoading,
    onSelectCollection,
    onCreateCollection,
    onPlayCollection: _onPlayCollection,
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
      loading: m.sidebar_collections_loading(),
      empty: m.sidebar_collections_empty(),
    };
  });
</script>

<aside class="sidebar">
  {#if isMacOS}
    <div
      class="sidebar-drag-region"
      data-tauri-drag-region
      aria-hidden="true"
    ></div>
  {/if}

  <BrandLogo {isMacOS} />

  <div class="sidebar-nav-region">
    <SidebarNav {currentView} {onNavigate} />
  </div>

  <div class="sidebar-collections-region">
    {#if isCollectionsLoading}
      <div class="collection-loading">{labels.loading}</div>
    {:else if collections.length === 0}
      <div class="collection-empty">{labels.empty}</div>
    {:else}
      {#if officialCollections.length > 0}
        <CollapsibleGroup title={labels.official} icon={StarIcon}>
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
      {/if}

      <CollapsibleGroup title={labels.custom} icon={ListMusicIcon}>
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
    {/if}
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
    padding: 12px 8px 0;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .collection-list {
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: 0 8px;
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

  .collection-loading,
  .collection-empty {
    padding: 12px 10px;
    font-size: 12px;
    color: var(--text-tertiary);
  }
</style>
