use serde::Serialize;
use thiserror::Error;

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

#[derive(Debug, Error)]
pub enum CaptureSourceError {
    #[error("failed to list monitors: {0}")]
    ListMonitors(#[source] windows_capture::monitor::Error),

    #[error("failed to list windows: {0}")]
    ListWindows(#[source] Box<dyn std::error::Error>),
}

impl Serialize for CaptureSourceError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
