//! Tag 编辑器相关的 Tauri command。
//!
//! 提供双层存储（remote + local overlay）的 CRUD、合并计算、三路合并与冲突解决能力。

use crate::app_state::AppState;
use crate::tag_editor::{ConflictResolution, EntityType, MergeConflict, MergeResult};
use crate::tag_registry::{LocalizedValue, TagRegistry};
use tauri::State;

/// 返回合并后的完整 tag 注册表（remote + local overlay 合并计算结果）。
#[tauri::command]
pub fn get_tag_editor_merged(state: State<'_, AppState>) -> Result<TagRegistry, String> {
    Ok(state.tag_editor.compute_merged())
}

/// 返回本地 overlay 层的原始内容。
#[tauri::command]
pub fn get_tag_editor_local_overlay(state: State<'_, AppState>) -> Result<TagRegistry, String> {
    Ok(state.tag_editor.local_registry().clone())
}

/// 设置指定实体在指定维度上的 tag 值（写入本地 overlay）。
#[tauri::command]
pub fn set_tag_editor_entity_tag(
    state: State<'_, AppState>,
    entity_type: EntityType,
    cid: String,
    dimension_key: String,
    values: Vec<LocalizedValue>,
) -> Result<(), String> {
    state
        .tag_editor
        .set_entity_tag(entity_type, &cid, &dimension_key, values)
        .map_err(|e| e.to_string())
}

/// 删除指定实体在指定维度上的本地 overlay tag。
#[tauri::command]
pub fn remove_tag_editor_entity_tag(
    state: State<'_, AppState>,
    entity_type: EntityType,
    cid: String,
    dimension_key: String,
) -> Result<(), String> {
    state
        .tag_editor
        .remove_entity_tag(entity_type, &cid, &dimension_key)
        .map_err(|e| e.to_string())
}

/// 新增本地维度定义。
#[tauri::command]
pub fn add_tag_editor_dimension(
    state: State<'_, AppState>,
    key: String,
    label_zh: String,
    label_en: String,
) -> Result<(), String> {
    state
        .tag_editor
        .add_local_dimension(&key, &label_zh, &label_en)
        .map_err(|e| e.to_string())
}

/// 删除本地维度定义及其关联的所有 tag 数据。
#[tauri::command]
pub fn remove_tag_editor_dimension(state: State<'_, AppState>, key: String) -> Result<(), String> {
    state
        .tag_editor
        .remove_local_dimension(&key)
        .map_err(|e| e.to_string())
}

/// 接收新的远端快照并执行三路合并，返回合并结果（含冲突列表）。
#[tauri::command]
pub fn apply_tag_editor_remote_update(
    state: State<'_, AppState>,
    new_remote: TagRegistry,
) -> Result<MergeResult, String> {
    state
        .tag_editor
        .apply_remote_update(new_remote)
        .map_err(|e| e.to_string())
}

/// 解决单个三路合并冲突。
#[tauri::command]
pub fn resolve_tag_editor_conflict(
    state: State<'_, AppState>,
    entity_type: EntityType,
    cid: String,
    dimension_key: String,
    keep: ConflictResolution,
) -> Result<(), String> {
    state
        .tag_editor
        .resolve_conflict(entity_type, &cid, &dimension_key, keep)
        .map_err(|e| e.to_string())
}
