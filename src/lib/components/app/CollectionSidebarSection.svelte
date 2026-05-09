<script lang="ts">
  import PlusIcon from '@lucide/svelte/icons/plus';
  import ImportIcon from '@lucide/svelte/icons/download';
  import type { CollectionSummary } from '$lib/types';

  interface Props {
    collections: CollectionSummary[];
    selectedCollectionId: string | null;
    isLoading: boolean;
    onSelect: (id: string) => void;
    onCreate: () => void;
    onImport: () => void;
  }

  let {
    collections,
    selectedCollectionId,
    isLoading,
    onSelect,
    onCreate,
    onImport,
  }: Props = $props();

  const officialCollections = $derived.by(() =>
    collections.filter((c) => c.isOfficial)
  );
  const userCollections = $derived.by(() =>
    collections.filter((c) => !c.isOfficial)
  );
</script>

<div class="collection-sidebar-section">
  <div class="section-header">
    <span class="section-title">合集</span>
    <div class="section-actions">
      <button
        type="button"
        class="section-action-btn"
        title="导入合集"
        aria-label="导入合集"
        onclick={onImport}
      >
        <ImportIcon size={14} />
      </button>
      <button
        type="button"
        class="section-action-btn"
        title="新建合集"
        aria-label="新建合集"
        onclick={onCreate}
      >
        <PlusIcon size={14} />
      </button>
    </div>
  </div>

  {#if isLoading}
    <div class="collection-loading">加载中…</div>
  {:else if collections.length === 0}
    <div class="collection-empty">暂无合集</div>
  {:else}
    <div class="collection-list">
      {#each officialCollections as collection (collection.id)}
        <button
          type="button"
          class="collection-item"
          class:is-selected={selectedCollectionId === collection.id}
          class:is-official={true}
          onclick={() => onSelect(collection.id)}
        >
          <span class="collection-name">
            <span class="official-badge">★</span>
            {collection.name}
          </span>
          <span class="collection-count">{collection.songCount}</span>
        </button>
      {/each}
      {#each userCollections as collection (collection.id)}
        <button
          type="button"
          class="collection-item"
          class:is-selected={selectedCollectionId === collection.id}
          onclick={() => onSelect(collection.id)}
        >
          <span class="collection-name">{collection.name}</span>
          <span class="collection-count">{collection.songCount}</span>
        </button>
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

  .collection-loading,
  .collection-empty {
    padding: 12px 10px;
    font-size: 12px;
    color: var(--text-tertiary);
  }
</style>
