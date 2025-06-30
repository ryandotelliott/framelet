mod capture_sources;
mod screen_recorder;
mod types;

use std::{
    os::raw::c_void,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
};
use tauri::{Emitter, Manager, State};

use tauri_plugin_decorum::WebviewWindowExt;

use crate::{
    capture_sources::CaptureSourceError,
    types::{CaptureSourceType, Region},
};
use capture_sources::CaptureSourceManager;
use types::CaptureSource;

use windows_capture::{monitor::Monitor, window::Window, WindowsCaptureGraphicsCaptureItem};

#[tauri::command]
async fn get_capture_sources() -> Result<Vec<CaptureSource>, CaptureSourceError> {
    CaptureSourceManager::get_all_capture_sources()
}

#[tauri::command]
async fn start_recording(
    state: State<'_, Mutex<Option<RecordingSession>>>,
    handle: isize,
    source_type: CaptureSourceType,
    output_path: String,
    region: Option<Region>,
) -> Result<String, String> {
    // Used to prevent concurrent recordings
    let mut session_guard = state.lock().unwrap();
    if session_guard.is_some() {
        return Err("Recording already in progress".into());
    }

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

    let stop_signal = Arc::new(AtomicBool::new(false));
    let thread_handle = thread::spawn({
        let stop_signal = Arc::clone(&stop_signal);
        move || {
            if let Err(e) = screen_recorder::start_recording(item, output_path, stop_signal, region)
            {
                eprintln!("Recording error: {}", e);
            }
        }
    });

    *session_guard = Some(RecordingSession {
        stop_signal,
        recording_thread: Some(thread_handle),
    });
    Ok("Recording started".into())
}

#[tauri::command]
async fn stop_recording(
    state: State<'_, Mutex<Option<RecordingSession>>>,
) -> Result<String, String> {
    let mut session_guard = state.lock().unwrap();
    if let Some(mut session) = session_guard.take() {
        session.stop_signal.store(true, Ordering::Relaxed);
        if let Some(handle) = session.recording_thread.take() {
            handle
                .join()
                .map_err(|_| "Failed to join recording thread".to_string())?;
        }
        Ok("Recording stopped".into())
    } else {
        Err("No recording in progress".into())
    }
}

#[tauri::command]
async fn open_region_selector(app: tauri::AppHandle, monitor_handle: isize) -> Result<(), String> {
    use windows::Win32::Graphics::Gdi::{GetMonitorInfoW, HMONITOR, MONITORINFO};

    // Retrieve rectangle directly from the HMONITOR
    let hmonitor = HMONITOR(monitor_handle as *mut c_void);

    let mut mi = MONITORINFO {
        cbSize: std::mem::size_of::<MONITORINFO>() as u32,
        ..Default::default()
    };

    if unsafe { GetMonitorInfoW(hmonitor, &mut mi) }.as_bool() {
        let left = mi.rcMonitor.left as f64;
        let top = mi.rcMonitor.top as f64;

        // Creating the window on-demand might have a slight performance penalty, but when trying to
        // reuse the same window there was odd behavior with the window flashing a menubar / visible resize.
        let window = tauri::WebviewWindowBuilder::new(
            &app,
            "region-selector",
            tauri::WebviewUrl::App("src/windows/region-selector/index.html".into()),
        )
        .title("Region Selector")
        .decorations(false)
        .transparent(true)
        .always_on_top(true)
        .resizable(false)
        .visible(false) // keep hidden until needed
        .position(left, top)
        .fullscreen(true)
        .build();

        let window = window.map_err(|e| e.to_string())?;

        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Failed to retrieve monitor information".into())
    }
}

#[tauri::command]
async fn close_region_selector(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("region-selector") {
        window
            .destroy()
            .map_err(|e| format!("Failed to hide region selector: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
async fn region_selected(app: tauri::AppHandle, coordinates: Region) -> Result<(), String> {
    println!("Region selected: {:?}", coordinates);

    // Emit an event to the main window with the selected coordinates
    app.emit("region-selected", &coordinates)
        .map_err(|e| format!("Failed to emit region-selected event: {}", e))?;

    // Close the region selector window
    if let Some(window) = app.get_webview_window("region-selector") {
        window
            .destroy()
            .map_err(|e| format!("Failed to hide region selector: {}", e))?;
    }

    Ok(())
}

struct RecordingSession {
    stop_signal: Arc<AtomicBool>,
    recording_thread: Option<thread::JoinHandle<()>>,
}

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
