//! Copyright: 2024 Lizc. All rights reserved.
//! License: MIT License
//! You may obtain a copy of the License at https://opensource.org/licenses/MIT
//!
//! Author: Lizc
//! Created Data: 2024-06-05
//!
//! Description: settings.

#![allow(unused)]

use crate::consts::APP_DATA_CONFIG;
use crate::utils::path::ensure_file_exists;
use lazy_static::lazy_static;
use serde::{de::value::Error, Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error as StdError;
use std::sync::Mutex;
use tauri::path::BaseDirectory;

static mut SETTINGS: Mutex<Option<&mut Settings>> = Mutex::new(None);

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiInfo {
    pub url: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default = "HashMap::new")]
    pub api: HashMap<String, ApiInfo>,
    #[serde(default = "String::new", skip)]
    file: String,
}

fn default_version() -> String {
    "0.1.0".to_string()
}

impl Settings {
    pub fn new() -> Self {
        Self {
            version: default_version(),
            api: HashMap::new(),
            file: String::new(),
        }
    }

    pub fn load_from_file(file: &str) -> Result<Self, Box<dyn StdError>> {
        ensure_file_exists(&file)?;
        let data = std::fs::read_to_string(file.to_string())?;
        let mut settings: Settings = serde_json::from_str(&data)?;
        settings.file = file.to_string();
        Ok(settings)
    }

    pub fn save(&self) {
        let data = serde_json::to_string_pretty(&self).unwrap();
        std::fs::write(self.file.clone(), data).unwrap();
    }

    pub fn add_api(&mut self, name: &str, url: &str, token: &str) {
        let api = ApiInfo {
            url: url.to_string(),
            token: token.to_string(),
        };
        self.api.insert(name.to_string(), api);
    }

    pub fn get_api(&self, name: &str) -> Option<&ApiInfo> {
        self.api.get(name)
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}
