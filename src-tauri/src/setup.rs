use crate::config::Config;
use crate::perform;
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::menu::MenuId;
use tauri::{menu, Listener, Manager};

pub enum MenuButton {
    Hide,
    Quit,
}

impl From<MenuButton> for MenuId {
    fn from(val: MenuButton) -> Self {
        match val {
            MenuButton::Hide => "hide".into(),
            MenuButton::Quit => "quit".into(),
        }
    }
}

impl TryFrom<&MenuId> for MenuButton {
    type Error = &'static str;

    fn try_from(value: &MenuId) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "hide" => Ok(MenuButton::Hide),
            "quit" => Ok(MenuButton::Quit),
            _ => Err("Unknown menu button"),
        }
    }
}

pub fn setup(
    config_mtx: Arc<Mutex<Config>>,
) -> impl FnOnce(&mut tauri::App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    move |app: &mut tauri::App| {
        let mut config = config_mtx.lock().unwrap();
        config.set_config_dir(app.app_handle());
        drop(config);

        // load the config instead of using the default
        match Config::load(app.app_handle()) {
            Ok(new_config) => {
                let mut config = config_mtx.lock().unwrap();
                *config = new_config;
                log::info!("Loaded config: {:?}", *config);
                config.save().unwrap();
            }
            Err(e) => {
                log::error!("Could not load config {:?}", e);
                log::info!("Using default config");
                if let Err(e) = config_mtx.lock().unwrap().save() {
                    log::error!("Could not save default config {:?}", e);
                }
            }
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
            perform::start(webview_window, thread_config).unwrap();
        });

        let hide = menu::CheckMenuItemBuilder::with_id(MenuButton::Hide, "Hide").build(app)?;
        let quit = menu::MenuItemBuilder::with_id(MenuButton::Quit, "Quit").build(app)?;
        let tray_menu = menu::MenuBuilder::new(app)
            .items(&[&hide])
            .separator()
            .items(&[&quit])
            .build()?;

        let tray_icon_image =
            tauri::image::Image::new(include_bytes!("../icons/128x128.png"), 128, 128);

        let _tray = tauri::tray::TrayIconBuilder::new()
            .menu(&tray_menu)
            .icon(tray_icon_image)
            .on_menu_event(move |app, event| match event.id().try_into() {
                Ok(MenuButton::Hide) => {
                    if let Some(webview_window) = app.get_webview_window("main") {
                        if hide.is_checked().unwrap_or(false) {
                            let _ = webview_window.hide();
                        } else {
                            let _ = webview_window.show();
                            let _ = webview_window.set_focus();
                        }
                    }
                }
                Ok(MenuButton::Quit) => {
                    log::debug!("Quitting...");
                    app.exit(0);
                }
                _ => unreachable!(),
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
