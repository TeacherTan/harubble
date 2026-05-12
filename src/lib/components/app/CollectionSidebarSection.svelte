<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { localeState } from '$lib/i18n';
  import PlusIcon from '@lucide/svelte/icons/plus';
  import ImportIcon from '@lucide/svelte/icons/download';
  import PlayIcon from '@lucide/svelte/icons/play';
  import type { CollectionSummary } from '$lib/types';

  interface Props {
    collections: CollectionSummary[];
    selectedCollectionId: string | null;
    isLoading: boolean;
    onSelect: (id: string) => void;
    onCreate: () => void;
    onImport: () => void;
    onPlay?: (id: string) => void;
  }

  let {
    collections,
    selectedCollectionId,
    isLoading,
    onSelect,
    onCreate,
    onImport,
    onPlay,
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
      title: m.sidebar_collections_title(),
      import: m.sidebar_collections_import(),
      create: m.sidebar_collections_create(),
      loading: m.sidebar_collections_loading(),
      empty: m.sidebar_collections_empty(),
    };
  });
</script>

<div class="collection-sidebar-section">
  <div class="section-header">
    <span class="section-title">{labels.title}</span>
    <div class="section-actions">
      <button
        type="button"
        class="section-action-btn"
        title={labels.import}
        aria-label={labels.import}
        onclick={onImport}
      >
        <ImportIcon size={14} />
      </button>
      <button
        type="button"
        class="section-action-btn"
        title={labels.create}
        aria-label={labels.create}
        onclick={onCreate}
      >
        <PlusIcon size={14} />
      </button>
    </div>
  </div>

  {#if isLoading}
    <div class="collection-loading">{labels.loading}</div>
  {:else if collections.length === 0}
    <div class="collection-empty">{labels.empty}</div>
  {:else}
    <div class="collection-list" role="listbox" aria-label={labels.title}>
      {#each officialCollections as collection (collection.id)}
        <div
          class="collection-item"
          class:is-selected={selectedCollectionId === collection.id}
          class:is-official={true}
          role="option"
          tabindex="0"
          aria-selected={selectedCollectionId === collection.id}
          onclick={() => onSelect(collection.id)}
          onkeydown={(e) => {
            if (e.key === 'Enter' || e.key === ' ') {
              e.preventDefault();
              onSelect(collection.id);
            }
          }}
        >
          <span class="collection-name">
            <span class="official-badge">★</span>
            {collection.name}
          </span>
          <span class="collection-item-trailing">
            <span class="collection-count">{collection.songCount}</span>
            {#if onPlay}
              <button
                type="button"
                class="collection-play-btn"
                aria-label={`播放 ${collection.name}`}
                onclick={(e) => {
                  e.stopPropagation();
                  onPlay(collection.id);
                }}
              >
                <PlayIcon size={12} />
              </button>
            {/if}
          </span>
        </div>
      {/each}
      {#each userCollections as collection (collection.id)}
        <div
          class="collection-item"
          class:is-selected={selectedCollectionId === collection.id}
          role="option"
          tabindex="0"
          aria-selected={selectedCollectionId === collection.id}
          onclick={() => onSelect(collection.id)}
          onkeydown={(e) => {
            if (e.key === 'Enter' || e.key === ' ') {
              e.preventDefault();
              onSelect(collection.id);
            }
          }}
        >
          <span class="collection-name">{collection.name}</span>
          <span class="collection-item-trailing">
            <span class="collection-count">{collection.songCount}</span>
            {#if onPlay}
              <button
                type="button"
                class="collection-play-btn"
                aria-label={`播放 ${collection.name}`}
                onclick={(e) => {
                  e.stopPropagation();
                  onPlay(collection.id);
                }}
              >
                <PlayIcon size={12} />
              </button>
            {/if}
          </span>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .collection-sidebar-section {
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 0 0 12px;
  }

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 4px 4px;
  }

  .section-title {
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--text-tertiary);
  }

  .section-actions {
    display: flex;
    gap: 2px;
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

  .collection-loading,
  .collection-empty {
    padding: 12px 10px;
    font-size: 12px;
    color: var(--text-tertiary);
  }
</style>
