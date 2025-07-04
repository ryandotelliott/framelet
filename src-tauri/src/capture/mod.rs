mod manager;
mod sources;

pub use manager::CaptureSourceManager;
pub use sources::{MonitorInfo, WindowInfo};

// Re-export error types
pub use sources::CaptureSourceError;
