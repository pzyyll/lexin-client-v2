//! Copyright: 2024 Lizc. All rights reserved.
//! License: MIT License
//! You may obtain a copy of the License at https://opensource.org/licenses/MIT
//!
//! Author: Lizc
//! Created Data: 2024-05-27
//!
//! Description: Translate windows.

use crate::consts;
use tauri::Runtime;

#[tauri::command]
pub async fn open_translate_window(app: tauri::AppHandle<impl Runtime>) {
    crate::windows::translate::show(&app);
}

pub fn show<R: Runtime>(app: &tauri::AppHandle<R>) {
    let window = tauri::WebviewWindowBuilder::new(
        app,
        consts::WIN_LABEL_TRANSLATE,
        tauri::WebviewUrl::App(consts::WIN_LABEL_TRANSLATE.into()),
    )
    .decorations(false)
    .always_on_top(true)
    .resizable(true)
    //.auto_resize()
    //.min_inner_size(350.0, 250.0)
    //.transparent(true)
    .fullscreen(false)
    .build()
    .unwrap();

    // #[cfg(target_os = "windows")]
    // {
    //     use window_vibrancy::apply_mica;
    //     apply_mica(&window, true.into()).unwrap();
    // }
    #[cfg(target_os = "macos")]
    {
        use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
        apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
            .expect(
            "Unsupported platform! 'apply_vibrancy' is only supported on macOS",
        );
    }
}
