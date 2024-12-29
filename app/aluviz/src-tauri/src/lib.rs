use std::{
    fs::{self, File},
    path::Path,
    sync::Once,
};

use log::LevelFilter;
use simplelog::{Config, WriteLogger};

mod managers;
mod memory;
mod processors;
mod simulators;

#[cfg(test)]
mod tests;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

static INIT_LOGGER_ONCE: Once = Once::new();

fn init_file_logger() {
    INIT_LOGGER_ONCE.call_once(|| {
        let path = "./logs";
        let path = Path::new(path);
        if !path.is_dir() {
            fs::create_dir(path).expect("cannot create log folder");
        }
        WriteLogger::init(
            LevelFilter::Debug,
            Config::default(),
            File::create("./logs/tests.log")
                .expect("App should have permission to create this log file here."),
        )
        .unwrap();
    });
}
