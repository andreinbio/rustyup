
extern crate serde;

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::config::Config;

const APP_INFO: AppInfo = AppInfo{name: "preferences", author: "Rust language community"};
const APP_KEY: &str = "rustyup/settings";

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AppSettings {
    position: (i32, i32),
    size: (u32, u32),
}

pub fn load() -> AppSettings {
    AppSettings::load();
}

pub fn save(settings: AppSettings) {

}
