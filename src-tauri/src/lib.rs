// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod capture_manager;
mod capture_sources;
mod screen_recorder;
mod types;

use capture_manager::CaptureManager;
use capture_sources::CaptureSourceManager;
use types::{CaptureSource, MonitorInfo, WindowInfo};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_monitors() -> Result<Vec<MonitorInfo>, String> {
    CaptureSourceManager::get_monitors()
}

#[tauri::command]
async fn get_windows() -> Result<Vec<WindowInfo>, String> {
    CaptureSourceManager::get_windows()
}

#[tauri::command]
async fn get_capture_sources() -> Result<Vec<CaptureSource>, String> {
    CaptureSourceManager::get_all_capture_sources()
}

#[tauri::command]
async fn start_capture_recording(source_id: usize, output_path: String) -> Result<String, String> {
    let monitors = screen_recorder::get_available_monitors()
        .map_err(|e| format!("Failed to get monitors: {}", e))?;
    let windows = screen_recorder::get_available_windows()
        .map_err(|e| format!("Failed to get windows: {}", e))?;

    // Determine if we're recording a monitor or window
    if source_id < monitors.len() {
        // Recording a monitor
        let monitor = monitors[source_id].clone();
        CaptureManager::start_capture_session(move |stop_signal| {
            screen_recorder::start_recording(monitor, output_path, stop_signal)
        })
    } else {
        // Recording a window
        let window_id = source_id - monitors.len();
        if window_id >= windows.len() {
            return Err("Invalid source ID".to_string());
        }

        let window = windows[window_id].clone();
        CaptureManager::start_capture_session(move |stop_signal| {
            screen_recorder::start_window_recording(window, output_path, stop_signal)
        })
    }
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
            greet,
            get_monitors,
            get_windows,
            get_capture_sources,
            start_capture_recording,
            stop_recording
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
