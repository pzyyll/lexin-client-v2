//! Copyright: 2024 Lizc. All rights reserved.
//! License: MIT License
//! You may obtain a copy of the License at https://opensource.org/licenses/MIT
//!
//! Author: Lizc
//! Created Data: 2024-06-06
//!
//! Description: Translate.

#![allow(unused)]

use crate::consts::SERVER_API_KEY;
use crate::AppState;
use base64::prelude::*;
use futures::StreamExt;
use lazy_static::lazy_static;
use serde_json::json;
use tauri::Runtime;

lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

enum TranslatePath {
    Text,
    Languages,
    Detect,
    Speech,
    ImgToText,
}

impl TranslatePath {
    fn to_path(&self) -> String {
        match self {
            TranslatePath::Text => "/api/translate/text".to_string(),
            TranslatePath::Languages => "api/translate/languages".to_string(),
            TranslatePath::Detect => "/api/translate/detect".to_string(),
            TranslatePath::Speech => {
                "/api/translate/text-to-speech".to_string()
            }
            TranslatePath::ImgToText => {
                "/api/translate/img-to-text".to_string()
            }
        }
    }

    fn join_url(&self, url: &str) -> String {
        reqwest::Url::parse(url)
            .map(|u| u.join(&self.to_path()).unwrap())
            .unwrap()
            .to_string()
    }
}

#[tauri::command]
pub async fn translate_text(
    state: tauri::State<'_, AppState>,
    api_type: String,
    text: String,
    from: String,
    to: String,
) -> Result<serde_json::Value, String> {
    if let Some(api) = state.settings.get_api(SERVER_API_KEY) {
        let url = api.url.clone();
        let token = api.token.clone();
        // println!("{:?}", api);
        // println!("api_type: {api_type}, text: {text}, from: {from}, to: {to}");

        let payload = json!({
            "text": text,
            "source_language": from,
            "target_language": to,
            "api_type": api_type
        });

        let res = CLIENT
            .post(TranslatePath::Text.join_url(&url))
            .bearer_auth(token)
            .json(&payload)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let js = res
            .json::<serde_json::Value>()
            .await
            .map_err(|e| e.to_string())?;
        return Ok(js);
    }

    Err("No API found".to_string())
}

#[tauri::command]
pub async fn translate_languages(
    state: tauri::State<'_, AppState>,
    api_type: String,
    display_language_code: String,
) -> Result<serde_json::Value, String> {
    if let Some(api) = state.settings.get_api(SERVER_API_KEY) {
        let url = api.url.clone();
        let token = api.token.clone();

        let res = CLIENT
            .get(TranslatePath::Languages.join_url(&url))
            .bearer_auth(token)
            .query(&[("dlc", display_language_code), ("api_type", api_type)])
            .send()
            .await
            .map_err(|e| e.to_string())?;
        return res
            .json::<serde_json::Value>()
            .await
            .map_err(|e| e.to_string());
    }

    Err("Not api found".to_string())
}

#[tauri::command]
pub async fn translate_detect(
    state: tauri::State<'_, AppState>,
    api_type: String,
    text: String,
) -> Result<serde_json::Value, String> {
    if let Some(api) = state.settings.get_api(SERVER_API_KEY) {
        let url = api.url.clone();
        let token = api.token.clone();

        return CLIENT
            .get(TranslatePath::Detect.join_url(&url))
            .bearer_auth(token)
            .query(&[("text", text), ("api_type", api_type)])
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json::<serde_json::Value>()
            .await
            .map_err(|e| e.to_string());
    }
    Ok("".into())
}

#[tauri::command]
pub async fn translate_speech(
    state: tauri::State<'_, AppState>,
    text: String,
    lang: String,
) -> Result<serde_json::Value, String> {
    if let Some(api) = state.settings.get_api(SERVER_API_KEY) {
        let url = api.url.clone();
        let token = api.token.clone();
        let res = CLIENT
            .get(TranslatePath::Speech.join_url(&url))
            .bearer_auth(token)
            .query(&[("text", text), ("language_code", lang)])
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if res.status().is_success() {
            let bytes = res.bytes().await.map_err(|e| e.to_string())?;
            return Ok(json!({
                "data": bytes
            }));
        }
        return Err(format!("Error: {:?}", res.text().await));
    }
    Err("Not Api Found".into())
}

#[tauri::command]
pub async fn translate_img2text(
    state: tauri::State<'_, AppState>,
    api_type: String,
    img_bytes: Vec<u8>,
) -> Result<serde_json::Value, String> {
    // println!("{:?}", img_bytes);

    if let Some(api) = state.settings.get_api(SERVER_API_KEY) {
        let url = api.url.clone();
        let token = api.token.clone();

        let form = reqwest::multipart::Form::new().part(
            "file",
            reqwest::multipart::Part::bytes(img_bytes).file_name("img.png"),
        );
        let res = CLIENT
            .post(TranslatePath::ImgToText.join_url(&url))
            .bearer_auth(token)
            .multipart(form)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if res.status().is_success() {
            return res
                .json::<serde_json::Value>()
                .await
                .map_err(|e| e.to_string());
        }
        return Err(format!("Error: {:?}", res.text().await));
    }

    Err("No Image Found".to_string())
}
