use crate::config::Config;
use crate::mouser;
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::{menu, Listener, Manager};

pub fn setup(
    config_mtx: Arc<Mutex<Config>>,
) -> impl FnOnce(&mut tauri::App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    move |app: &mut tauri::App| {
        // load the config instead of using the default
        if let Ok(new_config) = Config::load(app.app_handle()) {
            let mut config = config_mtx.lock().unwrap();
            *config = new_config;
            println!("Loaded config: {:?}", *config);
            log::info!("Loaded config: {:?}", *config);
            config.save().unwrap();
        } else {
            log::error!("Could not load config");
        }

        let speed_event_config = config_mtx.clone();
        let _speed_event = app.listen_any("speed_change", move |msg| {
            let speed: f32 = msg.payload().parse().unwrap();
            let mut config = speed_event_config.lock().unwrap();
            config.speed = speed;
        });

        let save_event_config = config_mtx.clone();
        let _save_event = app.listen_any("save_config", move |_| {
            let config = save_event_config.lock().unwrap();
            config.save().unwrap();
        });

        let webview_window = app.get_webview_window("main").unwrap();
        let thread_config = Arc::clone(&config_mtx);
        thread::spawn(move || {
            mouser::start(webview_window, thread_config).unwrap();
        });

        let hide = menu::CheckMenuItemBuilder::with_id("hide", "Hide").build(app)?;
        let quit = menu::MenuItemBuilder::with_id("quit", "Quit").build(app)?;
        let tray_menu = menu::MenuBuilder::new(app)
            .items(&[&hide])
            .separator()
            .check("test", "Test")
            .items(&[&quit])
            .build()?;

        let tray_icon_image =
            tauri::image::Image::new(include_bytes!("../icons/128x128.png"), 128, 128);

        let _tray = tauri::tray::TrayIconBuilder::new()
            .menu(&tray_menu)
            .icon(tray_icon_image)
            .on_menu_event(move |app, event| match event.id().as_ref() {
                "hide" => {
                    if let Some(webview_window) = app.get_webview_window("main") {
                        if hide.is_checked().unwrap_or(false) {
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
    }
}
