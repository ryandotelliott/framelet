use std::{
    io::{self, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Instant,
};

use windows_capture::{
    capture::{Context, GraphicsCaptureApiHandler},
    encoder::{AudioSettingsBuilder, ContainerSettingsBuilder, VideoEncoder, VideoSettingsBuilder},
    frame::Frame,
    graphics_capture_api::InternalCaptureControl,
    monitor::Monitor,
    settings::{ColorFormat, CursorCaptureSettings, DrawBorderSettings, Settings},
};

// Structure to hold recording configuration
#[derive(Debug, Clone)]
pub struct RecordingConfig {
    pub monitor_width: u32,
    pub monitor_height: u32,
    pub monitor_name: String,
    pub output_path: String,
    pub stop_signal: Arc<AtomicBool>,
}

// Handles capture events.
pub struct ScreenRecorder {
    // The video encoder that will be used to encode the frames.
    encoder: Option<VideoEncoder>,
    // To measure the time the capture has been running
    start: Instant,
    // Signal to stop recording
    stop_signal: Arc<AtomicBool>,
}

impl GraphicsCaptureApiHandler for ScreenRecorder {
    // The type of flags used to get the values from the settings.
    type Flags = RecordingConfig;

    // The type of error that can be returned from `CaptureControl` and `start` functions.
    type Error = Box<dyn std::error::Error + Send + Sync>;

    // Function that will be called to create a new instance. The flags can be passed from settings.
    fn new(ctx: Context<Self::Flags>) -> Result<Self, Self::Error> {
        println!("Starting recording on: {}", ctx.flags.monitor_name);
        println!(
            "Using dimensions: {}x{}",
            ctx.flags.monitor_width, ctx.flags.monitor_height
        );
        println!("Output file: {}", ctx.flags.output_path);
        println!("Press Ctrl+C or any key to stop recording...");

        let encoder = VideoEncoder::new(
            VideoSettingsBuilder::new(ctx.flags.monitor_width, ctx.flags.monitor_height),
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
        // Check if stop signal has been triggered
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

        // Note: The frame has other uses too, for example, you can save a single frame to a file, like this:
        // frame.save_as_image("frame.png", ImageFormat::Png)?;
        // Or get the raw data like this so you have full control:
        // let data = frame.buffer()?;

        Ok(())
    }

    // Optional handler called when the capture item (usually a window) closes.
    fn on_closed(&mut self) -> Result<(), Self::Error> {
        println!("Capture session ended");
        Ok(())
    }
}

pub fn get_available_monitors() -> Result<Vec<Monitor>, Box<dyn std::error::Error>> {
    let monitors = Monitor::enumerate()?;
    Ok(monitors)
}

pub fn start_recording(
    monitor: Monitor,
    output_path: String,
    stop_signal: Arc<AtomicBool>,
) -> Result<(), Box<dyn std::error::Error>> {
    let width = monitor.width()?;
    let height = monitor.height()?;
    let name = monitor.name()?;

    let config = RecordingConfig {
        monitor_width: width,
        monitor_height: height,
        monitor_name: name,
        output_path,
        stop_signal,
    };

    let settings = Settings::new(
        monitor,
        CursorCaptureSettings::Default,
        DrawBorderSettings::Default,
        ColorFormat::Rgba8,
        config,
    );

    ScreenRecorder::start(settings)?;
    Ok(())
}

pub fn setup_key_listener(stop_signal: Arc<AtomicBool>) {
    thread::spawn(move || {
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        stop_signal.store(true, Ordering::Relaxed);
    });
}
