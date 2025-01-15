// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod actions;
mod config;
mod mouser;
mod taskbar;

use config::Config;
use std::{
    sync::{Arc, Mutex},
    thread,
};
use taskbar::{clear_taskbar_progress, update_taskbar_progress};
use tauri::{Manager, SystemTrayEvent};

struct AppState(Arc<Mutex<Config>>);

fn main() {
    let loaded_config = Config::load().unwrap_or_else(|e| {
        let config = Config::default();
        eprintln!("Could not load config: {}", e);
        config
    });
    println!("Using config: {:#?}", loaded_config);
    let config = Arc::new(Mutex::new(loaded_config));

    let tray_menu = tauri::SystemTrayMenu::new()
        .add_item(tauri::CustomMenuItem::new("hide".to_string(), "Show/Hide"))
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(tauri::CustomMenuItem::new("quit".to_string(), "Quit"));
    let tray = tauri::SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_speed,
            set_speed,
            get_config,
            update_taskbar_progress,
            clear_taskbar_progress
        ])
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                match app.get_window("main") {
                    Some(window) => actions::toggle_window(&window),
                    None => return,
                };
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "hide" => {
                    match app.get_window("main") {
                        Some(window) => actions::toggle_window(&window),
                        None => return,
                    };
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .manage(AppState(Arc::clone(&config)))
        .setup(move |app| {
            let window = app.get_window("main").unwrap();

            let speed_config_listener = Arc::clone(&config);
            let _speed_event = app.listen_global("speed_change", move |msg| {
                let speed: f32 = msg.payload().unwrap().parse().unwrap();
                let mut config = speed_config_listener.lock().unwrap();
                config.speed = speed;
            });

            let save_listener_config = Arc::clone(&config);
            let _save_event = app.listen_global("save_config", move |_| {
                let config = save_listener_config.lock().unwrap();
                config.save().unwrap();
            });

            let thread_config = Arc::clone(&config);
            thread::spawn(move || {
                mouser::start(window, thread_config).unwrap();
            });

            Ok(())
        })
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
                event.window().hide().unwrap();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_speed(state: tauri::State<AppState>) -> Result<f32, String> {
    let config = state.0.lock().unwrap();
    Ok(config.speed)
}

#[tauri::command]
fn set_speed(state: tauri::State<AppState>, speed: f32) {
    let mut config = state.0.lock().unwrap();
    config.speed = speed;
}

#[tauri::command]
fn get_config(state: tauri::State<AppState>) -> Result<Config, String> {
    let config = state.0.lock().unwrap();
    Ok(config.clone())
}
