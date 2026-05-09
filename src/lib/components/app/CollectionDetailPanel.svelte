<script lang="ts">
  import { fade, fly } from 'svelte/transition';
  import CollectionSongRow from '$lib/components/app/CollectionSongRow.svelte';
  import type { Collection } from '$lib/types';

  interface Props {
    collection: Collection | null;
    isLoading: boolean;
    reducedMotion: boolean;
    onEdit: () => void;
    onDelete: (id: string) => void;
    onExport: (id: string) => void;
    onRemoveSongs: (collectionId: string, songIds: string[]) => void;
    onReorderSongs: (collectionId: string, songIds: string[]) => void;
  }

  let props: Props = $props();

  function dur(base: number): number {
    return props.reducedMotion ? 0 : base;
  }

  const isEditable = $derived.by(() => !props.collection?.isOfficial);
  const songCountLabel = $derived.by(
    () => `${props.collection?.songIds.length ?? 0} 首歌曲`
  );

  let dragSourceIndex = $state<number | null>(null);

  function handleDragStart(event: DragEvent, index: number) {
    dragSourceIndex = index;
    if (event.dataTransfer) {
      event.dataTransfer.effectAllowed = 'move';
      event.dataTransfer.setData('text/plain', String(index));
    }
  }

  function handleDragOver(_event: DragEvent, _index: number) {
    // Allow drop
  }

  function handleDrop(_event: DragEvent, targetIndex: number) {
    if (
      dragSourceIndex === null ||
      dragSourceIndex === targetIndex ||
      !props.collection
    ) {
      dragSourceIndex = null;
      return;
    }

    const newOrder = [...props.collection.songIds];
    const [moved] = newOrder.splice(dragSourceIndex, 1);
    newOrder.splice(targetIndex, 0, moved);

    dragSourceIndex = null;
    props.onReorderSongs(props.collection.id, newOrder);
  }
</script>

{#if props.isLoading}
  <div class="collection-detail-loading" in:fade={{ duration: dur(200) }}>
    <span>加载中…</span>
  </div>
{:else if !props.collection}
  <div class="collection-detail-loading" in:fade={{ duration: dur(200) }}>
    <span>请从侧边栏选择一个合集</span>
  </div>
{:else}
  {@const collection = props.collection}
  <div
    class="collection-detail-card"
    class:is-reduced-motion={props.reducedMotion}
    in:fade={{ duration: dur(220) }}
    out:fade={{ duration: dur(220) }}
  >
    <div class="collection-hero">
      <div
        class="collection-hero-info"
        in:fly={{ y: 14, duration: dur(220), delay: dur(30) }}
        out:fly={{ y: 8, duration: dur(220) }}
      >
        {#if collection.isOfficial}
          <span class="collection-official-tag">★ 官方合集</span>
        {/if}
        <h1 class="collection-hero-title">{collection.name}</h1>
        {#if collection.description}
          <p class="collection-hero-description">
            {collection.description}
          </p>
        {/if}
        <div class="collection-hero-meta">
          <span class="collection-song-count">{songCountLabel}</span>
        </div>
        <div class="controls collection-hero-actions">
          {#if isEditable}
            <button type="button" class="btn" onclick={props.onEdit}>
              编辑
            </button>
            <button
              type="button"
              class="btn btn-danger"
              onclick={() => {
                if (
                  confirm(
                    `确定要删除合集「${collection.name}」吗？此操作不可撤销。`
                  )
                ) {
                  props.onDelete(collection.id);
                }
              }}
            >
              删除
            </button>
          {/if}
          <button
            type="button"
            class="btn"
            onclick={() => props.onExport(collection.id)}
          >
            导出
          </button>
        </div>
      </div>
    </div>

    <div
      class="song-list"
      role="list"
      in:fly={{ y: 10, duration: dur(200), delay: dur(70) }}
      out:fade={{ duration: dur(200) }}
    >
      {#each collection.songIds as songId, index (songId)}
        <CollectionSongRow
          {songId}
          {index}
          draggable={isEditable}
          onDragStart={handleDragStart}
          onDragOver={handleDragOver}
          onDragEnd={() => {
            dragSourceIndex = null;
          }}
          onDrop={handleDrop}
          onRemove={(sid) => props.onRemoveSongs(collection.id, [sid])}
        />
      {/each}
      {#if collection.songIds.length === 0}
        <div class="empty-song-list">
          暂无歌曲，从专辑详情页将歌曲添加到此合集
        </div>
      {/if}
    </div>
  </div>
{/if}

<style>
  .collection-detail-loading {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 48px 24px;
    color: var(--text-tertiary);
    font-size: 14px;
  }

  .collection-detail-card {
    display: flex;
    flex-direction: column;
    gap: 24px;
    padding: 24px;
  }

  .collection-hero {
    display: flex;
    gap: 20px;
  }

  .collection-hero-info {
    display: flex;
    flex-direction: column;
    gap: 8px;
    min-width: 0;
  }

  .collection-official-tag {
    font-size: 11px;
    font-weight: 600;
    color: var(--accent);
    letter-spacing: 0.04em;
  }

  .collection-hero-title {
    font-size: 22px;
    font-weight: 700;
    color: var(--text-primary);
    margin: 0;
    line-height: 1.2;
  }

  .collection-hero-description {
    font-size: 13px;
    color: var(--text-secondary);
    margin: 0;
    line-height: 1.5;
  }

  .collection-hero-meta {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-top: 4px;
  }

  .collection-song-count {
    font-size: 12px;
    color: var(--text-tertiary);
  }

  .collection-hero-actions {
    display: flex;
    gap: 8px;
    margin-top: 8px;
  }

  .btn {
    appearance: none;
    border: 1px solid rgba(255, 255, 255, 0.12);
    background: rgba(255, 255, 255, 0.06);
    color: var(--text-primary);
    font-family: var(--font-body);
    font-size: 13px;
    font-weight: 500;
    padding: 6px 14px;
    border-radius: 8px;
    cursor: pointer;
    transition:
      background-color 0.15s ease,
      border-color 0.15s ease;
  }

  .btn:hover {
    background: rgba(255, 255, 255, 0.1);
    border-color: rgba(255, 255, 255, 0.2);
  }

  .btn-danger {
    color: #f87171;
    border-color: rgba(248, 113, 113, 0.3);
  }

  .btn-danger:hover {
    background: rgba(248, 113, 113, 0.1);
    border-color: rgba(248, 113, 113, 0.5);
  }

  .is-reduced-motion .btn {
    transition: none;
  }

  .song-list {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .empty-song-list {
    padding: 32px 16px;
    text-align: center;
    font-size: 13px;
    color: var(--text-tertiary);
  }
</style>
