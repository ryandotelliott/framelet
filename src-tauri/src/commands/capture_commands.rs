use crate::capture::{
    get_all_capture_sources, get_webcams as get_webcams_handler, CaptureSource, Webcam,
};

/// Gets all available capture sources (monitors and windows)
#[tauri::command]
pub fn get_capture_sources() -> Result<Vec<CaptureSource>, String> {
    get_all_capture_sources().map_err(|e| e.to_string())
}

/// Gets all available webcams
#[tauri::command]
pub async fn get_webcams() -> Result<Vec<Webcam>, String> {
    get_webcams_handler().await.map_err(|e| e.to_string())
}
