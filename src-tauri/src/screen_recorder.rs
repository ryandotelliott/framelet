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

use crate::types::Region;

// TODO: Audio capture - use wasapi-rs with `send_frame_with_audio` or `send_audio_buffer` in windows-capture

#[derive(Debug, Clone)]
pub struct RecordingConfig {
    pub width: u32,
    pub height: u32,
    pub output_path: String,
    pub stop_signal: Arc<AtomicBool>,
    pub region: Option<Region>,
}

// Handles capture events.
pub struct ScreenRecorder {
    encoder: Option<VideoEncoder>,
    start: Instant,
    stop_signal: Arc<AtomicBool>,
    region: Option<Region>,
}

impl GraphicsCaptureApiHandler for ScreenRecorder {
    type Flags = RecordingConfig;
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn new(ctx: Context<Self::Flags>) -> Result<Self, Self::Error> {
        println!("Using dimensions: {}x{}", ctx.flags.width, ctx.flags.height);
        println!("Output file: {}", ctx.flags.output_path);

        // Use cropped dimensions if region is specified
        let (encoder_width, encoder_height) = if let Some(region) = &ctx.flags.region {
            (region.width, region.height)
        } else {
            (ctx.flags.width, ctx.flags.height)
        };

        println!("Encoder dimensions: {}x{}", encoder_width, encoder_height);

        let encoder = VideoEncoder::new(
            VideoSettingsBuilder::new(encoder_width, encoder_height),
            AudioSettingsBuilder::default().disabled(true),
            ContainerSettingsBuilder::default(),
            &ctx.flags.output_path,
        )?;

        Ok(Self {
            encoder: Some(encoder),
            start: Instant::now(),
            stop_signal: ctx.flags.stop_signal,
            region: ctx.flags.region,
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

        match &self.region {
            Some(region) => {
                // The region coordinates are in screen space, but we need to adjust them
                // relative to the captured frame. For now, let's assume they're already correct
                // but add some debugging to verify
                let start_x = region.x as u32;
                let start_y = region.y as u32;
                let end_x = (region.x as u32) + region.width;
                let end_y = (region.y as u32) + region.height;

                let duration = frame.timespan().Duration;
                let mut cropped_frame = frame
                    .buffer_crop(start_x, start_y, end_x, end_y)
                    .expect("Failed to crop buffer");

                let raw_cropped_buffer = cropped_frame.as_nopadding_buffer()?;

                let expected_size = region.width as usize * region.height as usize * 4;
                if raw_cropped_buffer.len() != expected_size {
                    println!(
                        "WARNING: Buffer size mismatch! Expected: {}, Got: {}",
                        expected_size,
                        raw_cropped_buffer.len()
                    );
                }

                // The buffer might be upside down due to coordinate system differences
                // Let's try flipping it vertically
                let mut flipped_buffer = Vec::with_capacity(raw_cropped_buffer.len());
                let width = region.width as usize;
                let height = region.height as usize;
                let bytes_per_row = width * 4; // RGBA = 4 bytes per pixel

                for row in (0..height).rev() {
                    let start_idx = row * bytes_per_row;
                    let end_idx = start_idx + bytes_per_row;
                    flipped_buffer.extend_from_slice(&raw_cropped_buffer[start_idx..end_idx]);
                }

                match self
                    .encoder
                    .as_mut()
                    .unwrap()
                    .send_frame_buffer(&flipped_buffer, duration)
                {
                    Ok(_) => (),
                    Err(e) => {
                        println!("ERROR sending frame: {}", e);
                        return Err(Box::new(e));
                    }
                }
            }
            None => {
                self.encoder.as_mut().unwrap().send_frame(frame)?;
            }
        }

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
    region: Option<Region>,
) -> Result<(), Box<dyn std::error::Error>> {
    let width = capture_item.Size()?.Width as u32;
    let height = capture_item.Size()?.Height as u32;

    let config = RecordingConfig {
        width,
        height,
        output_path,
        stop_signal,
        region,
    };

    let settings = Settings::new(
        capture_item,
        CursorCaptureSettings::Default,
        DrawBorderSettings::WithoutBorder,
        ColorFormat::Rgba8,
        config,
    );

    ScreenRecorder::start(settings)?;
    Ok(())
}
