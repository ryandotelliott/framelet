use crate::{
    types::Region,
    window::{
        close_region_selector as close_region_selector_handler,
        open_region_selector as open_region_selector_handler,
        region_selected as region_selected_handler,
    },
};

/// Opens the region selector window for the specified monitor
#[tauri::command]
pub async fn open_region_selector(
    app: tauri::AppHandle,
    monitor_handle: isize,
) -> Result<(), String> {
    open_region_selector_handler(app, monitor_handle).await
}

/// Closes the region selector window
#[tauri::command]
pub async fn close_region_selector(app: tauri::AppHandle) -> Result<(), String> {
    close_region_selector_handler(app).await
}

/// Handles region selection and emits the selected coordinates
#[tauri::command]
pub async fn region_selected(app: tauri::AppHandle, coordinates: Region) -> Result<(), String> {
    region_selected_handler(app, coordinates).await
}
