import type {
  TagEditorEntityType,
  TagEditorLocalizedValue,
  TagEditorMergeConflict,
  TagEditorRegistry,
} from '$lib/types';

let merged = $state<TagEditorRegistry | null>(null);
let localOverlay = $state<TagEditorRegistry | null>(null);
let selectedEntityType = $state<TagEditorEntityType>('album');
let selectedCid = $state<string | null>(null);
let conflicts = $state<TagEditorMergeConflict[]>([]);
let loading = $state(false);

function reset() {
  merged = null;
  localOverlay = null;
  selectedEntityType = 'album';
  selectedCid = null;
  conflicts = [];
  loading = false;
}

export const tagEditorStore = {
  get merged() {
    return merged;
  },
  set merged(value: TagEditorRegistry | null) {
    merged = value;
  },
  get localOverlay() {
    return localOverlay;
  },
  set localOverlay(value: TagEditorRegistry | null) {
    localOverlay = value;
  },
  get selectedEntityType() {
    return selectedEntityType;
  },
  set selectedEntityType(value: TagEditorEntityType) {
    selectedEntityType = value;
  },
  get selectedCid() {
    return selectedCid;
  },
  set selectedCid(value: string | null) {
    selectedCid = value;
  },
  get conflicts() {
    return conflicts;
  },
  set conflicts(value: TagEditorMergeConflict[]) {
    conflicts = value;
  },
  get loading() {
    return loading;
  },
  set loading(value: boolean) {
    loading = value;
  },
  get selectedEntityTags(): Record<string, TagEditorLocalizedValue[]> {
    if (!merged || !selectedCid) return {};
    const map =
      selectedEntityType === 'album' ? merged.albums : merged.songs;
    return map[selectedCid]?.tags ?? {};
  },
  reset,
};

if (import.meta.hot) {
  import.meta.hot.dispose(() => {
    reset();
  });
}
