use crate::screen_recorder;
use crate::types::{CaptureSource, CaptureSourceType, MonitorInfo, WindowInfo};
use serde::Serialize;
use thiserror::Error;
use windows_capture::monitor::Monitor;
use windows_capture::WindowsCaptureGraphicsCaptureItem;

pub struct CaptureSourceManager;

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

impl CaptureSourceManager {
    pub fn get_monitors() -> Result<Vec<MonitorInfo>, CaptureSourceError> {
        let monitors = Monitor::enumerate().map_err(CaptureSourceError::ListMonitors)?;

        let mut monitor_info = Vec::new();
        for (id, monitor) in monitors.iter().enumerate() {
            let hmonitor = monitor.as_raw_hmonitor() as isize;

            let name = match monitor.name() {
                Ok(n) if !n.trim().is_empty() => n,
                _ => continue,
            };

            let width = match monitor.width() {
                Ok(w) => w,
                _ => continue,
            };
            let height = match monitor.height() {
                Ok(h) => h,
                _ => continue,
            };

            monitor_info.push(MonitorInfo {
                id,
                hmonitor,
                name,
                width,
                height,
            });
        }

        Ok(monitor_info)
    }

    pub fn get_windows() -> Result<Vec<WindowInfo>, CaptureSourceError> {
        let windows = screen_recorder::get_available_windows()
            .map_err(|e| CaptureSourceError::ListWindows(e))?;

        let mut window_info = Vec::new();
        for (id, window) in windows.iter().enumerate() {
            if !window.is_valid() {
                continue;
            }

            let title = match window.title() {
                Ok(t) if !t.trim().is_empty() => t,
                _ => continue,
            };

            let hwnd = window.as_raw_hwnd() as isize;

            let capture_item = match WindowsCaptureGraphicsCaptureItem::try_from(window.clone()) {
                Ok(item) => item,
                Err(_) => continue,
            };

            let size = match capture_item.Size() {
                Ok(s) => s,
                Err(_) => continue,
            };

            let width = size.Width as u32;
            let height = size.Height as u32;

            window_info.push(WindowInfo {
                id,
                hwnd,
                title,
                width,
                height,
            });
        }

        Ok(window_info)
    }

    pub fn get_all_capture_sources() -> Result<Vec<CaptureSource>, CaptureSourceError> {
        let mut sources = Vec::new();

        // Add monitors
        let monitors = Self::get_monitors()?;
        println!("Found {} monitors", monitors.len());

        sources.extend(monitors.into_iter().map(|monitor| CaptureSource {
            name: monitor.name,
            width: monitor.width,
            height: monitor.height,
            source_type: CaptureSourceType::Monitor,
            handle: monitor.hmonitor,
        }));

        let windows = Self::get_windows()?;
        println!("Found {} windows", windows.len());

        sources.extend(windows.into_iter().map(|window| CaptureSource {
            name: window.title,
            width: window.width,
            height: window.height,
            source_type: CaptureSourceType::Window,
            handle: window.hwnd,
        }));

        println!("Total sources found: {}", sources.len());
        Ok(sources)
    }
}
