use std::error::Error;
use tauri::Runtime;

mod cmds;
mod plugin;
mod windows;

fn app_setup<R: Runtime>(
    app: &mut tauri::App<R>,
) -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    {
        use tauri::Manager;
        use tauri_plugin_global_shortcut::{GlobalShortcut, ShortcutState};
        let gs = app.state::<GlobalShortcut<R>>();
        gs.on_shortcut("Alt+L", |app, _shortcut, event| {
            if let ShortcutState::Pressed = event.state {
                println!("CmdOrCtrl+Shift+D pressed");
                match tauri::WebviewWindowBuilder::new(
                    app,
                    "devlab",
                    tauri::WebviewUrl::App("debug-lab".into()),
                )
                .title("Lab Dashboard")
                .build()
                {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error opening window: {:?}", e);
                        use tauri::Error::WebviewLabelAlreadyExists;
                        // check e is WebviewLabelAlreadyExists
                        if let WebviewLabelAlreadyExists(_) = e {
                            // focus the window
                            println!("Focusing window");
                            let devlan_win = app
                                .get_webview_window("devlab")
                                .expect("window not found");
                            devlan_win
                                .set_focus()
                                .expect("failed to set focus to window");
                            devlan_win.show().expect("failed to show window");
                            devlan_win
                                .unminimize()
                                .expect("failed to unmaximize window");
                        }
                    }
                }
            }
        })?;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(plugin::single_instance::get_plugin())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(cmds::register_cmds())
        .setup(app_setup)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
