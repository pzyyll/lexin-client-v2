//! Copyright: 2024 Lizc. All rights reserved.
//! License: MIT License
//! You may obtain a copy of the License at https://opensource.org/licenses/MIT
//!
//! Author: Lizc
//! Created Data: 2024-06-08
//!
//! Description: Windows platform implementation of keyevent

#![allow(unused)]

#[cfg(windows)]
pub mod impl_windows;

#[cfg(windows)]
pub use impl_windows::*;

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct MouseEventInfo {
    pub pt: Point,
}
