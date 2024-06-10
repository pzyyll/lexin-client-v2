//! Copyright: 2024 Lizc. All rights reserved.
//! License: MIT License
//! You may obtain a copy of the License at https://opensource.org/licenses/MIT
//!
//! Author: Lizc
//! Created Data: 2024-05-27
//!
//! Description: The home window module for the application.

use crate::consts;
use tauri::{Manager, Runtime, WebviewWindowBuilder};

#[allow(unused)]
pub fn show<R: Runtime>(app: &tauri::AppHandle<R>) {
    match app.get_webview_window(consts::WIN_LABEL_MAIN) {
        Some(win) => {
            win.show().expect("failed to show window");
            if win.is_minimized().unwrap_or(false) {
                win.unminimize().expect("failed to unminimize window");
            }
            win.set_focus();
        }
        None => {
            WebviewWindowBuilder::new(
                app,
                consts::WIN_LABEL_MAIN,
                tauri::WebviewUrl::App("main".into()),
            )
            .resizable(true)
            .fullscreen(false)
            .title(consts::APP_NAME)
            .inner_size(800.0, 600.0)
            .build()
            .unwrap();
        }
    }
}
