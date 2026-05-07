<script lang="ts">
  import { Button } from '$lib/components/ui/button/index.js';
  import type {
    TagEditorEntityType,
    TagEditorRegistry,
  } from '$lib/types';

  interface Props {
    merged: TagEditorRegistry;
    selectedEntityType: TagEditorEntityType;
    selectedCid: string | null;
    onSelectEntity: (type: TagEditorEntityType, cid: string) => void;
    onAddDimension: (key: string, labelZh: string, labelEn: string) => Promise<void>;
    onRemoveDimension: (key: string) => Promise<void>;
  }

  let {
    merged,
    selectedEntityType,
    selectedCid,
    onSelectEntity,
    onAddDimension,
    onRemoveDimension,
  }: Props = $props();

  let activeTab = $state<TagEditorEntityType>('album');

  const entityCids = $derived.by(() => {
    const map = activeTab === 'album' ? merged.albums : merged.songs;
    return Object.keys(map).sort();
  });

  function handleSelectEntity(cid: string) {
    onSelectEntity(activeTab, cid);
  }

  let newDimKey = $state('');
  let newDimZh = $state('');
  let newDimEn = $state('');

  async function handleAddDimension() {
    if (!newDimKey.trim() || !newDimZh.trim()) return;
    await onAddDimension(newDimKey.trim(), newDimZh.trim(), newDimEn.trim());
    newDimKey = '';
    newDimZh = '';
    newDimEn = '';
  }
</script>

<aside class="tag-editor-sidebar">
  <section class="sidebar-section">
    <h3 class="sidebar-heading">维度管理</h3>
    <ul class="dimension-list">
      {#each merged.tagDimensions as dim (dim.key)}
        <li class="dimension-item">
          <span class="dimension-label">{dim.label['zh-CN'] ?? dim.key}</span>
          <button
            class="dimension-remove"
            onclick={() => onRemoveDimension(dim.key)}
            aria-label="删除维度 {dim.key}"
          >×</button>
        </li>
      {/each}
    </ul>
    <div class="add-dimension-form">
      <input bind:value={newDimKey} placeholder="key" class="dim-input" />
      <input bind:value={newDimZh} placeholder="中文名" class="dim-input" />
      <input bind:value={newDimEn} placeholder="English" class="dim-input" />
      <Button size="sm" onclick={handleAddDimension}>添加</Button>
    </div>
  </section>

  <section class="sidebar-section">
    <h3 class="sidebar-heading">实体列表</h3>
    <div class="entity-tabs">
      <button
        class="tab-btn"
        class:active={activeTab === 'album'}
        onclick={() => { activeTab = 'album'; }}
      >专辑</button>
      <button
        class="tab-btn"
        class:active={activeTab === 'song'}
        onclick={() => { activeTab = 'song'; }}
      >单曲</button>
    </div>
    <ul class="entity-list">
      {#each entityCids as cid (cid)}
        <li>
          <button
            class="entity-item"
            class:selected={selectedEntityType === activeTab && selectedCid === cid}
            onclick={() => handleSelectEntity(cid)}
          >
            {cid}
          </button>
        </li>
      {/each}
      {#if entityCids.length === 0}
        <li class="entity-empty">暂无数据</li>
      {/if}
    </ul>
  </section>
</aside>

<style>
  .tag-editor-sidebar {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    border-right: 1px solid var(--color-border, #e5e7eb);
    padding-right: 1rem;
  }

  .sidebar-heading {
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--color-text-secondary, #6b7280);
    margin-bottom: 0.5rem;
    font-family: var(--font-sans);
  }

  .sidebar-section {
    display: flex;
    flex-direction: column;
  }

  .dimension-list {
    list-style: none;
    padding: 0;
    margin: 0 0 0.5rem;
  }

  .dimension-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.25rem 0.5rem;
    font-size: 0.8125rem;
    font-family: var(--font-sans);
  }

  .dimension-label {
    color: var(--color-text-primary, #1f2937);
  }

  .dimension-remove {
    background: none;
    border: none;
    cursor: pointer;
    color: var(--color-text-secondary, #9ca3af);
    font-size: 1rem;
    line-height: 1;
    padding: 0 0.25rem;
  }

  .dimension-remove:hover {
    color: var(--color-danger, #ef4444);
  }

  .add-dimension-form {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .dim-input {
    font-size: 0.75rem;
    padding: 0.25rem 0.5rem;
    border: 1px solid var(--color-border, #d1d5db);
    border-radius: 4px;
    font-family: var(--font-sans);
  }

  .entity-tabs {
    display: flex;
    gap: 0.25rem;
    margin-bottom: 0.5rem;
  }

  .tab-btn {
    flex: 1;
    padding: 0.25rem 0.5rem;
    font-size: 0.75rem;
    border: 1px solid var(--color-border, #d1d5db);
    border-radius: 4px;
    background: transparent;
    cursor: pointer;
    font-family: var(--font-sans);
    color: var(--color-text-secondary, #6b7280);
  }

  .tab-btn.active {
    background: var(--color-primary, #3b82f6);
    color: white;
    border-color: var(--color-primary, #3b82f6);
  }

  .entity-list {
    list-style: none;
    padding: 0;
    margin: 0;
    max-height: 400px;
    overflow-y: auto;
  }

  .entity-item {
    display: block;
    width: 100%;
    text-align: left;
    padding: 0.375rem 0.5rem;
    font-size: 0.75rem;
    font-family: var(--font-mono, monospace);
    border: none;
    background: transparent;
    cursor: pointer;
    border-radius: 4px;
    color: var(--color-text-primary, #1f2937);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .entity-item:hover {
    background: var(--color-hover, #f3f4f6);
  }

  .entity-item.selected {
    background: var(--color-primary, #3b82f6);
    color: white;
  }

  .entity-empty {
    font-size: 0.75rem;
    color: var(--color-text-secondary, #9ca3af);
    padding: 0.5rem;
    font-family: var(--font-sans);
  }
</style>
