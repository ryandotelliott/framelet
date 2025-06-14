use std::{
    env, fs,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{SystemTime, UNIX_EPOCH},
};

use inquire::{Select, Text};
use windows_capture::monitor::Monitor;

use crate::screen_recorder::{get_available_monitors, setup_key_listener, start_recording};

#[derive(Debug)]
pub struct MonitorOption {
    pub monitor: Monitor,
    pub display_name: String,
}

impl std::fmt::Display for MonitorOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name)
    }
}

pub fn run_cli() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎬 Framelet - Screen Recorder");
    println!("=============================");

    // Get available monitors
    let monitors = get_available_monitors()?;
    if monitors.is_empty() {
        return Err("No monitors found".into());
    }

    // Create monitor options for the user to select from
    let mut monitor_options = Vec::new();
    for (index, monitor) in monitors.into_iter().enumerate() {
        let width = monitor.width().unwrap_or(0);
        let height = monitor.height().unwrap_or(0);
        let name = monitor
            .name()
            .unwrap_or_else(|_| format!("Monitor {}", index + 1));

        let display_name = format!("Monitor {}: {} ({}x{})", index + 1, name, width, height);
        monitor_options.push(MonitorOption {
            monitor,
            display_name,
        });
    }

    // Let user select a monitor
    let selected_monitor = Select::new("Select monitor to record:", monitor_options).prompt()?;

    // Ask for output directory
    let current_dir = env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .to_string_lossy()
        .to_string();

    let output_dir = Text::new("Output directory:")
        .with_default(&current_dir)
        .with_help_message("Press Enter to use current directory")
        .prompt()?;

    // Create output directory if it doesn't exist
    let output_path = PathBuf::from(&output_dir);
    if !output_path.exists() {
        fs::create_dir_all(&output_path)?;
        println!("Created output directory: {}", output_path.display());
    }

    // Create a temporary filename
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let temp_filename = format!("framelet_recording_{}.mp4", timestamp);
    let temp_path = output_path.join(&temp_filename);

    // Create stop signal
    let stop_signal = Arc::new(AtomicBool::new(false));

    // Setup Ctrl+C handler
    let stop_signal_clone = stop_signal.clone();
    ctrlc::set_handler(move || {
        println!("\nReceived Ctrl+C, stopping recording...");
        stop_signal_clone.store(true, Ordering::Relaxed);
    })?;

    // Setup key listener for any key press
    let stop_signal_clone = stop_signal.clone();
    setup_key_listener(stop_signal_clone);

    println!("\n🔴 Starting recording...");
    println!("Monitor: {}", selected_monitor.display_name);
    println!("Output: {}", temp_path.display());
    println!("\nPress any key or Ctrl+C to stop recording.");

    // Start recording
    match start_recording(
        selected_monitor.monitor,
        temp_path.to_string_lossy().to_string(),
        stop_signal,
    ) {
        Ok(_) => {
            println!("\n✅ Recording completed successfully!");
            println!("File saved to: {}", temp_path.display());
        }
        Err(e) => {
            println!("\n❌ Recording failed: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
