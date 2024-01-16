// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod actions;
mod config;
mod mouser;

use config::Config;
use std::{
    sync::{Arc, Mutex},
    thread,
};
use tauri::{Manager, SystemTrayEvent};

fn main() {
    let loaded_config = Config::load();
    println!("Loaded config: {:?}", loaded_config);
    let config = Arc::new(Mutex::new(loaded_config.unwrap_or_default()));

    let menu_hide = tauri::CustomMenuItem::new("hide".to_string(), "Show/Hide");
    let menu_quit = tauri::CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = tauri::SystemTrayMenu::new()
        .add_item(menu_hide)
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(menu_quit);
    let tray = tauri::SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                let window = match app.get_window("main") {
                    Some(window) => match window.is_visible().expect("winvis") {
                        true => {
                            window.hide().expect("winhide");
                            return;
                        }
                        false => window,
                    },
                    None => return,
                };
                window.show().unwrap();
                window.set_focus().unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "hide" => {
                    let window = match app.get_window("main") {
                        Some(window) => match window.is_visible().expect("winvis") {
                            true => {
                                // hide the window instead of closing due to processes not closing memory leak: https://github.com/tauri-apps/wry/issues/590
                                window.hide().expect("winhide");
                                // window.close().expect("winclose");
                                return;
                            }
                            false => window,
                        },
                        None => return,
                    };
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        // .manage(config)
        .setup(move |app| {
            let window = app.get_window("main").unwrap();

            let config_listener = Arc::clone(&config);
            let speed_event = app.listen_global("speed-change", move |msg| {
                let speed: f32 = msg.payload().unwrap().parse().unwrap();
                let mut config = config_listener.lock().unwrap();
                config.speed = speed;
            });

            let config_listener = Arc::clone(&config);
            let speed_event = app.listen_global("save-config", move |_| {
                let mut config = config_listener.lock().unwrap();
                config.save().unwrap();
            });

            let config_thread = Arc::clone(&config);
            thread::spawn(move || {
                mouser::start(window, config_thread).unwrap();
            });

            Ok(())
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
