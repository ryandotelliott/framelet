mod sources;
pub mod types;
mod webcam;

pub use sources::{get_all_capture_sources, get_monitors, get_windows};
pub use types::{CaptureSource, CaptureSourceError, CaptureSourceType, MonitorInfo, WindowInfo};

pub use webcam::{get_webcams, Webcam, WebcamError};
