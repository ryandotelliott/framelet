use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CaptureSourceType {
    Monitor,
    Window,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaptureSource {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub source_type: CaptureSourceType,
    pub handle: isize, // HWND or HMONITOR
    pub left: i32,
    pub top: i32,
}

#[derive(Debug, Serialize)]
pub struct MonitorInfo {
    pub id: usize,
    pub hmonitor: isize,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub left: i32,
    pub top: i32,
}

#[derive(Debug, Serialize)]
pub struct WindowInfo {
    pub id: usize,
    pub hwnd: isize,
    pub title: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum CaptureSourceError {
    #[error("failed to list monitors: {0}")]
    ListMonitors(#[source] windows_capture::monitor::Error),

    #[error("failed to list windows: {0}")]
    ListWindows(#[source] Box<dyn std::error::Error>),
}
