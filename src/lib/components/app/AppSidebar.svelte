<script lang="ts">
  import BrandLogo from '$lib/components/app/BrandLogo.svelte';
  import SidebarNav from '$lib/components/app/SidebarNav.svelte';
  import CollectionSidebarSection from '$lib/components/app/CollectionSidebarSection.svelte';
  import type { CollectionSummary } from '$lib/types';
  import type { AppView } from '$lib/features/shell/store.svelte';

  interface Props {
    isMacOS: boolean;
    currentView: AppView;
    searchQuery: string;
    onNavigate: (view: AppView) => void;
    onSearchQueryChange: (query: string) => void;
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

  .sidebar-collections-region {
    flex: 1;
    min-height: 0;
    overflow-y: auto;
    padding: 12px 16px;
    scrollbar-width: thin;
    scrollbar-color: transparent transparent;
  }

  .sidebar-collections-region:hover {
    scrollbar-color: rgba(255, 255, 255, 0.28) transparent;
  }
</style>
