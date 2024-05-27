//! Copyright: 2024 Lizc. All rights reserved.
//! License: MIT License
//! You may obtain a copy of the License at https://opensource.org/licenses/MIT
//!
//! Author: Lizc
//! Created Data: 2024-05-27
//!
//! Description: Store the global state of the app for easy access.

use std::sync::atomic::AtomicBool;

#[derive(Debug)]
pub struct ExitPrevent(AtomicBool);

impl From<bool> for ExitPrevent {
    fn from(b: bool) -> Self {
        Self(AtomicBool::new(b))
    }
}

impl Into<bool> for ExitPrevent {
    fn into(self) -> bool {
        self.0.load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl PartialEq<bool> for ExitPrevent {
    fn eq(&self, other: &bool) -> bool {
        self.0.load(std::sync::atomic::Ordering::Relaxed) == *other
    }
}

impl ExitPrevent {
    pub fn set(&self, b: bool) {
        self.0.store(b, std::sync::atomic::Ordering::Relaxed);
    }
}

pub struct AppState {
    pub exit_prevent: ExitPrevent,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            exit_prevent: ExitPrevent::from(true),
        }
    }
}
