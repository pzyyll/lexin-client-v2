//! Copyright: 2024 Lizc. All rights reserved.
//! License: MIT License
//! You may obtain a copy of the License at https://opensource.org/licenses/MIT
//!
//! Author: Lizc
//! Created Data: 2024-05-27
//!
//! Description: This file contains the implementation of the tray setup function.
//! The setup function is responsible for setting up the tray icon and menu for the application.
//! It uses the tauri crate to interact with the system tray.

use crate::consts;
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::Runtime;

pub fn setup<R: Runtime>(app: &tauri::AppHandle<R>) {
    let tray_icon = app.tray_by_id("main").expect("tray not found");
    let _ = tray_icon.set_tooltip(consts::APP_NAME.into());

    let menu = MenuBuilder::new(app)
        .item(&MenuItemBuilder::with_id("exit", "Exit").build(app).unwrap())
        .build()
        .unwrap();

    let _ = tray_icon.set_menu(menu.into());
    tray_icon.on_menu_event(|app, event| {
        if event.id == "exit" {
            // app.state::<crate::core::state::AppState>()
            //     .exit_prevent
            //     .set(false);
            app.exit(0);
        }
    });
    // tray_icon.set_show_menu_on_left_click(true).unwrap();
    tray_icon.on_tray_icon_event(
        |tray_icon, event: tauri::tray::TrayIconEvent| {
            match event {
                tauri::tray::TrayIconEvent::Click {
                    button,
                    button_state,
                    ..
                } => {
                    if button == tauri::tray::MouseButton::Left
                        && button_state == tauri::tray::MouseButtonState::Up
                    {
                        println!("Left click");
                        // show the main windows
                        crate::windows::home::show(tray_icon.app_handle());
                    }
                }
                // tauri::tray::ClickType::Left => {
                //     println!("Left click");
                //     // show the main windows
                //     crate::windows::home::show(tray_icon.app_handle());
                // }
                _ => {}
            }
        },
    );
}
