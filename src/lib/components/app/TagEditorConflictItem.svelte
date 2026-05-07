<script lang="ts">
  import { Button } from '$lib/components/ui/button/index.js';
  import type { ConflictResolution, TagEditorMergeConflict } from '$lib/types';

  interface Props {
    conflict: TagEditorMergeConflict;
    onResolve: (
      conflict: TagEditorMergeConflict,
      resolution: ConflictResolution
    ) => Promise<void>;
  }

  let { conflict, onResolve }: Props = $props();

  function formatValues(
    values: Record<string, string | undefined>[] | null
  ): string {
    if (!values || values.length === 0) return '(空)';
    return values
      .map((v) => v['zh-CN'] || v['en-US'] || Object.values(v)[0] || '')
      .join(', ');
  }
</script>

<div class="conflict-item">
  <div class="conflict-meta">
    <span class="conflict-entity"
      >{conflict.entityType === 'album' ? '专辑' : '单曲'}: {conflict.cid}</span
    >
    <span class="conflict-dim">维度: {conflict.dimensionKey}</span>
  </div>

  <div class="conflict-values">
    <div class="value-row">
      <span class="value-label">基线:</span>
      <span class="value-text">{formatValues(conflict.baseValues)}</span>
    </div>
    <div class="value-row">
      <span class="value-label">远端:</span>
      <span class="value-text remote"
        >{formatValues(conflict.remoteValues)}</span
      >
    </div>
    <div class="value-row">
      <span class="value-label">本地:</span>
      <span class="value-text local">{formatValues(conflict.localValues)}</span>
    </div>
  </div>

  <div class="conflict-actions">
    <Button size="sm" onclick={() => onResolve(conflict, 'keepLocal')}
      >保留本地</Button
    >
    <Button
      size="sm"
      variant="outline"
      onclick={() => onResolve(conflict, 'keepRemote')}>保留远端</Button
    >
  </div>
</div>

<style>
  .conflict-item {
    padding: 0.75rem;
    border: 1px solid var(--color-border, #e5e7eb);
    border-radius: 8px;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .conflict-meta {
    display: flex;
    gap: 1rem;
    font-size: 0.75rem;
    font-family: var(--font-mono, monospace);
    color: var(--color-text-secondary, #6b7280);
  }

  .conflict-values {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .value-row {
    display: flex;
    gap: 0.5rem;
    font-size: 0.75rem;
    font-family: var(--font-sans);
  }

  .value-label {
    width: 3rem;
    flex-shrink: 0;
    color: var(--color-text-secondary, #9ca3af);
  }

  .value-text {
    color: var(--color-text-primary, #374151);
  }

  .value-text.remote {
    color: var(--color-info, #2563eb);
  }

  .value-text.local {
    color: var(--color-success, #16a34a);
  }

  .conflict-actions {
    display: flex;
    gap: 0.5rem;
    padding-top: 0.25rem;
  }
</style>
