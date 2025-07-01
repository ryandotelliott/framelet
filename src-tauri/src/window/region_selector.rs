use crate::types::Region;
use std::os::raw::c_void;
use tauri::{Emitter, Manager};
use windows::Win32::Graphics::Gdi::{GetMonitorInfoW, HMONITOR, MONITORINFO};

/// Opens the region selector window for the specified monitor
pub async fn open_region_selector(
    app: tauri::AppHandle,
    monitor_handle: isize,
) -> Result<(), String> {
    // Retrieve rectangle directly from the HMONITOR
    let hmonitor = HMONITOR(monitor_handle as *mut c_void);

    let mut mi = MONITORINFO {
        cbSize: std::mem::size_of::<MONITORINFO>() as u32,
        ..Default::default()
    };

    if unsafe { GetMonitorInfoW(hmonitor, &mut mi) }.as_bool() {
        let left = mi.rcMonitor.left as f64;
        let top = mi.rcMonitor.top as f64;

        // Creating the window on-demand might have a slight performance penalty, but when trying to
        // reuse the same window there was odd behavior with the window flashing a menubar / visible resize.
        let window = tauri::WebviewWindowBuilder::new(
            &app,
            "region-selector",
            tauri::WebviewUrl::App("src/panels/region-selector/index.html".into()),
        )
        .title("Region Selector")
        .decorations(false)
        .transparent(true)
        .always_on_top(true)
        .resizable(false)
        .visible(false) // keep hidden until needed
        .position(left, top)
        .fullscreen(true)
        .build();

        let window = window.map_err(|e| e.to_string())?;

        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Failed to retrieve monitor information".into())
    }
}

/// Closes the region selector window
pub async fn close_region_selector(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("region-selector") {
        window
            .destroy()
            .map_err(|e| format!("Failed to hide region selector: {}", e))?;
    }
    Ok(())
}

/// Handles region selection and emits the selected coordinates
pub async fn region_selected(app: tauri::AppHandle, coordinates: Region) -> Result<(), String> {
    println!("Region selected: {:?}", coordinates);

    // Emit an event to the main window with the selected coordinates
    app.emit("region-selected", &coordinates)
        .map_err(|e| format!("Failed to emit region-selected event: {}", e))?;

    // Close the region selector window
    if let Some(window) = app.get_webview_window("region-selector") {
        window
            .destroy()
            .map_err(|e| format!("Failed to hide region selector: {}", e))?;
    }

    Ok(())
}
