// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod capture_sources;
mod screen_recorder;
mod types;

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
};
use tauri::State;

use capture_sources::CaptureSourceManager;
use types::CaptureSource;
use windows_capture::{monitor::Monitor, window::Window, WindowsCaptureGraphicsCaptureItem};

use crate::{capture_sources::CaptureSourceError, types::CaptureSourceType};

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
        let stop_signal = stop_signal.clone();
        move || {
            if let Err(e) = screen_recorder::start_recording(item, output_path, stop_signal) {
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

struct RecordingSession {
    stop_signal: Arc<AtomicBool>,
    recording_thread: Option<thread::JoinHandle<()>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(None::<RecordingSession>))
        .invoke_handler(tauri::generate_handler![
            get_capture_sources,
            start_recording,
            stop_recording
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
