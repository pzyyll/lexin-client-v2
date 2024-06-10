//! Copyright: 2024 Lizc. All rights reserved.
//! License: MIT License
//! You may obtain a copy of the License at https://opensource.org/licenses/MIT
//!
//! Author: Lizc
//! Created Data: 2024-05-27
//!
//! Description: The consts module for the application.

#![allow(unused)]

pub const WIN_LABEL_MAIN: &str = "main";
pub const WIN_LABEL_TRANSLATE: &str = "translate";
pub const APP_NAME: &str = "Lexi Navigator";
pub const APP_DATA_CONFIG: &str = "settings.json";
pub const SERVER_API_KEY: &str = "lexinsvr";

pub enum WindowEvent {
    CPCP,
}

impl WindowEvent {
    pub fn to_string(&self) -> &str {
        match self {
            WindowEvent::CPCP => "cpcp",
        }
    }
}
