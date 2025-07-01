pub mod screen_recorder;
mod session;

pub use screen_recorder::{start_recording, RecordingConfig, ScreenRecorder};
pub use session::RecordingSession;
