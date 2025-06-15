use crate::screen_recorder;
use crate::types::{CaptureSource, CaptureSourceType, MonitorInfo, WindowInfo};
use windows_capture::{window::Window, WindowsCaptureGraphicsCaptureItem};

pub struct CaptureSourceManager;

impl CaptureSourceManager {
    pub fn get_monitors() -> Result<Vec<MonitorInfo>, String> {
        let monitors = screen_recorder::get_available_monitors()
            .map_err(|e| format!("Failed to get monitors: {}", e))?;

        let mut monitor_info = Vec::new();
        for (id, monitor) in monitors.iter().enumerate() {
            let name = monitor
                .name()
                .map_err(|e| format!("Failed to get monitor name: {}", e))?;
            let width = monitor
                .width()
                .map_err(|e| format!("Failed to get monitor width: {}", e))?;
            let height = monitor
                .height()
                .map_err(|e| format!("Failed to get monitor height: {}", e))?;

            monitor_info.push(MonitorInfo {
                id,
                name,
                width,
                height,
            });
        }

        Ok(monitor_info)
    }

    pub fn get_windows() -> Result<Vec<WindowInfo>, String> {
        let windows = screen_recorder::get_available_windows()
            .map_err(|e| format!("Failed to get windows: {}", e))?;

        let mut window_info = Vec::new();
        for (id, window) in windows.iter().enumerate() {
            let title = window
                .title()
                .map_err(|e| format!("Failed to get window title: {}", e))?;

            // Get HWND
            let hwnd = Self::get_window_hwnd(window)
                .map_err(|e| format!("Failed to get window HWND: {}", e))?;

            // Get dimensions by converting to capture item
            let capture_item = WindowsCaptureGraphicsCaptureItem::try_from(window.clone())
                .map_err(|e| format!("Failed to create capture item: {}", e))?;
            let width = capture_item
                .Size()
                .map_err(|e| format!("Failed to get window size: {}", e))?
                .Width as u32;
            let height = capture_item
                .Size()
                .map_err(|e| format!("Failed to get window size: {}", e))?
                .Height as u32;

            window_info.push(WindowInfo {
                id,
                title,
                width,
                height,
                hwnd,
            });
        }

        Ok(window_info)
    }

    pub fn get_all_capture_sources() -> Result<Vec<CaptureSource>, String> {
        println!("Starting get_all_capture_sources");
        let mut sources = Vec::new();
        let mut id_counter = 0;

        // Add monitors
        println!("Getting monitors...");
        let monitors = screen_recorder::get_available_monitors().map_err(|e| {
            let error_msg = format!("Failed to get monitors: {}", e);
            println!("{}", error_msg);
            error_msg
        })?;

        println!("Found {} monitors", monitors.len());

        for monitor in monitors.iter() {
            let name = monitor
                .name()
                .map_err(|e| format!("Failed to get monitor name: {}", e))?;
            let width = monitor
                .width()
                .map_err(|e| format!("Failed to get monitor width: {}", e))?;
            let height = monitor
                .height()
                .map_err(|e| format!("Failed to get monitor height: {}", e))?;

            println!("Adding monitor: {} ({}x{})", name, width, height);
            sources.push(CaptureSource {
                id: id_counter,
                name: format!("🖥️ {}", name),
                width,
                height,
                source_type: CaptureSourceType::Monitor,
            });
            id_counter += 1;
        }

        println!("Getting windows...");
        match screen_recorder::get_available_windows() {
            Ok(windows) => {
                println!("Found {} windows", windows.len());
                for window in windows.iter() {
                    match Self::process_window(window, id_counter) {
                        Ok(source) => {
                            println!("Adding window: {}", source.name);
                            sources.push(source);
                            id_counter += 1;
                        }
                        Err(e) => {
                            println!("Skipping window due to error: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("Warning: Failed to get windows: {}", e);
                // Don't fail completely, just continue without windows
            }
        }

        println!("Total sources found: {}", sources.len());
        Ok(sources)
    }

    fn process_window(window: &Window, id: usize) -> Result<CaptureSource, String> {
        let title = window
            .title()
            .map_err(|e| format!("Failed to get window title: {}", e))?;

        // Get dimensions by converting to capture item
        let capture_item = WindowsCaptureGraphicsCaptureItem::try_from(window.clone())
            .map_err(|e| format!("Failed to create capture item: {}", e))?;
        let width = capture_item
            .Size()
            .map_err(|e| format!("Failed to get window size: {}", e))?
            .Width as u32;
        let height = capture_item
            .Size()
            .map_err(|e| format!("Failed to get window size: {}", e))?
            .Height as u32;

        Ok(CaptureSource {
            id,
            name: format!("🪟 {}", title),
            width,
            height,
            source_type: CaptureSourceType::Window,
        })
    }

    fn get_window_hwnd(window: &Window) -> Result<isize, Box<dyn std::error::Error>> {
        // For now, we'll use a hash of the window title as a unique identifier
        // In a full implementation, you would use Windows API to get the actual HWND
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let title = window.title()?;
        let mut hasher = DefaultHasher::new();
        title.hash(&mut hasher);
        Ok(hasher.finish() as isize)
    }
}
