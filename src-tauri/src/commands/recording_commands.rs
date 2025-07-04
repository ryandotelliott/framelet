use std::{
    sync::{atomic::AtomicBool, Arc, Mutex},
    thread,
};
use tauri::State;
use windows_capture::{monitor::Monitor, window::Window, WindowsCaptureGraphicsCaptureItem};

use crate::{
    capture::types::CaptureSourceType,
    recording::types::Region,
    recording::{start_recording as start_screen_recording, RecordingSession},
};

/// Starts a new recording session
#[tauri::command]
pub async fn start_recording(
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
            if let Err(e) = start_screen_recording(item, output_path, stop_signal, region) {
                eprintln!("Recording error: {}", e);
            }
        }
    });

    *session_guard = Some(RecordingSession::new(stop_signal, thread_handle));
    Ok("Recording started".into())
}

/// Stops the current recording session
#[tauri::command]
pub async fn stop_recording(
    state: State<'_, Mutex<Option<RecordingSession>>>,
) -> Result<String, String> {
    let mut session_guard = state.lock().unwrap();
    if let Some(mut session) = session_guard.take() {
        session.stop()?;
        Ok("Recording stopped".into())
    } else {
        Err("No recording in progress".into())
    }
}
