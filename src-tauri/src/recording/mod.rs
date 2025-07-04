pub mod screen_recorder;
mod session;
pub mod types;

pub use screen_recorder::{start_recording, RecordingConfig, ScreenRecorder};
pub use session::RecordingSession;
pub use types::Region;
