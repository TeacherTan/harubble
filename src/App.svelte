<script lang="ts">
  import { createAppRuntime } from '$lib/features/shell/appRuntime.svelte';
  import TopToolbar from '$lib/components/app/TopToolbar.svelte';
  import StatusToastHost from '$lib/components/app/StatusToastHost.svelte';
  import AppSidebar from '$lib/components/app/AppSidebar.svelte';
  import LibraryView from '$lib/components/app/LibraryView.svelte';
  import PlayerFlyoutStack from '$lib/components/app/PlayerFlyoutStack.svelte';
  import FullscreenPlayer from '$lib/components/app/FullscreenPlayer.svelte';
  import AppSideSheets from '$lib/components/app/AppSideSheets.svelte';
  import HomeView from '$lib/components/app/HomeView.svelte';
  import TagEditorView from '$lib/components/app/TagEditorView.svelte';
  import CollectionDetailPanel from '$lib/components/app/CollectionDetailPanel.svelte';
  import CollectionFormDialog from '$lib/components/app/CollectionFormDialog.svelte';
  import AlbumOverview from '$lib/components/app/AlbumOverview.svelte';

  const runtime = createAppRuntime();
</script>

{#if runtime.isMacOS}
  <div
    class="macos-window-drag-region"
    data-tauri-drag-region
    aria-hidden="true"
  ></div>
{/if}

<StatusToastHost />

<div class="app-shell" class:macos-overlay={runtime.isMacOS}>
  <AppSidebar
    isMacOS={runtime.isMacOS}
    currentView={runtime.currentView}
    searchQuery={runtime.librarySearchQuery}
    searchScope={runtime.librarySearchScope}
    onNavigate={(view) => {
      runtime.shellStore.currentView = view;
    }}
    onSearchQueryChange={runtime.libraryController.setSearchQuery}
    onSearchScopeChange={runtime.libraryController.setSearchScope}
  />

  <section class="main-region">
    {#if runtime.isMacOS}
      <div
        class="main-drag-region"
        data-tauri-drag-region
        aria-hidden="true"
      ></div>
    {/if}

    <TopToolbar
      activeDownloadCount={runtime.activeDownloadCount}
      isRefreshing={runtime.isRefreshing}
      settingsOpen={runtime.settingsOpen}
      downloadPanelOpen={runtime.downloadPanelOpen}
      onRefresh={runtime.handleRefresh}
      onOpenDownloads={runtime.handleToggleDownloads}
      onOpenSettings={runtime.handleToggleSettings}
    />

    {#if runtime.currentView === 'home'}
      <HomeView {runtime} />
    {:else if runtime.currentView === 'tagEditor'}
      <TagEditorView {runtime} />
    {:else if runtime.currentView === 'collection'}
      <CollectionDetailPanel
        collection={runtime.collectionController.selectedCollection}
        isLoading={runtime.collectionController.isDetailLoading}
        reducedMotion={runtime.prefersReducedMotion}
        currentSongCid={runtime.currentSong?.cid ?? null}
        isPlaybackActive={runtime.isPlaying || runtime.isPaused}
        isPlaybackPaused={runtime.isPaused}
        onEdit={runtime.collectionController.openEditDialog}
        onDelete={runtime.collectionController.handleDelete}
        onExport={runtime.collectionController.handleExport}
        onRemoveSongs={runtime.collectionController.handleRemoveSongs}
        onReorderSongs={runtime.collectionController.handleReorderSongs}
        onPlaySong={runtime.handlePlayCollectionSong}
        onTogglePlay={runtime.isPlaying
          ? runtime.playerController.pause
          : runtime.playerController.resume}
        onDownloadSong={runtime.downloadController.handleSongDownload}
        getSongDownloadState={runtime.downloadController.getSongDownloadState}
        isSongDownloadInteractionBlocked={runtime.downloadController
          .isSongDownloadInteractionBlocked}
        collections={runtime.collectionController.collections}
        onAddToCollection={(colId, songCid) =>
          runtime.collectionController.handleAddSongs(colId, [songCid])}
      />
    {:else if runtime.currentView === 'overview'}
      <AlbumOverview
        albums={runtime.albums}
        selectedAlbumCid={runtime.selectedAlbumCid}
        reducedMotion={runtime.prefersReducedMotion}
        searchQuery={runtime.librarySearchQuery}
        searchLoading={runtime.librarySearchLoading}
        searchResponse={runtime.librarySearchResponse}
        onSelectAlbum={runtime.handleSelectAlbum}
        onSelectSearchResult={runtime.handleSelectSearchResult}
      />
    {:else}
      <LibraryView {runtime} />
    {/if}

    <PlayerFlyoutStack
      song={runtime.currentSong}
      isPlaying={runtime.isPlaying}
      isPaused={runtime.isPaused}
      hasPrevious={runtime.playerHasPrevious}
      hasNext={runtime.playerHasNext}
      progress={runtime.progress}
      duration={runtime.duration}
      isLoading={runtime.isLoading}
      reducedMotion={runtime.prefersReducedMotion}
      isShuffled={runtime.shuffleEnabled}
      repeatMode={runtime.repeatMode}
      lyricsOpen={runtime.lyricsOpen}
      playlistOpen={runtime.playlistOpen}
      lyricsLoading={runtime.lyricsLoading}
      lyricsError={runtime.lyricsError}
      lyricsLines={runtime.lyricsLines}
      lyricsUnavailable={runtime.lyricsUnavailable}
      activeLyricIndex={runtime.activeLyricIndex}
      playbackOrder={runtime.playbackOrder}
      downloadState={runtime.currentSongDownloadState}
      downloadDisabled={runtime.currentSongDownloadDisabled}
      onPrevious={runtime.playerController.playPrevious}
      onTogglePlay={runtime.isPlaying
        ? runtime.playerController.pause
        : runtime.playerController.resume}
      onSeek={runtime.playerController.seek}
      onNext={runtime.playerController.playNext}
      onShuffleChange={runtime.playerController.toggleShuffle}
      onRepeatModeChange={runtime.playerController.toggleRepeat}
      onToggleLyrics={runtime.playerController.toggleLyrics}
      onTogglePlaylist={runtime.playerController.togglePlaylist}
      onToggleFullscreen={runtime.playerController.toggleFullscreen}
      onDownload={runtime.handleCurrentSongDownload}
      onPlayQueueEntry={runtime.playerController.playQueueEntry}
    />

    {#if runtime.fullscreenOpen && runtime.currentSong}
      <FullscreenPlayer
        song={runtime.currentSong}
        isPlaying={runtime.isPlaying}
        isPaused={runtime.isPaused}
        isLoading={runtime.isLoading}
        hasPrevious={runtime.playerHasPrevious}
        hasNext={runtime.playerHasNext}
        progress={runtime.progress}
        duration={runtime.duration}
        isShuffled={runtime.shuffleEnabled}
        repeatMode={runtime.repeatMode}
        lyricsLoading={runtime.lyricsLoading}
        lyricsError={runtime.lyricsError}
        lyricsLines={runtime.lyricsLines}
        activeLyricIndex={runtime.activeLyricIndex}
        reducedMotion={runtime.prefersReducedMotion}
        onPrevious={runtime.playerController.playPrevious}
        onTogglePlay={runtime.isPlaying
          ? runtime.playerController.pause
          : runtime.playerController.resume}
        onSeek={runtime.playerController.seek}
        onNext={runtime.playerController.playNext}
        onShuffleChange={runtime.playerController.toggleShuffle}
        onRepeatModeChange={runtime.playerController.toggleRepeat}
        onDownload={runtime.handleCurrentSongDownload}
        downloadState={runtime.currentSongDownloadState}
        downloadDisabled={runtime.currentSongDownloadDisabled}
        onClose={runtime.playerController.toggleFullscreen}
      />
    {/if}

    <AppSideSheets
      SettingsSheetView={runtime.SettingsSheetView}
      DownloadTasksSheetView={runtime.DownloadTasksSheetView}
      bind:settingsOpen={runtime.shellStore.settingsOpen}
      bind:downloadPanelOpen={runtime.shellStore.downloadPanelOpen}
      bind:format={runtime.settingsState.format}
      bind:outputDir={runtime.settingsState.outputDir}
      bind:downloadLyrics={runtime.settingsState.downloadLyrics}
      bind:notifyOnDownloadComplete={
        runtime.settingsState.notifyOnDownloadComplete
      }
      bind:notifyOnPlaybackChange={runtime.settingsState.notifyOnPlaybackChange}
      bind:logLevel={runtime.settingsState.logLevel}
      bind:locale={runtime.settingsState.locale}
      settingsLogRefreshToken={runtime.settingsState.settingsLogRefreshToken}
      notifyInfo={runtime.notifyInfo}
      notifyError={runtime.notifyError}
      onOutputDirChange={runtime.handleOutputDirChange}
      jobs={runtime.filteredDownloadJobs}
      hasDownloadHistory={runtime.hasDownloadHistory}
      bind:searchQuery={runtime.downloadController.searchQuery}
      bind:scopeFilter={runtime.downloadController.scopeFilter}
      bind:statusFilter={runtime.downloadController.statusFilter}
      bind:kindFilter={runtime.downloadController.kindFilter}
      canClearDownloadHistory={runtime.downloadController
        .canClearDownloadHistory}
      getJobProgress={runtime.downloadController.getJobProgress}
      getJobProgressText={runtime.downloadController.getJobProgressText}
      getJobStatusLabel={runtime.downloadController.getJobStatusLabel}
      getJobKindLabel={runtime.downloadController.getJobKindLabel}
      getJobSummaryLabel={runtime.downloadController.getJobSummaryLabel}
      getJobDisplayTitle={runtime.downloadController.getJobDisplayTitle}
      getJobErrorSummary={runtime.downloadController.getJobErrorSummary}
      isJobActive={runtime.downloadController.isJobActive}
      canCancelTask={runtime.downloadController.canCancelTask}
      canRetryTask={runtime.downloadController.canRetryTask}
      getTaskErrorLabel={runtime.downloadController.getTaskErrorLabel}
      getTaskStatusLabel={runtime.downloadController.getTaskStatusLabel}
      onClearDownloadHistory={runtime.downloadController
        .handleClearDownloadHistory}
      onCancelDownloadJob={runtime.downloadController.handleCancelDownloadJob}
      onRetryDownloadJob={runtime.downloadController.handleRetryDownloadJob}
      onCancelDownloadTask={runtime.downloadController.handleCancelDownloadTask}
      onRetryDownloadTask={runtime.downloadController.handleRetryDownloadTask}
    />
  </section>
</div>

<CollectionFormDialog
  bind:open={runtime.collectionController.formDialogOpen}
  mode={runtime.collectionController.formDialogMode}
  initialName={runtime.collectionController.selectedCollection?.name ?? ''}
  initialDescription={runtime.collectionController.selectedCollection
    ?.description ?? ''}
  onSubmit={(name, description) => {
    if (runtime.collectionController.formDialogMode === 'create') {
      return runtime.collectionController.handleCreate(name, description);
    }
    const id = runtime.collectionController.selectedCollectionId;
    if (id) {
      return runtime.collectionController.handleUpdate(id, name, description);
    }
  }}
  onClose={runtime.collectionController.closeFormDialog}
/>
