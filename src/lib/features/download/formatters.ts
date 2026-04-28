import type { DownloadTaskSnapshot } from '$lib/types';
import * as m from '$lib/paraglide/messages.js';

export function buildSelectionKey(songCids: string[]): string {
  return [...songCids].sort().join(',');
}

export function formatByteSize(bytes: number | null | undefined): string {
  if (
    bytes === null ||
    bytes === undefined ||
    !Number.isFinite(bytes) ||
    bytes < 0
  ) {
    return m.download_fmt_unknown_size();
  }

  if (bytes < 1024) return `${bytes} B`;
  const units = ['KB', 'MB', 'GB', 'TB'];
  let value = bytes;
  let unitIndex = -1;

  while (value >= 1024 && unitIndex < units.length - 1) {
    value /= 1024;
    unitIndex += 1;
  }

  const precision = value >= 100 ? 0 : value >= 10 ? 1 : 2;
  return `${value.toFixed(precision)} ${units[unitIndex]}`;
}

export function formatSpeed(bytesPerSec: number): string {
  if (!Number.isFinite(bytesPerSec) || bytesPerSec < 0) {
    return m.download_fmt_unknown_speed();
  }

  if (bytesPerSec < 1024) return `${bytesPerSec.toFixed(0)} B/s`;
  const units = ['KB/s', 'MB/s', 'GB/s'];
  let value = bytesPerSec;
  let unitIndex = -1;

  while (value >= 1024 && unitIndex < units.length - 1) {
    value /= 1024;
    unitIndex += 1;
  }

  const precision = value >= 100 ? 0 : value >= 10 ? 1 : 2;
  return `${value.toFixed(precision)} ${units[unitIndex]}`;
}

export function getTaskStatusLabel(task: DownloadTaskSnapshot): string {
  switch (task.status) {
    case 'queued':
      return m.download_fmt_task_queued();
    case 'preparing':
      return m.download_fmt_task_preparing();
    case 'downloading':
      return m.download_fmt_task_downloading();
    case 'writing':
      return m.download_fmt_task_writing();
    case 'completed':
      return m.download_fmt_task_completed();
    case 'failed':
      return m.download_fmt_task_failed();
    case 'cancelled':
      return m.download_fmt_task_cancelled();
    default:
      return task.status;
  }
}
