//! Copyright: 2024 Lizc. All rights reserved.
//! License: MIT License
//! You may obtain a copy of the License at https://opensource.org/licenses/MIT
//!
//! Author: Lizc
//! Created Data: 2024-05-27
//!
//! Description: Store the global state of the app for easy access.

use crate::consts::APP_DATA_CONFIG;
use crate::utils::settings::Settings;
use std::sync::atomic::AtomicBool;
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};

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
    #[allow(unused)]
    pub fn set(&self, b: bool) {
        self.0.store(b, std::sync::atomic::Ordering::Relaxed);
    }
}

pub struct AppState {
    pub exit_prevent: ExitPrevent,
    pub settings: Settings,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            exit_prevent: ExitPrevent::from(true),
            settings: Settings::new(),
        }
    }
}

impl AppState {
    pub fn build<R: tauri::Runtime>(app: &AppHandle<R>) -> Self {
        let settings = Settings::load_from_file(
            app.path()
                .resolve(APP_DATA_CONFIG, BaseDirectory::AppData)
                .unwrap()
                .to_str()
                .unwrap(),
        )
        .unwrap_or_else(|e| {
            eprintln!("Error loading settings: {:?}", e);
            Settings::new()
        });
        Self {
            settings,
            ..Default::default()
        }
    }
}
