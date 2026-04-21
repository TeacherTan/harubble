use crate::audio::sanitize_filename;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

const AUDIO_EXTENSIONS: [&str; 3] = ["flac", "wav", "mp3"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LocalTrackDownloadStatus {
    Missing,
    Detected,
    Verified,
    Mismatch,
    Partial,
    Unverifiable,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LocalInventoryStatus {
    Idle,
    Scanning,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VerificationMode {
    None,
    WhenAvailable,
    Strict,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LocalTrackEvidenceMatchRule {
    RootRelativePath,
    AlbumRelativePath,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LocalAudioFileVerificationState {
    Unchecked,
    Verified,
    Mismatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalAudioFileEvidence {
    /// 相对于当前 active outputDir 的规范化相对路径。
    pub relative_path: String,
    /// 扫描时读取到的文件大小。
    pub file_size: u64,
    /// 扫描时读取到的 mtime（Unix ms），用于后续校验链扩展。
    pub modified_at_ms: Option<u64>,
    /// 预留给 checksum / provenance 链的候选摘要字段。
    pub candidate_checksum: Option<String>,
    /// 该文件是否位于专辑子目录下。
    pub is_in_album_directory: bool,
    /// 基于直接 checksum 或 provenance 解析出的校验结论。
    pub verification_state: LocalAudioFileVerificationState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchedTrackEvidence {
    pub relative_path: String,
    pub file_size: u64,
    pub modified_at_ms: Option<u64>,
    pub candidate_checksum: Option<String>,
    pub is_in_album_directory: bool,
    pub match_rule: LocalTrackEvidenceMatchRule,
    pub verification_state: LocalAudioFileVerificationState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackDownloadBadge {
    pub is_downloaded: bool,
    pub download_status: LocalTrackDownloadStatus,
    pub inventory_version: String,
}

impl Default for TrackDownloadBadge {
    fn default() -> Self {
        Self {
            is_downloaded: false,
            download_status: LocalTrackDownloadStatus::Missing,
            inventory_version: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumDownloadBadge {
    /// 是否可将当前专辑视为“本地已有内容”。
    pub is_downloaded: bool,
    /// 当前列表级专辑提示状态；现阶段以保守提示语义为主。
    pub download_status: LocalTrackDownloadStatus,
    /// 用于前端缓存失效的盘点版本。
    pub inventory_version: String,
}

impl Default for AlbumDownloadBadge {
    fn default() -> Self {
        Self {
            is_downloaded: false,
            download_status: LocalTrackDownloadStatus::Missing,
            inventory_version: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalInventorySnapshot {
    pub root_output_dir: String,
    pub status: LocalInventoryStatus,
    pub inventory_version: String,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
    pub scanned_file_count: usize,
    pub matched_track_count: usize,
    pub verified_track_count: usize,
    pub last_error: Option<String>,
}

impl Default for LocalInventorySnapshot {
    fn default() -> Self {
        Self {
            root_output_dir: String::new(),
            status: LocalInventoryStatus::Idle,
            inventory_version: String::new(),
            started_at: None,
            finished_at: None,
            scanned_file_count: 0,
            matched_track_count: 0,
            verified_track_count: 0,
            last_error: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalInventoryScanProgressEvent {
    pub root_output_dir: String,
    pub inventory_version: String,
    pub files_scanned: usize,
    pub matched_track_count: usize,
    pub verified_track_count: usize,
    pub current_path: Option<String>,
}

pub fn is_downloaded_status(status: LocalTrackDownloadStatus) -> bool {
    matches!(
        status,
        LocalTrackDownloadStatus::Detected
            | LocalTrackDownloadStatus::Verified
            | LocalTrackDownloadStatus::Partial
            | LocalTrackDownloadStatus::Unverifiable
    )
}

pub fn missing_track_badge(inventory_version: impl Into<String>) -> TrackDownloadBadge {
    badge_for_status(LocalTrackDownloadStatus::Missing, inventory_version)
}

pub fn missing_album_badge(inventory_version: impl Into<String>) -> AlbumDownloadBadge {
    album_badge_for_status(LocalTrackDownloadStatus::Missing, inventory_version)
}

pub fn badge_for_detected_file(
    verification_mode: VerificationMode,
    inventory_version: impl Into<String>,
) -> TrackDownloadBadge {
    let status = match verification_mode {
        VerificationMode::Strict => LocalTrackDownloadStatus::Unverifiable,
        VerificationMode::None | VerificationMode::WhenAvailable => {
            LocalTrackDownloadStatus::Detected
        }
    };
    badge_for_status(status, inventory_version)
}

pub fn badge_for_status(
    status: LocalTrackDownloadStatus,
    inventory_version: impl Into<String>,
) -> TrackDownloadBadge {
    TrackDownloadBadge {
        is_downloaded: is_downloaded_status(status),
        download_status: status,
        inventory_version: inventory_version.into(),
    }
}

pub fn album_badge_for_status(
    status: LocalTrackDownloadStatus,
    inventory_version: impl Into<String>,
) -> AlbumDownloadBadge {
    AlbumDownloadBadge {
        is_downloaded: is_downloaded_status(status),
        download_status: status,
        inventory_version: inventory_version.into(),
    }
}

pub fn aggregate_album_download_badge(
    track_badges: &[TrackDownloadBadge],
    inventory_version: impl Into<String>,
) -> AlbumDownloadBadge {
    let inventory_version = inventory_version.into();

    if track_badges.is_empty() {
        return missing_album_badge(inventory_version);
    }

    let statuses = track_badges
        .iter()
        .map(|badge| badge.download_status)
        .collect::<Vec<_>>();

    if statuses
        .iter()
        .all(|status| *status == LocalTrackDownloadStatus::Missing)
    {
        return missing_album_badge(inventory_version);
    }

    if statuses
        .iter()
        .any(|status| *status == LocalTrackDownloadStatus::Mismatch)
    {
        return album_badge_for_status(LocalTrackDownloadStatus::Mismatch, inventory_version);
    }

    if statuses
        .iter()
        .any(|status| *status == LocalTrackDownloadStatus::Partial)
    {
        return album_badge_for_status(LocalTrackDownloadStatus::Partial, inventory_version);
    }

    let downloaded_count = statuses
        .iter()
        .filter(|status| is_downloaded_status(**status))
        .count();

    if downloaded_count == 0 {
        if statuses
            .iter()
            .any(|status| *status == LocalTrackDownloadStatus::Unknown)
        {
            return album_badge_for_status(LocalTrackDownloadStatus::Unknown, inventory_version);
        }
        return missing_album_badge(inventory_version);
    }

    if downloaded_count < statuses.len() {
        return album_badge_for_status(LocalTrackDownloadStatus::Partial, inventory_version);
    }

    if statuses
        .iter()
        .all(|status| *status == LocalTrackDownloadStatus::Verified)
    {
        return album_badge_for_status(LocalTrackDownloadStatus::Verified, inventory_version);
    }

    if statuses
        .iter()
        .any(|status| *status == LocalTrackDownloadStatus::Unverifiable)
    {
        return album_badge_for_status(LocalTrackDownloadStatus::Unverifiable, inventory_version);
    }

    if statuses
        .iter()
        .any(|status| *status == LocalTrackDownloadStatus::Detected)
    {
        return album_badge_for_status(LocalTrackDownloadStatus::Detected, inventory_version);
    }

    album_badge_for_status(LocalTrackDownloadStatus::Unknown, inventory_version)
}

pub fn album_badge_from_evidence(
    audio_files: &[LocalAudioFileEvidence],
    album_name: &str,
    inventory_version: impl Into<String>,
) -> AlbumDownloadBadge {
    let safe_album_name = sanitize_filename(album_name);
    let album_prefix = format!("{safe_album_name}/");

    if audio_files
        .iter()
        .any(|evidence| evidence.relative_path.starts_with(&album_prefix))
    {
        return album_badge_for_status(LocalTrackDownloadStatus::Partial, inventory_version);
    }

    missing_album_badge(inventory_version)
}

pub fn track_badge_from_matches(
    matches: &[MatchedTrackEvidence],
    verification_mode: VerificationMode,
    inventory_version: impl Into<String>,
) -> TrackDownloadBadge {
    if matches.is_empty() {
        return missing_track_badge(inventory_version);
    }

    if matches.len() > 1 {
        return badge_for_status(LocalTrackDownloadStatus::Partial, inventory_version);
    }

    match matches[0].verification_state {
        LocalAudioFileVerificationState::Verified => {
            badge_for_status(LocalTrackDownloadStatus::Verified, inventory_version)
        }
        LocalAudioFileVerificationState::Mismatch => {
            badge_for_status(LocalTrackDownloadStatus::Mismatch, inventory_version)
        }
        LocalAudioFileVerificationState::Unchecked => {
            badge_for_detected_file(verification_mode, inventory_version)
        }
    }
}

pub fn matched_track_evidence(
    audio_files: &[LocalAudioFileEvidence],
    album_name: &str,
    song_name: &str,
) -> Vec<MatchedTrackEvidence> {
    let safe_song_name = sanitize_filename(song_name);
    let safe_album_name = sanitize_filename(album_name);
    let root_candidates = AUDIO_EXTENSIONS
        .iter()
        .map(|extension| format!("{safe_song_name}.{extension}"))
        .collect::<HashSet<_>>();
    let album_candidates = AUDIO_EXTENSIONS
        .iter()
        .map(|extension| format!("{safe_album_name}/{safe_song_name}.{extension}"))
        .collect::<HashSet<_>>();

    audio_files
        .iter()
        .filter_map(|evidence| {
            let match_rule = if root_candidates.contains(&evidence.relative_path) {
                Some(LocalTrackEvidenceMatchRule::RootRelativePath)
            } else if album_candidates.contains(&evidence.relative_path) {
                Some(LocalTrackEvidenceMatchRule::AlbumRelativePath)
            } else {
                None
            }?;

            Some(MatchedTrackEvidence {
                relative_path: evidence.relative_path.clone(),
                file_size: evidence.file_size,
                modified_at_ms: evidence.modified_at_ms,
                candidate_checksum: evidence.candidate_checksum.clone(),
                is_in_album_directory: evidence.is_in_album_directory,
                match_rule,
                verification_state: evidence.verification_state,
            })
        })
        .collect()
}

pub fn candidate_relative_paths(album_name: &str, song_name: &str) -> Vec<String> {
    let safe_song_name = sanitize_filename(song_name);
    let safe_album_name = sanitize_filename(album_name);
    let mut candidates = Vec::with_capacity(AUDIO_EXTENSIONS.len() * 2);

    for extension in AUDIO_EXTENSIONS {
        candidates.push(format!("{safe_song_name}.{extension}"));
        candidates.push(format!("{safe_album_name}/{safe_song_name}.{extension}"));
    }

    candidates
}

pub fn has_detected_track(
    relative_audio_paths: &HashSet<String>,
    album_name: &str,
    song_name: &str,
) -> bool {
    candidate_relative_paths(album_name, song_name)
        .into_iter()
        .any(|candidate| relative_audio_paths.contains(&candidate))
}

#[cfg(test)]
mod tests {
    use super::{
        aggregate_album_download_badge, album_badge_from_evidence, badge_for_detected_file,
        badge_for_status, candidate_relative_paths, has_detected_track, is_downloaded_status,
        matched_track_evidence, track_badge_from_matches, AlbumDownloadBadge,
        LocalAudioFileEvidence, LocalAudioFileVerificationState, LocalTrackDownloadStatus,
        VerificationMode,
    };
    use std::collections::HashSet;

    #[test]
    fn builds_root_and_album_candidates_for_all_audio_extensions() {
        let candidates = candidate_relative_paths("A/B:C?D", "Track/01");

        assert!(candidates.contains(&"Track_01.flac".to_string()));
        assert!(candidates.contains(&"Track_01.wav".to_string()));
        assert!(candidates.contains(&"Track_01.mp3".to_string()));
        assert!(candidates.contains(&"A_B_C_D/Track_01.flac".to_string()));
    }

    #[test]
    fn detects_track_from_single_song_or_album_layout() {
        let mut files = HashSet::new();
        files.insert("Album/Track.flac".to_string());
        files.insert("Other.wav".to_string());

        assert!(has_detected_track(&files, "Album", "Track"));
        assert!(has_detected_track(&files, "Anything", "Other"));
        assert!(!has_detected_track(&files, "Album", "Missing"));
    }

    #[test]
    fn maps_detected_files_to_unverifiable_in_strict_mode() {
        let strict_badge = badge_for_detected_file(VerificationMode::Strict, "v1");
        let relaxed_badge = badge_for_detected_file(VerificationMode::WhenAvailable, "v1");

        assert_eq!(
            strict_badge.download_status,
            LocalTrackDownloadStatus::Unverifiable
        );
        assert_eq!(
            relaxed_badge.download_status,
            LocalTrackDownloadStatus::Detected
        );
        assert!(strict_badge.is_downloaded);
    }

    #[test]
    fn downloaded_status_mapping_matches_contract() {
        assert!(is_downloaded_status(LocalTrackDownloadStatus::Detected));
        assert!(is_downloaded_status(LocalTrackDownloadStatus::Verified));
        assert!(is_downloaded_status(LocalTrackDownloadStatus::Partial));
        assert!(is_downloaded_status(LocalTrackDownloadStatus::Unverifiable));
        assert!(!is_downloaded_status(LocalTrackDownloadStatus::Missing));
        assert!(!is_downloaded_status(LocalTrackDownloadStatus::Mismatch));
        assert!(!is_downloaded_status(LocalTrackDownloadStatus::Unknown));
    }

    #[test]
    fn collects_matched_track_evidence_with_match_rules() {
        let audio_files = vec![
            LocalAudioFileEvidence {
                relative_path: "Track.flac".to_string(),
                file_size: 12,
                modified_at_ms: Some(10),
                candidate_checksum: None,
                is_in_album_directory: false,
                verification_state: LocalAudioFileVerificationState::Unchecked,
            },
            LocalAudioFileEvidence {
                relative_path: "Album/Track.wav".to_string(),
                file_size: 24,
                modified_at_ms: Some(20),
                candidate_checksum: None,
                is_in_album_directory: true,
                verification_state: LocalAudioFileVerificationState::Unchecked,
            },
            LocalAudioFileEvidence {
                relative_path: "Album/Other.wav".to_string(),
                file_size: 24,
                modified_at_ms: Some(20),
                candidate_checksum: None,
                is_in_album_directory: true,
                verification_state: LocalAudioFileVerificationState::Unchecked,
            },
        ];

        let matches = matched_track_evidence(&audio_files, "Album", "Track");

        assert_eq!(matches.len(), 2);
        assert!(matches
            .iter()
            .any(|item| item.relative_path == "Track.flac"));
        assert!(matches
            .iter()
            .any(|item| item.relative_path == "Album/Track.wav"));
    }

    #[test]
    fn marks_multiple_matches_as_partial() {
        let matches = vec![
            matched_track_evidence(
                &[LocalAudioFileEvidence {
                    relative_path: "Track.flac".to_string(),
                    file_size: 12,
                    modified_at_ms: Some(10),
                    candidate_checksum: None,
                    is_in_album_directory: false,
                    verification_state: LocalAudioFileVerificationState::Unchecked,
                }],
                "Album",
                "Track",
            )
            .pop()
            .expect("root match"),
            matched_track_evidence(
                &[LocalAudioFileEvidence {
                    relative_path: "Album/Track.wav".to_string(),
                    file_size: 24,
                    modified_at_ms: Some(20),
                    candidate_checksum: None,
                    is_in_album_directory: true,
                    verification_state: LocalAudioFileVerificationState::Unchecked,
                }],
                "Album",
                "Track",
            )
            .pop()
            .expect("album match"),
        ];

        let badge = track_badge_from_matches(&matches, VerificationMode::WhenAvailable, "v1");
        assert_eq!(badge.download_status, LocalTrackDownloadStatus::Partial);
        assert!(badge.is_downloaded);
    }

    #[test]
    fn album_badge_is_missing_when_aggregate_input_is_empty() {
        let badge = aggregate_album_download_badge(&[], "v1");

        assert_eq!(badge.download_status, LocalTrackDownloadStatus::Missing);
        assert!(!badge.is_downloaded);
    }

    #[test]
    fn album_badge_is_verified_when_all_tracks_are_verified() {
        let badge = aggregate_album_download_badge(
            &[
                badge_for_status(LocalTrackDownloadStatus::Verified, "v1"),
                badge_for_status(LocalTrackDownloadStatus::Verified, "v1"),
            ],
            "v1",
        );

        assert_eq!(badge.download_status, LocalTrackDownloadStatus::Verified);
        assert!(badge.is_downloaded);
    }

    #[test]
    fn album_badge_is_mismatch_when_any_track_is_mismatch() {
        let badge = aggregate_album_download_badge(
            &[
                badge_for_status(LocalTrackDownloadStatus::Mismatch, "v1"),
                badge_for_status(LocalTrackDownloadStatus::Detected, "v1"),
            ],
            "v1",
        );

        assert_eq!(badge.download_status, LocalTrackDownloadStatus::Mismatch);
        assert!(!badge.is_downloaded);
    }

    #[test]
    fn album_badge_is_partial_when_only_some_tracks_are_downloaded() {
        let badge = aggregate_album_download_badge(
            &[
                badge_for_status(LocalTrackDownloadStatus::Detected, "v1"),
                badge_for_status(LocalTrackDownloadStatus::Missing, "v1"),
            ],
            "v1",
        );

        assert_eq!(badge.download_status, LocalTrackDownloadStatus::Partial);
        assert!(badge.is_downloaded);
    }

    #[test]
    fn album_badge_is_unverifiable_when_all_tracks_are_downloaded_but_not_verifiable() {
        let badge = aggregate_album_download_badge(
            &[
                badge_for_status(LocalTrackDownloadStatus::Unverifiable, "v1"),
                badge_for_status(LocalTrackDownloadStatus::Detected, "v1"),
            ],
            "v1",
        );

        assert_eq!(
            badge.download_status,
            LocalTrackDownloadStatus::Unverifiable
        );
        assert!(badge.is_downloaded);
    }

    #[test]
    fn album_badge_is_missing_when_all_tracks_are_missing() {
        let badge = aggregate_album_download_badge(
            &[
                badge_for_status(LocalTrackDownloadStatus::Missing, "v1"),
                badge_for_status(LocalTrackDownloadStatus::Missing, "v1"),
            ],
            "v1",
        );

        assert_eq!(badge.download_status, LocalTrackDownloadStatus::Missing);
        assert!(!badge.is_downloaded);
    }

    #[test]
    fn album_list_badge_uses_album_directory_evidence_conservatively() {
        let badge: AlbumDownloadBadge = album_badge_from_evidence(
            &[LocalAudioFileEvidence {
                relative_path: "Album/Track.flac".to_string(),
                file_size: 12,
                modified_at_ms: Some(10),
                candidate_checksum: None,
                is_in_album_directory: true,
                verification_state: LocalAudioFileVerificationState::Unchecked,
            }],
            "Album",
            "v1",
        );

        assert_eq!(badge.download_status, LocalTrackDownloadStatus::Partial);
        assert!(badge.is_downloaded);
    }
}
