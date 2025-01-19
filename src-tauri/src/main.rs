// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// mod actions;
mod actions;
mod config;
mod lua;
mod mouser;
mod setup;

use config::Config;
use std::{
    sync::{Arc, Mutex},
};

struct AppState {
    config: Arc<Mutex<Config>>,
}

fn main() {
    // use the default config
    // later will try to load the config from a file
    let config_mtx = Arc::new(Mutex::new(Config::default()));

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Stdout,
                ))
                .build(),
        )
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Webview,
                ))
                .build(),
        )
        .invoke_handler(tauri::generate_handler![get_speed, set_speed, get_config,])
        .manage(AppState {
            config: Arc::clone(&config_mtx),
        })
        .setup(setup::setup(config_mtx))
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_speed(state: tauri::State<AppState>) -> Result<f32, String> {
    let config = state.config.lock().unwrap();
    Ok(config.speed)
}

#[tauri::command]
fn set_speed(state: tauri::State<AppState>, speed: f32) {
    let mut config = state.config.lock().unwrap();
    config.speed = speed;
}

#[tauri::command]
fn get_config(state: tauri::State<AppState>) -> Result<Config, String> {
    let config = state.config.lock().unwrap();
    Ok(config.clone())
}
