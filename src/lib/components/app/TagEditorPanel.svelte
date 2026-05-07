<script lang="ts">
  import TagEditorDimension from './TagEditorDimension.svelte';
  import type {
    TagEditorEntityType,
    TagEditorLocalizedValue,
    TagEditorRegistry,
  } from '$lib/types';

  interface Props {
    merged: TagEditorRegistry;
    selectedEntityType: TagEditorEntityType;
    selectedCid: string | null;
    selectedEntityTags: Record<string, TagEditorLocalizedValue[]>;
    onSetTag: (
      dimensionKey: string,
      values: TagEditorLocalizedValue[]
    ) => Promise<void>;
    onRemoveTag: (dimensionKey: string) => Promise<void>;
  }

  let {
    merged,
    selectedCid,
    selectedEntityTags,
    onSetTag,
    onRemoveTag,
  }: Props = $props();
</script>

<div class="tag-editor-panel">
  {#if !selectedCid}
    <div class="panel-empty">
      <p>请从左侧选择一个实体进行编辑</p>
    </div>
  {:else}
    <h3 class="panel-title">{selectedCid}</h3>

    <div class="dimension-rows">
      {#each merged.tagDimensions as dim (dim.key)}
        <TagEditorDimension
          dimensionKey={dim.key}
          dimensionLabel={dim.label['zh-CN'] ?? dim.key}
          values={selectedEntityTags[dim.key] ?? []}
          {onSetTag}
          {onRemoveTag}
        />
      {/each}
    </div>
  {/if}
</div>

<style>
  .tag-editor-panel {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .panel-empty {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 4rem 2rem;
    color: var(--color-text-secondary, #6b7280);
    font-family: var(--font-sans);
  }

  .panel-title {
    font-size: 0.875rem;
    font-weight: 600;
    font-family: var(--font-mono, monospace);
    color: var(--color-text-primary, #1f2937);
    padding-bottom: 0.5rem;
    border-bottom: 1px solid var(--color-border, #e5e7eb);
  }

  .dimension-rows {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
</style>
