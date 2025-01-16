// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod actions;
mod actions2;
mod config;
mod mouser;
mod lua;

use config::Config;
use std::{
    sync::{Arc, Mutex},
    thread,
};
use tauri::{Emitter, Listener, Manager};

struct AppState(Arc<Mutex<Config>>);

fn main() {
    lua::test_lua().unwrap();
    // let loaded_config = Config::load().unwrap_or_else(|e| {
    //     let config = Config::default();
    //     eprintln!("Could not load config: {}", e);
    //     config
    // });
    // println!("Using config: {:#?}", loaded_config);
    // let config = Arc::new(Mutex::new(loaded_config));
    let config = Arc::new(Mutex::new(Config::default()));

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_speed, set_speed, get_config,])
        .manage(AppState(Arc::clone(&config)))
        .setup(move |app| {
            // load the config instead of using the default
            if let Ok(new_config) = Config::load(app.app_handle()) {
                let mut config_mtx = config.lock().unwrap();
                *config_mtx = new_config;
            } else {
                eprintln!("Could not load config");
            }

            let window = app.get_webview_window("main").unwrap();

            let speed_config_listener = Arc::clone(&config);
            let _speed_event = app.listen_any("speed_change", move |msg| {
                let speed: f32 = msg.payload().parse().unwrap();
                let mut config = speed_config_listener.lock().unwrap();
                config.speed = speed;
            });

            let save_listener_config = Arc::clone(&config);
            let _save_event = app.listen_any("save_config", move |_| {
                let config = save_listener_config.lock().unwrap();
                config.save().unwrap();
            });

            let thread_config = Arc::clone(&config);
            thread::spawn(move || {
                mouser::start(window, thread_config).unwrap();
            });

            let hide = tauri::menu::CheckMenuItemBuilder::with_id("hide", "Hide").build(app)?;
            let quit = tauri::menu::MenuItemBuilder::with_id("quit", "Quit").build(app)?;
            let tray_menu = tauri::menu::MenuBuilder::new(app)
                .items(&[&hide])
                .separator()
                .check("test", "Test")
                .items(&[&quit])
                .build()?;

            let tray = tauri::tray::TrayIconBuilder::new()
                .menu(&tray_menu)
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    "hide" => {
                        if let Some(webview_window) = app.get_webview_window("main") {
                            if let Ok(true) = webview_window.is_visible() {
                                let _ = webview_window.hide();
                            } else {
                                let _ = webview_window.show();
                                let _ = webview_window.set_focus();
                            }
                        }
                    }
                    "quit" => {
                        dbg!("Quit");
                        app.exit(0);
                    }
                    "test" => {
                        dbg!("Test");
                        dbg!(event);
                    }
                    _ => eprintln!("Unknown tray event: {:?}", event),
                })
                .on_tray_icon_event(|tray, event| {
                    if let tauri::tray::TrayIconEvent::Click {
                        button: tauri::tray::MouseButton::Left,
                        button_state: tauri::tray::MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(webview_window) = app.get_webview_window("main") {
                            let _ = webview_window.show();
                            let _ = webview_window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
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
