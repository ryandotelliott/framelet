use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

/// Manages the lifecycle of a recording session
pub struct RecordingSession {
    stop_signal: Arc<AtomicBool>,
    recording_thread: Option<thread::JoinHandle<()>>,
}

impl RecordingSession {
    /// Creates a new recording session
    pub fn new(stop_signal: Arc<AtomicBool>, recording_thread: thread::JoinHandle<()>) -> Self {
        Self {
            stop_signal,
            recording_thread: Some(recording_thread),
        }
    }

    /// Stops the recording session
    pub fn stop(&mut self) -> Result<(), String> {
        self.stop_signal.store(true, Ordering::Relaxed);

        if let Some(handle) = self.recording_thread.take() {
            handle
                .join()
                .map_err(|_| "Failed to join recording thread".to_string())?;
        }

        Ok(())
    }

    /// Checks if the session is active
    pub fn is_active(&self) -> bool {
        self.recording_thread.is_some()
    }
}
