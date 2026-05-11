import { invoke } from '@tauri-apps/api/core';
import type { LogFileStatus, LogViewerPage, LogViewerQuery } from './types';

export { selectDirectory, sendTestNotification } from './api';

export async function clearAudioCache(): Promise<number> {
  return invoke<number>('clear_audio_cache');
}

export async function listLogRecords(
  query: LogViewerQuery
): Promise<LogViewerPage> {
  return invoke<LogViewerPage>('list_log_records', { query });
}

export async function getLogFileStatus(): Promise<LogFileStatus> {
  return invoke<LogFileStatus>('get_log_file_status');
}
