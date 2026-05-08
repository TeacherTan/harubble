<script lang="ts">
  import { Popover } from 'bits-ui';
  import * as m from '$lib/paraglide/messages.js';
  import { localeState } from '$lib/i18n';
  import type { AlbumDetail, SongEntry } from '$lib/types';

  interface AlbumMetadata {
    kind: 'album';
    album: AlbumDetail;
  }

  interface SongMetadata {
    kind: 'song';
    song: SongEntry;
    albumCid: string;
  }

  export type MetadataTarget = AlbumMetadata | SongMetadata;

  interface Props {
    target: MetadataTarget;
  }

  let { target }: Props = $props();

  const title = $derived.by(() => {
    return target.kind === 'album' ? target.album.name : target.song.name;
  });

  interface MetadataRow {
    label: string;
    value: string;
  }

  const triggerAriaLabel = $derived.by(() => {
    void localeState.current;
    return m.metadata_trigger_aria();
  });

  const rows = $derived.by((): MetadataRow[] => {
    void localeState.current;
    if (target.kind === 'album') {
      const a = target.album;
      const result: MetadataRow[] = [
        { label: m.metadata_label_cid(), value: a.cid },
        { label: m.metadata_label_album_name(), value: a.name },
      ];
      if (a.artists && a.artists.length > 0) {
        result.push({
          label: m.metadata_label_artists(),
          value: a.artists.join(', '),
        });
      }
      if (a.belong) {
        result.push({ label: m.metadata_label_belong(), value: a.belong });
      }
      if (a.intro) {
        result.push({ label: m.metadata_label_intro(), value: a.intro });
      }
      if (a.coverUrl) {
        result.push({ label: m.metadata_label_cover_url(), value: a.coverUrl });
      }
      if (a.coverDeUrl) {
        result.push({
          label: m.metadata_label_cover_de_url(),
          value: a.coverDeUrl,
        });
      }
      result.push({
        label: m.metadata_label_song_count(),
        value: String(a.songs.length),
      });
      if (a.tags.length > 0) {
        result.push({
          label: m.metadata_label_tags(),
          value: a.tags
            .map((t) => `${t.dimension}: ${t.values.join(', ')}`)
            .join(' | '),
        });
      }
      return result;
    }
    const s = target.song;
    const result: MetadataRow[] = [
      { label: m.metadata_label_cid(), value: s.cid },
      { label: m.metadata_label_song_name(), value: s.name },
      { label: m.metadata_label_album_cid(), value: target.albumCid },
      { label: m.metadata_label_artists(), value: s.artists.join(', ') },
    ];
    if (s.tags.length > 0) {
      result.push({
        label: m.metadata_label_tags(),
        value: s.tags
          .map((t) => `${t.dimension}: ${t.values.join(', ')}`)
          .join(' | '),
      });
    }
    return result;
  });
</script>

<Popover.Root>
  <Popover.Trigger class="meta-trigger" aria-label={triggerAriaLabel}>
    {#if target.kind === 'album'}
      <svg viewBox="0 0 24 24" aria-hidden="true">
        <circle cx="6" cy="12" r="2"></circle>
        <circle cx="12" cy="12" r="2"></circle>
        <circle cx="18" cy="12" r="2"></circle>
      </svg>
    {:else}
      <svg viewBox="0 0 24 24" aria-hidden="true">
        <circle cx="12" cy="12" r="1.5"></circle>
        <circle cx="12" cy="6.5" r="1.5"></circle>
        <circle cx="12" cy="17.5" r="1.5"></circle>
      </svg>
    {/if}
  </Popover.Trigger>
  <Popover.Portal>
    <Popover.Content
      class="meta-popover-content"
      side="bottom"
      align="start"
      sideOffset={6}
    >
      <Popover.Arrow class="meta-popover-arrow" />
      <div class="meta-popover-header">{title}</div>
      <div class="meta-popover-rows">
        {#each rows as row (row.label)}
          <div class="meta-popover-row">
            <span class="meta-popover-label">{row.label}</span>
            <span class="meta-popover-value">{row.value}</span>
          </div>
        {/each}
      </div>
    </Popover.Content>
  </Popover.Portal>
</Popover.Root>

<style>
  :global(.meta-trigger) {
    appearance: none;
    width: 28px;
    height: 28px;
    border-radius: 50%;
    border: none;
    background: transparent;
    color: var(--text-tertiary);
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    flex-shrink: 0;
    transition:
      background-color 0.16s ease-out,
      color 0.16s ease-out;
  }
  :global(.meta-trigger:hover) {
    background: rgba(var(--accent-rgb), 0.1);
    color: var(--accent);
  }
  :global(.meta-trigger svg) {
    width: 18px;
    height: 18px;
    fill: currentColor;
  }
  :global(.meta-popover-content) {
    z-index: var(--z-popover, 200);
    width: 320px;
    max-width: calc(100vw - 32px);
    max-height: 60vh;
    overflow-y: auto;
    padding: 12px 14px;
    border-radius: 12px;
    background: var(--bg-primary);
    border: 1px solid rgba(var(--accent-rgb), 0.1);
    box-shadow:
      0 8px 32px rgba(15, 23, 42, 0.12),
      0 2px 8px rgba(15, 23, 42, 0.06);
    animation: meta-popover-in 0.15s ease-out;
  }
  :global(.meta-popover-arrow) {
    fill: var(--bg-primary);
    stroke: rgba(var(--accent-rgb), 0.1);
    stroke-width: 1px;
  }
  @keyframes meta-popover-in {
    from {
      opacity: 0;
      transform: translateY(-4px) scale(0.97);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }
  .meta-popover-header {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 8px;
    padding-bottom: 6px;
    border-bottom: 1px solid rgba(var(--accent-rgb), 0.08);
  }
  .meta-popover-rows {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  .meta-popover-row {
    display: flex;
    gap: 10px;
    align-items: baseline;
  }
  .meta-popover-label {
    flex-shrink: 0;
    min-width: 64px;
    font-size: 11px;
    font-weight: 600;
    color: var(--text-secondary);
  }
  .meta-popover-value {
    font-size: 11px;
    color: var(--text-primary);
    word-break: break-all;
    font-family: var(--font-mono);
    line-height: 1.4;
  }
</style>
