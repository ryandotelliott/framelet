// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod capture_manager;
mod capture_sources;
mod screen_recorder;
mod types;

use capture_manager::CaptureManager;
use capture_sources::CaptureSourceManager;
use types::CaptureSource;
use windows_capture::{monitor::Monitor, window::Window, WindowsCaptureGraphicsCaptureItem};

use crate::{capture_sources::CaptureSourceError, types::CaptureSourceType};

// TODO: Need to clean up error handling to not just use format.

#[tauri::command]
async fn get_capture_sources() -> Result<Vec<CaptureSource>, CaptureSourceError> {
    CaptureSourceManager::get_all_capture_sources()
}

#[tauri::command]
async fn start_recording(
    handle: isize,
    source_type: CaptureSourceType,
    output_path: String,
) -> Result<String, String> {
    let item = match source_type {
        CaptureSourceType::Monitor => {
            let monitor = Monitor::from_raw_hmonitor(handle as *mut _);
            WindowsCaptureGraphicsCaptureItem::try_from(monitor)
                .map_err(|e| format!("Failed to create capture item: {}", e))?
        }
        CaptureSourceType::Window => {
            let win = Window::from_raw_hwnd(handle as *mut _);
            WindowsCaptureGraphicsCaptureItem::try_from(win)
                .map_err(|e| format!("Failed to create capture item: {}", e))?
        }
    };

    // start recording exactly once
    CaptureManager::start_capture_session(move |stop_signal| {
        screen_recorder::start_recording(item, output_path, stop_signal)
    })
}

#[tauri::command]
async fn stop_recording() -> Result<String, String> {
    CaptureManager::stop_capture_session()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_capture_sources,
            start_recording,
            stop_recording
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
