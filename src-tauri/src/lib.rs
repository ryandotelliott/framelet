// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::{Deserialize, Serialize};
use std::sync::{atomic::AtomicBool, Arc, Mutex};
use std::thread;

mod screen_recorder;

#[derive(Debug, Serialize, Deserialize)]
pub struct MonitorInfo {
    pub id: usize,
    pub name: String,
    pub width: u32,
    pub height: u32,
}

// Global state for recording with proper synchronization
lazy_static::lazy_static! {
    static ref RECORDING_STATE: Mutex<RecordingState> = Mutex::new(RecordingState {
        stop_signal: None,
        recording_thread: None,
    });
}

struct RecordingState {
    stop_signal: Option<Arc<AtomicBool>>,
    recording_thread: Option<thread::JoinHandle<()>>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_monitors() -> Result<Vec<MonitorInfo>, String> {
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

#[tauri::command]
async fn start_recording(monitor_id: usize, output_path: String) -> Result<String, String> {
    let monitors = screen_recorder::get_available_monitors()
        .map_err(|e| format!("Failed to get monitors: {}", e))?;

    if monitor_id >= monitors.len() {
        return Err("Invalid monitor ID".to_string());
    }

    let monitor = monitors[monitor_id].clone();
    let stop_signal = Arc::new(AtomicBool::new(false));

    // Store the stop signal safely
    let mut state = RECORDING_STATE.lock().unwrap();
    state.stop_signal = Some(stop_signal.clone());

    // Start recording in a separate thread
    let recording_thread = thread::spawn(move || {
        if let Err(e) = screen_recorder::start_recording(monitor, output_path, stop_signal) {
            eprintln!("Recording error: {}", e);
        }
    });

    state.recording_thread = Some(recording_thread);

    Ok("Recording started".to_string())
}

#[tauri::command]
async fn stop_recording() -> Result<String, String> {
    let mut state = RECORDING_STATE.lock().unwrap();

    if let Some(stop_signal) = &state.stop_signal {
        stop_signal.store(true, std::sync::atomic::Ordering::Relaxed);

        // Wait for the recording thread to finish
        if let Some(thread) = state.recording_thread.take() {
            drop(state); // Release the lock before joining
            thread
                .join()
                .map_err(|_| "Failed to join recording thread")?;

            // Reacquire the lock to clear the stop signal
            let mut state = RECORDING_STATE.lock().unwrap();
            state.stop_signal = None;
        }

        Ok("Recording stopped".to_string())
    } else {
        Err("No recording in progress".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_monitors,
            start_recording,
            stop_recording
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
