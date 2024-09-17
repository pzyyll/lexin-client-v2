use core::state::AppState;
use std::error::Error;
use std::sync::Arc;
use tauri::{Manager, Runtime};
use tauri_plugin_log::{Target, TargetKind};

mod cmds;
mod consts;
mod core;
mod module;
mod plugin;
mod utils;
mod windows;

fn app_setup<R: Runtime>(
    app: &mut tauri::App<R>,
) -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    {
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

        gs.on_shortcut("Alt+Shift+1", |_app, _shortcut, event| {
            if let ShortcutState::Pressed = event.state {
                println!("Alt+Shift+1 pressed");
                plugin::keyevent::get_focus_edit_text();
            }
        })?;
    }
    let app_state = AppState::build(app.handle());
    app.manage(app_state);
    windows::setup(app.handle());
    let apphandle = Arc::new(app.handle().clone());
    plugin::keyevent::register_copy_copy(move || {
        windows::translate::try_show_on_cpcp(&apphandle);
    });
    let apphandle = Arc::new(app.handle().clone());
    plugin::keyevent::register_double_alt(move || {
        // println!("double alt pressed");
        let _ = windows::translate::show(&apphandle, None::<fn(_)>);
    });
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin(plugin::single_instance::get_plugin())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
                    Target::new(TargetKind::Stderr),
                ])
                .build(),
        )
        .invoke_handler(cmds::register_cmds())
        .setup(app_setup)
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, event| match event {
            tauri::RunEvent::ExitRequested { code, api, .. } => {
                let exit_prevent_state = &app.state::<AppState>().exit_prevent;
                if code.is_none() && *exit_prevent_state == true {
                    println!("exiting prevented");
                    api.prevent_exit();
                }
            }
            tauri::RunEvent::Exit => {
                println!("exited");
                plugin::keyevent::clear();
            }
            tauri::RunEvent::WindowEvent { label, event, .. } => {
                // println!("window event: {} {:?}", label, event);
                match event {
                    tauri::WindowEvent::Focused(focused) => {
                        if focused {
                            println!("window focused {label}");
                        } else {
                            println!("window unfocused {label}");
                        }
                    }
                    _ => {}
                }
            }
            // tauri::RunEvent::MainEventsCleared => {
            //     #[cfg(windows)]
            //     {
            //         use ::windows::core::PCWSTR;
            //         use ::windows::Win32::Foundation::HINSTANCE;
            //         use ::windows::Win32::Graphics::Gdi::{GetDC, ReleaseDC};
            //         use ::windows::Win32::UI::WindowsAndMessaging::{
            //             DrawIcon, LoadImageW, HICON, IMAGE_ICON,
            //             LR_DEFAULTSIZE, LR_LOADFROMFILE,
            //         };
            //         use std::os::windows::ffi::OsStrExt;

            //         // let twin = app.get_window("translate");

            //         let translatewin = app.get_window("translate");
            //         if let Some(win) = translatewin {
            //             unsafe {
            //                 let hdc = GetDC(win.hwnd().unwrap());
            //                 let icon_name = std::path::Path::new(
            //                     "F:\\codespace\\win32api-guide\\icon.ico",
            //                 )
            //                 .as_os_str()
            //                 .encode_wide()
            //                 .chain(std::iter::once(0))
            //                 .collect::<Vec<u16>>();
            //                 if !hdc.is_invalid() {
            //                     println!("Got DC: {:?}", hdc);
            //                     let hicon = LoadImageW(
            //                         HINSTANCE::default(),
            //                         PCWSTR::from_raw(icon_name.as_ptr()),
            //                         IMAGE_ICON,
            //                         0,
            //                         0,
            //                         LR_DEFAULTSIZE | LR_LOADFROMFILE,
            //                     );
            //                     if hicon.is_ok() {
            //                         println!("load. {:?}", hicon);
            //                         let hicon = std::mem::transmute::<_, HICON>(
            //                             hicon.unwrap(),
            //                         );
            //                         let _ =
            //                             DrawIcon(hdc, 50, 0, hicon.clone());
            //                         let _ = DestroyIcon(hicon);
            //                     }
            //                 }
            //                 ReleaseDC(win.hwnd().unwrap(), hdc);
            //             }
            //         }
            //     }
            // }
            _ => {}
        });
}
