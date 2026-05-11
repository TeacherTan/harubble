//! 下载规划器。
//!
//! 该模块负责描述下载批次在进入执行器前的规划阶段。当前实现将任务拆分逻辑
//! 保持在 `service.rs` 中，因此这里只保留最小占位类型。

#[allow(dead_code)]
pub struct DownloadPlan;

#[allow(dead_code)]
impl DownloadPlan {
    pub fn is_empty(&self) -> bool {
        true
    }
}
