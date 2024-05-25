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
    .transparent(true)
    .build()
    .map_err(|e| e.to_string())?;

    Ok(format!("Open Window {router}"))
}

pub fn register_cmds() -> Box<tauri::ipc::InvokeHandler<tauri::Wry>> {
    Box::new(tauri::generate_handler![test_cmd, open_window])
}
