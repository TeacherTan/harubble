<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import AlbumSidebar from '$lib/components/app/AlbumSidebar.svelte';
  import type {
    Album,
    LibrarySearchScope,
    SearchLibraryResponse,
    SearchLibraryResultItem,
  } from '$lib/types';
  import type { AppView } from '$lib/features/shell/store.svelte';

  interface Props {
    isMacOS: boolean;
    currentView: AppView;
    albums: Album[];
    selectedAlbumCid: string | null;
    reducedMotion: boolean;
    loadingAlbums: boolean;
    errorMsg: string;
    searchQuery: string;
    searchScope: LibrarySearchScope;
    searchLoading: boolean;
    searchResponse: SearchLibraryResponse | null;
    onNavigateHome: () => void;
    onSearchQueryChange: (query: string) => void;
    onSearchScopeChange: (scope: LibrarySearchScope) => void;
    onSelect: (album: Album) => void | Promise<void>;
    onSelectSearchResult: (
      item: SearchLibraryResultItem
    ) => void | Promise<void>;
  }

  let props: Props = $props();
</script>

<aside class="sidebar">
  {#if props.isMacOS}
    <div
      class="sidebar-drag-region"
      data-tauri-drag-region
      aria-hidden="true"
    ></div>
  {/if}
  <button
    class="home-nav-button"
    class:active={props.currentView === 'home'}
    onclick={props.onNavigateHome}
    type="button"
    aria-label={m.shell_nav_home()}
  >
    <svg
      width="16"
      height="16"
      viewBox="0 0 16 16"
      fill="currentColor"
      aria-hidden="true"
    >
      <path d="M8 1.5l-6 5v7.5h4v-4h4v4h4v-7.5l-6-5z" />
    </svg>
    {m.shell_nav_home()}
  </button>
  <AlbumSidebar
    albums={props.albums}
    selectedAlbumCid={props.selectedAlbumCid}
    reducedMotion={props.reducedMotion}
    loadingAlbums={props.loadingAlbums}
    errorMsg={props.errorMsg}
    searchQuery={props.searchQuery}
    searchScope={props.searchScope}
    searchLoading={props.searchLoading}
    searchResponse={props.searchResponse}
    onSearchQueryChange={props.onSearchQueryChange}
    onSearchScopeChange={props.onSearchScopeChange}
    onSelect={props.onSelect}
    onSelectSearchResult={props.onSelectSearchResult}
  />
</aside>

<style>
  .home-nav-button {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    width: 100%;
    padding: 0.5rem 0.75rem;
    margin-bottom: 0.5rem;
    border: none;
    border-radius: 8px;
    background: none;
    color: var(--text-secondary, rgba(255, 255, 255, 0.6));
    font-family: var(--font-body);
    font-size: 0.8125rem;
    font-weight: 500;
    cursor: pointer;
    transition:
      background 0.15s ease,
      color 0.15s ease;
  }

  .home-nav-button:hover {
    background: var(--surface-secondary, rgba(255, 255, 255, 0.06));
    color: var(--text-primary, #fff);
  }

  .home-nav-button.active {
    background: var(--surface-secondary, rgba(255, 255, 255, 0.08));
    color: var(--text-primary, #fff);
  }
</style>
