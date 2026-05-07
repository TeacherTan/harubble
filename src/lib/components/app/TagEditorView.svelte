<script lang="ts">
  import { OverlayScrollbarsComponent } from 'overlayscrollbars-svelte';
  import TagEditorSidebar from './TagEditorSidebar.svelte';
  import TagEditorPanel from './TagEditorPanel.svelte';
  import TagEditorConflictList from './TagEditorConflictList.svelte';
  import type { PartialOptions } from 'overlayscrollbars';
  import type {
    ConflictResolution,
    TagEditorEntityType,
    TagEditorLocalizedValue,
    TagEditorMergeConflict,
    TagEditorRegistry,
  } from '$lib/types';

  interface Props {
    runtime: {
      tagEditorController: {
        merged: TagEditorRegistry | null;
        localOverlay: TagEditorRegistry | null;
        selectedEntityType: TagEditorEntityType;
        selectedCid: string | null;
        selectedEntityTags: Record<string, TagEditorLocalizedValue[]>;
        conflicts: TagEditorMergeConflict[];
        loading: boolean;
        loadData: () => Promise<void>;
        selectEntity: (type: TagEditorEntityType, cid: string) => void;
        setTag: (
          dimensionKey: string,
          values: TagEditorLocalizedValue[]
        ) => Promise<void>;
        removeTag: (dimensionKey: string) => Promise<void>;
        addDimension: (
          key: string,
          labelZh: string,
          labelEn: string
        ) => Promise<void>;
        removeDimension: (key: string) => Promise<void>;
        resolveConflict: (
          conflict: TagEditorMergeConflict,
          resolution: ConflictResolution
        ) => Promise<void>;
      };
      overlayScrollbarOptions: PartialOptions;
    };
  }

  let { runtime }: Props = $props();

  const controller = $derived(runtime.tagEditorController);

  $effect(() => {
    void controller.loadData();
  });
</script>

<div class="tag-editor-view">
  <OverlayScrollbarsComponent
    class="tag-editor-scroll"
    options={runtime.overlayScrollbarOptions}
    defer
  >
    {#if controller.loading && !controller.merged}
      <div class="tag-editor-loading">
        <p>加载中...</p>
      </div>
    {:else if controller.merged}
      <div class="tag-editor-layout">
        <TagEditorSidebar
          merged={controller.merged}
          selectedEntityType={controller.selectedEntityType}
          selectedCid={controller.selectedCid}
          onSelectEntity={controller.selectEntity}
          onAddDimension={controller.addDimension}
          onRemoveDimension={controller.removeDimension}
        />

        <TagEditorPanel
          merged={controller.merged}
          selectedEntityType={controller.selectedEntityType}
          selectedCid={controller.selectedCid}
          selectedEntityTags={controller.selectedEntityTags}
          onSetTag={controller.setTag}
          onRemoveTag={controller.removeTag}
        />
      </div>

      {#if controller.conflicts.length > 0}
        <TagEditorConflictList
          conflicts={controller.conflicts}
          onResolve={controller.resolveConflict}
        />
      {/if}
    {:else}
      <div class="tag-editor-empty">
        <p>暂无 Tag 数据</p>
      </div>
    {/if}
  </OverlayScrollbarsComponent>
</div>

<style>
  .tag-editor-view {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .tag-editor-view :global(.tag-editor-scroll) {
    flex: 1;
  }

  .tag-editor-layout {
    display: grid;
    grid-template-columns: 280px 1fr;
    gap: 1rem;
    padding: 1.5rem;
    min-height: 100%;
  }

  .tag-editor-loading,
  .tag-editor-empty {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 4rem;
    color: var(--color-text-secondary, #6b7280);
    font-family: var(--font-sans);
  }
</style>
