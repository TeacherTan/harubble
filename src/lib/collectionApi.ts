import { invoke } from '@tauri-apps/api/core';
import type { Collection, CollectionSummary } from './types';

export async function listCollections(): Promise<CollectionSummary[]> {
  return invoke('list_collections');
}

export async function getCollection(id: string): Promise<Collection> {
  return invoke('get_collection', { id });
}

export async function createCollection(
  name: string,
  description: string,
  coverPath?: string | null
): Promise<Collection> {
  return invoke('create_collection', {
    name,
    description,
    coverPath: coverPath ?? null,
  });
}

export async function updateCollection(
  id: string,
  name?: string | null,
  description?: string | null,
  coverPath?: string | null | undefined
): Promise<Collection> {
  return invoke('update_collection', {
    id,
    name: name ?? null,
    description: description ?? null,
    coverPath: coverPath === undefined ? null : coverPath,
  });
}

export async function deleteCollection(id: string): Promise<void> {
  return invoke('delete_collection', { id });
}

export async function addSongsToCollection(
  id: string,
  songIds: string[]
): Promise<void> {
  return invoke('add_songs_to_collection', { id, songIds });
}

export async function removeSongsFromCollection(
  id: string,
  songIds: string[]
): Promise<void> {
  return invoke('remove_songs_from_collection', { id, songIds });
}

export async function reorderCollectionSongs(
  id: string,
  songIds: string[]
): Promise<void> {
  return invoke('reorder_collection_songs', { id, songIds });
}

export async function exportCollection(id: string): Promise<string> {
  return invoke('export_collection', { id });
}

export async function importCollection(json: string): Promise<Collection> {
  return invoke('import_collection', { json });
}
