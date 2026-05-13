<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { localeState } from '$lib/i18n';
  import BrandLogo from '$lib/components/app/BrandLogo.svelte';
  import SidebarNav from '$lib/components/app/SidebarNav.svelte';
  import { CollapsibleGroup } from '$lib/components/ui/collapsible-group';
  import PlusIcon from '@lucide/svelte/icons/plus';
  import PlayIcon from '@lucide/svelte/icons/play';

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
    onPlayCollection,
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
        <CollapsibleGroup title={labels.official}>
          <div
            class="collection-list"
            role="listbox"
            aria-label={labels.official}
          >
            {#each officialCollections as collection (collection.id)}
              <div
                class="collection-item"
                class:is-selected={selectedCollectionId === collection.id}
                role="option"
                tabindex="0"
                aria-selected={selectedCollectionId === collection.id}
                onclick={() => onSelectCollection(collection.id)}
                onkeydown={(e) => {
                  if (e.key === 'Enter' || e.key === ' ') {
                    e.preventDefault();
                    onSelectCollection(collection.id);
                  }
                }}
              >
                <span class="collection-name">
                  <span class="official-badge">★</span>
                  {collection.name}
                </span>
                <span class="collection-item-trailing">
                  <span class="collection-count">{collection.songCount}</span>
                  {#if onPlayCollection}
                    <button
                      type="button"
                      class="collection-play-btn"
                      aria-label={`播放 ${collection.name}`}
                      onclick={(e) => {
                        e.stopPropagation();
                        onPlayCollection(collection.id);
                      }}
                    >
                      <PlayIcon size={12} />
                    </button>
                  {/if}
                </span>
              </div>
            {/each}
          </div>
        </CollapsibleGroup>
      {/if}

      <CollapsibleGroup title={labels.custom}>
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
            <div
              class="collection-item"
              class:is-selected={selectedCollectionId === collection.id}
              role="option"
              tabindex="0"
              aria-selected={selectedCollectionId === collection.id}
              onclick={() => onSelectCollection(collection.id)}
              onkeydown={(e) => {
                if (e.key === 'Enter' || e.key === ' ') {
                  e.preventDefault();
                  onSelectCollection(collection.id);
                }
              }}
            >
              <span class="collection-name">{collection.name}</span>
              <span class="collection-item-trailing">
                <span class="collection-count">{collection.songCount}</span>
                {#if onPlayCollection}
                  <button
                    type="button"
                    class="collection-play-btn"
                    aria-label={`播放 ${collection.name}`}
                    onclick={(e) => {
                      e.stopPropagation();
                      onPlayCollection(collection.id);
                    }}
                  >
                    <PlayIcon size={12} />
                  </button>
                {/if}
              </span>
            </div>
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
    gap: 8px;
  }

  .collection-list {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .collection-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    width: 100%;
    padding: 6px 10px;
    border: none;
    border-radius: 8px;
    background: none;
    color: var(--text-secondary);
    font-family: var(--font-body);
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    text-align: left;
    transition:
      background-color 0.15s ease,
      color 0.15s ease;
  }

  .collection-item:hover {
    background: rgba(255, 255, 255, 0.06);
    color: var(--text-primary);
  }

  .collection-item.is-selected {
    background: rgba(var(--accent-rgb), 0.12);
    color: var(--text-primary);
  }

  .collection-name {
    display: flex;
    align-items: center;
    gap: 6px;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .official-badge {
    color: var(--accent);
    font-size: 12px;
    flex-shrink: 0;
  }

  .collection-count {
    font-size: 11px;
    color: var(--text-tertiary);
    flex-shrink: 0;
  }

  .collection-item-trailing {
    display: flex;
    align-items: center;
    gap: 4px;
    flex-shrink: 0;
  }

  .collection-play-btn {
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
    opacity: 0;
    transition:
      opacity 0.15s ease,
      background-color 0.15s ease,
      color 0.15s ease;
  }

  .collection-item:hover .collection-play-btn {
    opacity: 1;
  }

  .collection-play-btn:hover {
    background: rgba(var(--accent-rgb), 0.15);
    color: var(--accent);
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
