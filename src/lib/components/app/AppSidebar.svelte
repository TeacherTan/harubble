<script lang="ts">
  import BrandLogo from '$lib/components/app/BrandLogo.svelte';
  import SidebarNav from '$lib/components/app/SidebarNav.svelte';
  import AlbumSidebarSection from '$lib/components/app/AlbumSidebarSection.svelte';
  import CollectionSidebarSection from '$lib/components/app/CollectionSidebarSection.svelte';
  import type {
    Album,
    CollectionSummary,
    LibrarySearchScope,
    SearchLibraryResponse,
    SearchLibraryResultItem,
  } from '$lib/types';
  import type { AppView } from '$lib/features/shell/store.svelte';

  interface Props {
    isMacOS: boolean;
    currentView: AppView;
    searchQuery: string;
    onNavigate: (view: AppView) => void;
    onSearchQueryChange: (query: string) => void;

    albums: Album[];
    selectedAlbumCid: string | null;
    reducedMotion: boolean;
    loadingAlbums: boolean;
    errorMsg: string;
    searchScope: LibrarySearchScope;
    searchLoading: boolean;
    searchResponse: SearchLibraryResponse | null;
    onSearchScopeChange: (scope: LibrarySearchScope) => void;
    onSelectAlbum: (album: Album) => void;
    onSelectSearchResult: (item: SearchLibraryResultItem) => void;

    collections: CollectionSummary[];
    selectedCollectionId: string | null;
    collectionsLoading: boolean;
    onCollectionSelect: (id: string) => void;
    onCollectionCreate: () => void;
    onCollectionImport: () => void;
  }

  let {
    isMacOS,
    currentView,
    searchQuery,
    onNavigate,
    onSearchQueryChange,
    albums,
    selectedAlbumCid,
    reducedMotion,
    loadingAlbums,
    errorMsg,
    searchScope,
    searchLoading,
    searchResponse,
    onSearchScopeChange,
    onSelectAlbum,
    onSelectSearchResult,
    collections,
    selectedCollectionId,
    collectionsLoading,
    onCollectionSelect,
    onCollectionCreate,
    onCollectionImport,
  }: Props = $props();
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
    <SidebarNav
      {currentView}
      {searchQuery}
      {onNavigate}
      {onSearchQueryChange}
    />
  </div>

  <div class="sidebar-library-region">
    <AlbumSidebarSection
      {albums}
      {selectedAlbumCid}
      {reducedMotion}
      {loadingAlbums}
      {errorMsg}
      {searchQuery}
      {searchScope}
      {searchLoading}
      {searchResponse}
      {onSearchQueryChange}
      {onSearchScopeChange}
      onSelect={onSelectAlbum}
      {onSelectSearchResult}
    />
  </div>

  <div class="sidebar-collections-region">
    <CollectionSidebarSection
      {collections}
      {selectedCollectionId}
      isLoading={collectionsLoading}
      onSelect={onCollectionSelect}
      onCreate={onCollectionCreate}
      onImport={onCollectionImport}
    />
  </div>
</aside>

<style>
  .sidebar-nav-region {
    flex-shrink: 0;
    padding: 16px 8px 0;
  }

  .sidebar-library-region {
    flex: 1;
    min-height: 0;
    padding: 24px 16px 0;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .sidebar-collections-region {
    flex-shrink: 0;
    max-height: 35%;
    overflow-y: auto;
    padding: 12px 16px 16px;
    border-top: 1px solid var(--border);
    scrollbar-width: thin;
    scrollbar-color: transparent transparent;
  }

  .sidebar-collections-region:hover {
    scrollbar-color: rgba(255, 255, 255, 0.28) transparent;
  }
</style>
