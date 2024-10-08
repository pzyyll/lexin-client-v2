//! Copyright: 2024 Lizc. All rights reserved.
//! License: MIT License
//! You may obtain a copy of the License at https://opensource.org/licenses/MIT
//!
//! Author: Lizc
//! Created Data: 2024-05-27
//!
//! Description: Translate windows.

use crate::consts;
use tauri::{Emitter, EventTarget, Listener, Manager, Runtime, WebviewWindow};
use tauri_plugin_clipboard_manager::ClipboardExt;

#[derive(Debug, Default)]
pub struct TWinState {
    is_pin: std::sync::Mutex<bool>,
}

impl TWinState {
    pub fn reset(&self) {
        *self.is_pin.lock().unwrap() = false;
    }
}

fn emit_on_cpcp<R: Runtime>(win: WebviewWindow<R>) {
    let text = win.app_handle().clipboard().read_text().unwrap();
    let _ = win.emit_to(
        EventTarget::webview_window(consts::WIN_LABEL_TRANSLATE),
        consts::WindowEvent::CPCP.to_string(),
        text,
    );
}

#[tauri::command]
pub async fn open_translate_window(app: tauri::AppHandle<impl Runtime>) {
    let _ = crate::windows::translate::show(&app, None::<fn(_)>);
}

#[tauri::command]
pub async fn set_pin<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    state: tauri::State<'_, TWinState>,
    is_pin: bool,
) -> Result<(), String> {
    let mut ispin = state.is_pin.lock().unwrap();
    *ispin = is_pin;
    Ok(())
}

pub fn try_show_on_cpcp<R: Runtime>(app: &tauri::AppHandle<R>) {
    if let Some(win) = app.get_webview_window(consts::WIN_LABEL_TRANSLATE) {
        let _ = win.show();
        let _ = win.unminimize();
        emit_on_cpcp(win);
    } else {
        let win = show(app, None::<fn(_)>).unwrap();
        let arc_app = std::sync::Arc::new(app.clone());

        win.once("on_ready", move |_event| {
            let result =
                arc_app.get_webview_window(consts::WIN_LABEL_TRANSLATE);
            match result {
                Some(win) => {
                    emit_on_cpcp(win.clone());
                }
                None => {
                    log::error!("window not found");
                }
            }
        });
    }
}

pub fn show<R, F>(
    app: &tauri::AppHandle<R>,
    on_page_load: Option<F>,
) -> Result<WebviewWindow<R>, String>
where
    R: Runtime,
    F: Fn(WebviewWindow<R>) + Send + Sync + 'static,
{
    let cursor = app.cursor_position().unwrap();

    let (x, y) = crate::utils::monitor_ex::to_logical_point(cursor.x, cursor.y);
    // println!("cursor: {}, {}", x, y);

    let window = tauri::WebviewWindowBuilder::new(
        app,
        consts::WIN_LABEL_TRANSLATE,
        tauri::WebviewUrl::App(consts::WIN_LABEL_TRANSLATE.into()),
    )
    //.decorations(false)
    .title(consts::APP_NAME)
    .always_on_top(true)
    .resizable(true)
    .auto_resize()
    .min_inner_size(300.0, 150.0)
    .inner_size(300.0, 270.0)
    //.transparent
    .position(x.into(), y.into())
    .fullscreen(false)
    .on_page_load(move |window, payload| {
        // println!("page loaded: {:?}", payload);
        use tauri::webview::PageLoadEvent;
        match payload.event() {
            PageLoadEvent::Finished => {
                if let Some(on_page_load) = &on_page_load {
                    on_page_load(window);
                }
            }
            _ => {}
        }
    })
    .build()
    .unwrap();

    match window.try_state::<TWinState>() {
        Some(state) => {
            state.reset();
        }
        None => {
            window.manage(TWinState::default());
        }
    }

    window.on_window_event(|event| {
        // println!("window event: {:?}", event);
        match event {
            tauri::WindowEvent::Focused(focused) => {
                if *focused {
                    println!("window focused");
                } else {
                    println!("window unfocused");
                }
            }
            tauri::WindowEvent::Destroyed => {
                crate::plugin::keyevent::unregister_mouse_event(
                    crate::consts::MouseEvent::LeftDown,
                );
            }
            _ => {}
        }
    });

    let arcw = std::sync::Arc::new(window.clone());
    crate::plugin::keyevent::register_mouse_event(
        crate::consts::MouseEvent::LeftDown,
        move |event| {
            let xmin = arcw.outer_position().unwrap().x;
            let ymin = arcw.outer_position().unwrap().y;
            let xmax: i32 = xmin + arcw.outer_size().unwrap().width as i32;
            let ymax = ymin + arcw.outer_size().unwrap().height as i32;
            if event.pt.x < xmin
                || event.pt.x > xmax
                || event.pt.y < ymin
                || event.pt.y > ymax
            {
                let state: tauri::State<TWinState> = arcw.state();
                println!("WinState: {:?}", state);
                if *state.is_pin.lock().unwrap() {
                    return;
                }
                arcw.close().unwrap();
            }
        },
    );

    Ok(window)

    // #[cfg(target_os = "windows")]
    // {
    //     // use window_vibrancy::apply_mica;
    //     // apply_mica(&window, true.into()).unwrap();
    //     use tauri::window::{Color, Effect, EffectState, EffectsBuilder};
    //     window.set_effects(
    //         EffectsBuilder::new()
    //             .color(Color(18, 18, 18, 18))
    //             .effect(Effect::TabbedLight)
    //             .state(EffectState::Active)
    //             .build()
    //     ).unwrap();
    // }
}
