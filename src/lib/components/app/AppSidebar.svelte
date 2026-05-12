<script lang="ts">
  import BrandLogo from '$lib/components/app/BrandLogo.svelte';
  import SidebarNav from '$lib/components/app/SidebarNav.svelte';

  import type { AppView } from '$lib/features/shell/store.svelte';

  interface Props {
    isMacOS: boolean;
    currentView: AppView;
    onNavigate: (view: AppView) => void;
  }

  let { isMacOS, currentView, onNavigate }: Props = $props();
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
</aside>

<style>
  .sidebar-nav-region {
    flex-shrink: 0;
    padding: 16px 8px 0;
  }
</style>
