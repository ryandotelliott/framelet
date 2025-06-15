use std::sync::{atomic::AtomicBool, Arc, Mutex};
use std::thread;

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

pub struct CaptureManager;

impl CaptureManager {
    pub fn start_capture_session<F>(recording_fn: F) -> Result<String, String>
    where
        F: FnOnce(Arc<AtomicBool>) -> Result<(), Box<dyn std::error::Error>> + Send + 'static,
    {
        let stop_signal = Arc::new(AtomicBool::new(false));

        // Store the stop signal safely
        let mut state = RECORDING_STATE.lock().unwrap();
        state.stop_signal = Some(stop_signal.clone());

        // Start recording in a separate thread
        let recording_thread = thread::spawn(move || {
            if let Err(e) = recording_fn(stop_signal) {
                eprintln!("Recording error: {}", e);
            }
        });

        state.recording_thread = Some(recording_thread);

        Ok("Recording started".to_string())
    }

    pub fn stop_capture_session() -> Result<String, String> {
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
}
