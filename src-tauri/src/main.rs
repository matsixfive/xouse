// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod actions;
mod config;
mod lua;
mod perform;
mod setup;

use config::Config;
use std::sync::{Arc, Mutex};

struct AppState {
    config: Arc<Mutex<Config>>,
}

fn main() {
    log::info!("starting tauri app");
    // use the default config
    // later will try to load the config from a file
    let config_mtx = Arc::new(Mutex::new(Config::default()));

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                // .target(tauri_plugin_log::Target::new(
                //     tauri_plugin_log::TargetKind::Stdout,
                // ))
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Webview,
                ))
                .level(log::LevelFilter::Info)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            save_config, get_speed, set_speed, get_config, timing
        ])
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
fn save_config(state: tauri::State<AppState>) -> Result<(), String> {
    let config = state.config.lock().map_err(|e| e.to_string())?;
    config.save().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn get_speed(state: tauri::State<AppState>) -> Result<f32, String> {
    let config = state.config.lock().map_err(|e| e.to_string())?;
    Ok(config.speed)
}

#[tauri::command]
fn set_speed(state: tauri::State<AppState>, speed: f32) -> Result<(), String> {
    let mut config = state.config.lock().map_err(|e| e.to_string())?;
    config.speed = speed;
    Ok(())
}

#[tauri::command]
fn get_config(state: tauri::State<AppState>) -> Result<Config, String> {
    let config = state.config.lock().map_err(|e| e.to_string())?;
    // log::info!("got config: {:?}", *config);
    Ok(config.clone())
}

#[tauri::command]
fn timing(time_in: String) -> Result<(String, String), String> {
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?;
    Ok((time_in, time.as_millis().to_string()))
}
