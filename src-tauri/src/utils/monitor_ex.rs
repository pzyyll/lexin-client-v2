//! Copyright: 2024 Lizc. All rights reserved.
//! License: MIT License
//! You may obtain a copy of the License at https://opensource.org/licenses/MIT
//!
//! Author: Lizc
//! Created Data: 2024-05-29
//!
//! Description: Get display information

#![cfg(windows)]
#![allow(unused)]

use windows::Win32::{
    Foundation::{HWND, POINT},
    Graphics::Gdi::{GetDC, GetDeviceCaps, LOGPIXELSX},
    Graphics::Gdi::{
        GetMonitorInfoW, MonitorFromPoint, MonitorFromWindow, MONITORINFO,
        MONITOR_DEFAULTTONEAREST,
    },
    UI::Shell::GetScaleFactorForMonitor,
    UI::WindowsAndMessaging::PhysicalToLogicalPoint,
};

pub fn get_monitor_info_bywin(hwnd: HWND) -> Result<MONITORINFO, &'static str> {
    let hmonitor = unsafe { MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST) };
    let mut monitor_info = MONITORINFO::default();
    monitor_info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
    let result = unsafe { GetMonitorInfoW(hmonitor, &mut monitor_info) };
    if result.into() {
        Ok(monitor_info)
    } else {
        Err("Error getting monitor info")
    }
}

pub fn get_monitor_info_bypoint(
    x: i32,
    y: i32,
) -> Result<MONITORINFO, &'static str> {
    let hmonitor =
        unsafe { MonitorFromPoint(POINT { x, y }, MONITOR_DEFAULTTONEAREST) };
    let mut monitor_info = MONITORINFO::default();
    monitor_info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
    let result = unsafe { GetMonitorInfoW(hmonitor, &mut monitor_info) };
    if result.into() {
        Ok(monitor_info)
    } else {
        Err("Error getting monitor info")
    }
}

pub fn to_logical_point(x: f64, y: f64) -> (f64, f64) {
    let hwnd =
        unsafe { windows::Win32::UI::WindowsAndMessaging::GetDesktopWindow() };

    let dpi = unsafe { GetDeviceCaps(GetDC(hwnd), LOGPIXELSX) };
    println!("dpi: {}", dpi);

    let scale = dpi as f64 / 96.0f64;

    (x / scale, y / scale)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(windows)]
    #[test]
    fn test_get_monitor_info_bywin() {
        use windows::Win32::UI::WindowsAndMessaging::GetDesktopWindow;
        let hwnd = unsafe { GetDesktopWindow() };
        let result = get_monitor_info_bywin(hwnd);
        assert!(result.is_ok());
    }
}
