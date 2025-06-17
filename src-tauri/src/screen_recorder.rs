use std::{
    io::{self, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Instant,
};

use windows_capture::{
    capture::{Context, GraphicsCaptureApiHandler},
    encoder::{AudioSettingsBuilder, ContainerSettingsBuilder, VideoEncoder, VideoSettingsBuilder},
    frame::Frame,
    graphics_capture_api::InternalCaptureControl,
    settings::{ColorFormat, CursorCaptureSettings, DrawBorderSettings, Settings},
    window::Window,
    WindowsCaptureGraphicsCaptureItem,
};

// TODO: Audio capture - use wasapi-rs with `send_frame_with_audio` or `send_audio_buffer` in windows-capture

#[derive(Debug, Clone)]
pub struct RecordingConfig {
    pub width: u32,
    pub height: u32,
    pub output_path: String,
    pub stop_signal: Arc<AtomicBool>,
}

// Handles capture events.
pub struct ScreenRecorder {
    encoder: Option<VideoEncoder>,
    start: Instant,
    stop_signal: Arc<AtomicBool>,
}

impl GraphicsCaptureApiHandler for ScreenRecorder {
    type Flags = RecordingConfig;
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn new(ctx: Context<Self::Flags>) -> Result<Self, Self::Error> {
        println!("Using dimensions: {}x{}", ctx.flags.width, ctx.flags.height);
        println!("Output file: {}", ctx.flags.output_path);

        let encoder = VideoEncoder::new(
            VideoSettingsBuilder::new(ctx.flags.width, ctx.flags.height),
            AudioSettingsBuilder::default().disabled(true),
            ContainerSettingsBuilder::default(),
            &ctx.flags.output_path,
        )?;

        Ok(Self {
            encoder: Some(encoder),
            start: Instant::now(),
            stop_signal: ctx.flags.stop_signal,
        })
    }

    // Called every time a new frame is available.
    fn on_frame_arrived(
        &mut self,
        frame: &mut Frame,
        capture_control: InternalCaptureControl,
    ) -> Result<(), Self::Error> {
        if self.stop_signal.load(Ordering::Relaxed) {
            println!("\nStopping recording...");
            self.encoder.take().unwrap().finish()?;
            capture_control.stop();
            return Ok(());
        }

        print!(
            "\rRecording for: {} seconds",
            self.start.elapsed().as_secs()
        );
        io::stdout().flush()?;

        // TODO: Apply any cropping here - potentially using WGPU?

        // Send the frame to the video encoder
        self.encoder.as_mut().unwrap().send_frame(frame)?;

        Ok(())
    }

    // Optional handler called when the capture item (usually a window) closes.
    fn on_closed(&mut self) -> Result<(), Self::Error> {
        println!("Capture session ended");
        Ok(())
    }
}

pub fn get_available_windows() -> Result<Vec<Window>, Box<dyn std::error::Error>> {
    let windows = Window::enumerate()?;

    // Filter out minimized windows and windows without titles
    let filtered_windows: Vec<Window> = windows
        .into_iter()
        .filter(|window| {
            // Check if window has a title and is not minimized
            if let Ok(title) = window.title() {
                if !title.trim().is_empty() {
                    // Check if window is visible (not minimized)
                    // The windows-capture library should handle this, but we can add additional checks
                    return true;
                }
            }
            false
        })
        .collect();

    Ok(filtered_windows)
}

pub fn start_recording(
    capture_item: WindowsCaptureGraphicsCaptureItem,
    output_path: String,
    stop_signal: Arc<AtomicBool>,
) -> Result<(), Box<dyn std::error::Error>> {
    let width = capture_item.Size()?.Width as u32;
    let height = capture_item.Size()?.Height as u32;

    let config = RecordingConfig {
        width,
        height,
        output_path,
        stop_signal,
    };

    let settings = Settings::new(
        capture_item,
        CursorCaptureSettings::Default,
        DrawBorderSettings::Default,
        ColorFormat::Rgba8,
        config,
    );

    ScreenRecorder::start(settings)?;
    Ok(())
}
