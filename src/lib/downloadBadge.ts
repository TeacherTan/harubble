import type { LocalTrackDownloadStatus } from './types';
import * as m from '$lib/paraglide/messages.js';

export function shouldShowDownloadBadge(
  status: LocalTrackDownloadStatus
): boolean {
  return status !== 'missing' && status !== 'unknown';
}

export function shouldShowAlbumListDownloadBadge(
  status: LocalTrackDownloadStatus
): boolean {
  return status !== 'partial' && shouldShowDownloadBadge(status);
}

export function getDownloadBadgeLabel(
  status: LocalTrackDownloadStatus
): string {
  switch (status) {
    case 'verified':
      return m.badge_verified();
    case 'mismatch':
      return m.badge_mismatch();
    case 'partial':
      return m.badge_partial();
    case 'unverifiable':
      return m.badge_unverifiable();
    case 'detected':
      return m.badge_detected();
    default:
      return m.badge_missing();
  }
}
