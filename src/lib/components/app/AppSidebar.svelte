<script lang="ts">
  import BrandLogo from '$lib/components/app/BrandLogo.svelte';
  import SidebarNav from '$lib/components/app/SidebarNav.svelte';
  import CollectionSidebarSection from '$lib/components/app/CollectionSidebarSection.svelte';

  import type { AppView } from '$lib/features/shell/store.svelte';
  import type { CollectionSummary } from '$lib/types';

  interface Props {
    isMacOS: boolean;
    currentView: AppView;
    onNavigate: (view: AppView) => void;
    collections: CollectionSummary[];
    selectedCollectionId: string | null;
    isCollectionsLoading: boolean;
    onSelectCollection: (id: string) => void;
    onCreateCollection: () => void;
    onImportCollection: () => void;
    onPlayCollection?: (id: string) => void;
  }

  let {
    isMacOS,
    currentView,
    onNavigate,
    collections,
    selectedCollectionId,
    isCollectionsLoading,
    onSelectCollection,
    onCreateCollection,
    onImportCollection,
    onPlayCollection,
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
    <SidebarNav {currentView} {onNavigate} />
  </div>

  {#if currentView === 'collection'}
    <div class="sidebar-collection-region">
      <CollectionSidebarSection
        {collections}
        {selectedCollectionId}
        isLoading={isCollectionsLoading}
        onSelect={onSelectCollection}
        onCreate={onCreateCollection}
        onImport={onImportCollection}
        onPlay={onPlayCollection}
      />
    </div>
  {/if}
</aside>

<style>
  .sidebar-nav-region {
    flex-shrink: 0;
    padding: 16px 8px 0;
  }

  .sidebar-collection-region {
    flex: 1;
    overflow-y: auto;
    padding: 8px 8px 0;
  }
</style>
