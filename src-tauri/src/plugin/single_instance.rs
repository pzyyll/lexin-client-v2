// This file is used to create a single instance of the app.
// If the app is already running,
// it will focus on the existing instance instead of opening a new one.

use tauri::plugin::TauriPlugin;
use tauri::Manager;
use tauri::Runtime;

pub fn get_plugin<R: Runtime>() -> TauriPlugin<R> {
    tauri_plugin_single_instance::init(|app, _args, _error| {
        app.webview_windows()
            .values()
            .next()
            .expect("no window found")
            .set_focus()
            .expect("failed to set focus to window");
    })
}
