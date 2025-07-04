use windows::Win32::Graphics::Gdi::{GetMonitorInfoW, HMONITOR, MONITORINFO};
use windows_capture::monitor::Monitor;
use windows_capture::WindowsCaptureGraphicsCaptureItem;

use crate::capture::types::{
    CaptureSource, CaptureSourceError, CaptureSourceType, MonitorInfo, WindowInfo,
};
use crate::recording::screen_recorder;

pub fn get_monitors() -> Result<Vec<MonitorInfo>, CaptureSourceError> {
    let monitors = Monitor::enumerate().map_err(CaptureSourceError::ListMonitors)?;

    let mut monitor_info = Vec::new();
    for (id, monitor) in monitors.iter().enumerate() {
        let mut mi = MONITORINFO {
            cbSize: std::mem::size_of::<MONITORINFO>() as u32,
            ..Default::default()
        };

        let hmonitor = HMONITOR(monitor.as_raw_hmonitor());

        let rect_ok = unsafe { GetMonitorInfoW(hmonitor, &mut mi) }.as_bool();

        let (left, top, width, height) = if rect_ok {
            let l = mi.rcMonitor.left;
            let t = mi.rcMonitor.top;
            let w = (mi.rcMonitor.right - mi.rcMonitor.left) as u32;
            let h = (mi.rcMonitor.bottom - mi.rcMonitor.top) as u32;
            (l, t, w, h)
        } else {
            let w = match monitor.width() {
                Ok(w) => w,
                _ => continue,
            };
            let h = match monitor.height() {
                Ok(h) => h,
                _ => continue,
            };
            (0, 0, w, h)
        };

        let name = match monitor.name() {
            Ok(n) if !n.trim().is_empty() => n,
            _ => continue,
        };

        monitor_info.push(MonitorInfo {
            id,
            hmonitor: hmonitor.0 as isize,
            name,
            width,
            height,
            left,
            top,
        });
    }

    Ok(monitor_info)
}

pub fn get_windows() -> Result<Vec<WindowInfo>, CaptureSourceError> {
    let windows =
        screen_recorder::get_available_windows().map_err(CaptureSourceError::ListWindows)?;

    let mut window_info = Vec::new();
    for (id, window) in windows.iter().enumerate() {
        let title = match window.title() {
            Ok(t) => t,
            Err(_) => continue,
        };

        let hwnd = window.as_raw_hwnd() as isize;

        let capture_item = match WindowsCaptureGraphicsCaptureItem::try_from(window.clone()) {
            Ok(item) => item,
            Err(_) => continue,
        };

        let size = match capture_item.Size() {
            Ok(s) => s,
            Err(_) => continue,
        };

        let width = size.Width as u32;
        let height = size.Height as u32;

        window_info.push(WindowInfo {
            id,
            hwnd,
            title,
            width,
            height,
        });
    }

    Ok(window_info)
}

pub fn get_all_capture_sources() -> Result<Vec<CaptureSource>, CaptureSourceError> {
    let mut sources = Vec::new();

    // Add monitors
    let monitors = get_monitors()?;
    println!("Found {} monitors", monitors.len());

    sources.extend(monitors.into_iter().map(|monitor| CaptureSource {
        name: monitor.name,
        width: monitor.width,
        height: monitor.height,
        source_type: CaptureSourceType::Monitor,
        handle: monitor.hmonitor,
        left: monitor.left,
        top: monitor.top,
    }));

    let windows = get_windows()?;
    println!("Found {} windows", windows.len());

    sources.extend(windows.into_iter().map(|window| CaptureSource {
        name: window.title,
        width: window.width,
        height: window.height,
        source_type: CaptureSourceType::Window,
        handle: window.hwnd,
        left: 0,
        top: 0,
    }));

    println!("Total sources found: {}", sources.len());
    Ok(sources)
}
