use crate::{capture::CaptureSourceManager, types::CaptureSource};

/// Gets all available capture sources (monitors and windows)
#[tauri::command]
pub async fn get_capture_sources() -> Result<Vec<CaptureSource>, String> {
    CaptureSourceManager::get_all_capture_sources().map_err(|e| e.to_string())
}
