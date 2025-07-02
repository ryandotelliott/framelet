pub mod capture;
pub mod commands;
pub mod recording;
pub mod types;
pub mod window;

pub use types::*;

use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_decorum::WebviewWindowExt;

use crate::{
    commands::{
        close_region_selector, get_capture_sources, open_region_selector, region_selected,
        start_recording, stop_recording,
    },
    recording::RecordingSession,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_decorum::init())
        .manage(Mutex::new(None::<RecordingSession>))
        .invoke_handler(tauri::generate_handler![
            get_capture_sources,
            start_recording,
            stop_recording,
            open_region_selector,
            close_region_selector,
            region_selected
        ])
        .setup(|app| {
            // Create a custom titlebar for main window using https://github.com/clearlysid/tauri-plugin-decorum/
            // On Windows this will hide decoration and render custom window controls
            // On macOS it expects a hiddenTitle: true and titleBarStyle: overlay
            let main_window = app.get_webview_window("main").unwrap();
            main_window.create_overlay_titlebar().unwrap();

            #[cfg(target_os = "macos")]
            main_window.set_traffic_lights_inset(16.0, 20.0).unwrap();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
