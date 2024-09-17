pub mod settings;

use crate::module::translate;

#[tauri::command]
async fn test_cmd(router: String) -> Result<String, String> {
    println!("Opening new window with router: {}", router);
    Ok("From Rust".to_string())
}

#[tauri::command]
async fn open_window(
    app: tauri::AppHandle,
    window: tauri::Window,
    router: String,
) -> Result<String, String> {
    println!(
        "Opening new window with router: {}|{}",
        router,
        window.label()
    );

    let _window = tauri::WebviewWindowBuilder::new(
        &app,
        router.as_str(),
        tauri::WebviewUrl::App(router.clone().into()),
    )
    .build()
    .map_err(|e| e.to_string())?;

    Ok(format!("Open Window {router}"))
}

#[tauri::command]
async fn resize_window_height(window: tauri::Window, height: f64) {
    #[cfg(windows)]
    {
        use crate::utils::monitor_ex::get_monitor_info_bywin;
        let monitor_info =
            get_monitor_info_bywin(window.hwnd().unwrap()).unwrap();
        // except the taskbar
        let work_height = monitor_info.rcWork.bottom - monitor_info.rcWork.top;
        let scala = window.scale_factor().unwrap();

        let outerpos =
            window.outer_position().unwrap().to_logical::<f64>(scala);
        let outersize = window.outer_size().unwrap().to_logical::<f64>(scala);
        let innersize = window.inner_size().unwrap().to_logical::<f64>(scala);

        let frame_height = outersize.height - innersize.height;

        // Check if the height overflow work_height
        let overflowh =
            outerpos.y + height + frame_height - work_height as f64 / scala;

        let set_height = if overflowh > 0.0 {
            height - overflowh
        } else {
            height
        };

        let _ = window
            .set_size(tauri::LogicalSize::new(innersize.width, set_height));
    }
}

pub fn register_cmds() -> Box<tauri::ipc::InvokeHandler<tauri::Wry>> {
    Box::new(tauri::generate_handler![
        test_cmd,
        open_window,
        crate::windows::translate::open_translate_window,
        crate::windows::translate::set_pin,
        resize_window_height,
        settings::get_settings,
        translate::translate_text,
        translate::translate_languages,
        translate::translate_detect,
        translate::translate_speech,
        translate::translate_img2text
    ])
}
