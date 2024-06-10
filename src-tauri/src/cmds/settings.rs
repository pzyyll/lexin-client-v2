use crate::core::state::AppState;

#[tauri::command]
pub async fn get_settings<R: tauri::Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    Ok(state.settings.to_string())
}
