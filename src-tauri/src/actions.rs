use crate::config::Config;
use gilrs::Button;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tauri::{Emitter, WebviewWindow};
use thiserror::Error;
use windows::Win32::UI::Input::KeyboardAndMouse as kbm;

#[derive(Clone, Debug)]
pub struct ActionMap {
    map: HashMap<Button, Vec<Action>>,
}

static EMPTY_ACTIONS: Vec<Action> = vec![];
impl std::ops::Index<Button> for ActionMap {
    type Output = Vec<Action>;

    fn index(&self, button: Button) -> &Self::Output {
        self.map.get(&button).unwrap_or(&EMPTY_ACTIONS)
    }
}

impl Default for ActionMap {
    fn default() -> Self {
        let map = HashMap::from([
            (
                Button::South,
                vec![Action::UpDown(UpDownAction::LuaScript {
                    script: String::from("print(number)\nnumber = number + 1"),
                })],
                // vec![Action::UpDown(UpDownAction::Click(MouseButton::Left))],
            ),
            (
                Button::East,
                vec![Action::UpDown(UpDownAction::Click(MouseButton::Right))],
            ),
            (
                Button::North,
                vec![Action::UpDown(UpDownAction::KeyPress {
                    key: rdev::Key::Space,
                    modifiers: vec![],
                })],
            ),
            (
                Button::DPadUp,
                vec![
                    Action::Simple(SimpleAction::SpeedInc),
                    Action::Simple(SimpleAction::Rumble),
                ],
            ),
            (
                Button::DPadDown,
                vec![
                    Action::Simple(SimpleAction::SpeedDec),
                    Action::Simple(SimpleAction::Rumble),
                ],
            ),
            (
                Button::RightTrigger,
                vec![
                    Action::UpDown(UpDownAction::KeyPress {
                        key: rdev::Key::Tab,
                        modifiers: vec![ModifierKey::Ctrl],
                    }),
                    Action::Simple(SimpleAction::Rumble),
                ],
            ),
            (
                Button::LeftTrigger,
                vec![
                    Action::UpDown(UpDownAction::KeyPress {
                        key: rdev::Key::Tab,
                        modifiers: vec![ModifierKey::Ctrl, ModifierKey::Shift],
                    }),
                    Action::Simple(SimpleAction::Rumble),
                ],
            ),
            (
                Button::RightTrigger2,
                vec![Action::UpDown(UpDownAction::SpeedUp)],
            ),
            (
                Button::LeftTrigger2,
                vec![Action::UpDown(UpDownAction::SpeedDown)],
            ),
            (
                Button::Select,
                vec![Action::Simple(SimpleAction::ToggleVis)],
            ),
        ]);
        Self { map }
    }
}

impl serde::Serialize for ActionMap {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = HashMap::new();
        for (button, action) in &self.map {
            map.insert(serialize_button(button), action);
        }
        map.serialize(serializer)
    }
}

impl<'d> serde::Deserialize<'d> for ActionMap {
    fn deserialize<D: serde::Deserializer<'d>>(deserializer: D) -> Result<Self, D::Error> {
        let map = HashMap::<String, Vec<Action>>::deserialize(deserializer)?;
        let mut action_map = HashMap::new();
        for (button, action) in map {
            action_map.insert(deserialize_button(button), action);
        }
        Ok(Self { map: action_map })
    }
}

pub fn serialize_button(button: &Button) -> &'static str {
    match button {
        Button::North => "North",
        Button::East => "East",
        Button::South => "South",
        Button::West => "West",
        Button::DPadUp => "DPadUp",
        Button::DPadRight => "DPadRight",
        Button::DPadDown => "DPadDown",
        Button::DPadLeft => "DPadLeft",
        Button::LeftTrigger2 => "LeftTrigger",
        Button::RightTrigger2 => "RightTrigger",
        Button::LeftTrigger => "LeftBumper",
        Button::RightTrigger => "RightBumper",
        Button::LeftThumb => "LeftThumb",
        Button::RightThumb => "RightThumb",
        Button::Start => "Start",
        Button::Select => "Select",
        _ => "Unknown",
    }
}

pub fn deserialize_button(button: String) -> Button {
    match button.as_str() {
        "North" => Button::North,
        "East" => Button::East,
        "South" => Button::South,
        "West" => Button::West,
        "DPadUp" => Button::DPadUp,
        "DPadRight" => Button::DPadRight,
        "DPadDown" => Button::DPadDown,
        "DPadLeft" => Button::DPadLeft,
        "LeftTrigger" => Button::LeftTrigger2,
        "RightTrigger" => Button::RightTrigger2,
        "LeftBumper" => Button::LeftTrigger,
        "RightBumper" => Button::RightTrigger,
        "LeftThumb" => Button::LeftThumb,
        "RightThumb" => Button::RightThumb,
        "Start" => Button::Start,
        "Select" => Button::Select,
        _ => Button::Unknown,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    Simple(SimpleAction), // Triggered on button press
    UpDown(UpDownAction), // Triggered on button press and release
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum SimpleAction {
    SpeedInc,
    SpeedDec,
    Rumble,
    ToggleVis,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum UpDownAction {
    Click(MouseButton),
    SpeedUp,
    SpeedDown,
    KeyPress {
        key: rdev::Key,
        modifiers: Vec<ModifierKey>,
    },
    LuaScript {
        script: String,
    },
}

pub trait Rumbler {
    fn rumble(&self) -> Result<(), gilrs::Error>;
}

pub struct ActionInterface<'lua> {
    pub config: Arc<Mutex<Config>>,
    pub window: WebviewWindow,
    pub lua: &'lua mlua::Lua,
    pub rumble: Option<Box<dyn Rumbler>>,
}

#[derive(Error, Debug)]
pub enum ActionError {
    #[error("Lua error: {0}")]
    Lua(#[from] mlua::Error),

    #[error("Keypress error: {0}")]
    Keypress(#[from] rdev::SimulateError),

    #[error("Rumble error: {0}")]
    Rumble(#[from] gilrs::Error),

    #[error("Tauri error: {0}")]
    Tauri(#[from] tauri::Error),

    #[error("Other error: {0}")]
    Other(String),
}

pub trait SimpleActionFn {
    fn call(&self, state: &ActionInterface) -> Result<(), ActionError>;
}

impl SimpleActionFn for SimpleAction {
    fn call(&self, interface: &ActionInterface) -> Result<(), ActionError> {
        match self {
            SimpleAction::SpeedInc => {
                log::info!(target: "actions", "speed inc");
                let config = &mut *interface.config.lock().unwrap();
                config.speed += config.speed_step;
            }
            SimpleAction::SpeedDec => {
                log::info!(target: "actions", "speed dec");
                let config = &mut *interface.config.lock().unwrap();
                if config.speed > config.speed_step {
                    config.speed -= config.speed_step;
                }
            }
            SimpleAction::Rumble => {
                log::info!(target: "actions", "rumble");
                if let Some(rumbler) = &interface.rumble {
                    rumbler.rumble()?;
                } else {
                    return Err(ActionError::Other("no rumbler".to_string()));
                }
            }
            SimpleAction::ToggleVis => {
                log::info!(target: "actions", "toggle vis");
                let webview_window = &interface.window;
                if let Ok(true) = webview_window.is_visible() {
                    webview_window.hide()?;
                } else {
                    webview_window.show()?;
                    webview_window.set_focus()?;
                }
            }
        }
        Ok(())
    }
}

pub trait UpDownActionFn {
    fn down(&self, interface: &ActionInterface) -> Result<(), ActionError>;
    fn up(&self, interface: &ActionInterface) -> Result<(), ActionError>;
}

impl UpDownActionFn for UpDownAction {
    fn down(&self, interface: &ActionInterface) -> Result<(), ActionError> {
        match self {
            UpDownAction::Click(button) => match button {
                MouseButton::Left => {
                    log::info!(target: "actions", "left click");
                    unsafe {
                        kbm::mouse_event(kbm::MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
                    }
                }
                MouseButton::Right => {
                    log::info!(target: "actions", "right click");
                    unsafe {
                        kbm::mouse_event(kbm::MOUSEEVENTF_RIGHTDOWN, 0, 0, 0, 0);
                    }
                }
                MouseButton::Middle => {
                    log::info!(target: "actions", "middle click");
                    unsafe {
                        kbm::mouse_event(kbm::MOUSEEVENTF_MIDDLEDOWN, 0, 0, 0, 0);
                    }
                }
            },
            UpDownAction::SpeedUp => {
                log::info!(target: "actions", "speed up");
                let config = &mut *interface.config.lock().unwrap();
                config.speed_mult *= config.speed_up;
            }
            UpDownAction::SpeedDown => {
                log::info!(target: "actions", "speed down");
                let config = &mut *interface.config.lock().unwrap();
                config.speed_mult /= config.speed_up;
            }
            UpDownAction::KeyPress { key, modifiers } => {
                log::info!(target: "actions", "pressing {:?} with modifiers {:?}", key, modifiers);
                for modifier in modifiers {
                    rdev::simulate(&rdev::EventType::KeyPress(modifier.into()))?;
                }

                rdev::simulate(&rdev::EventType::KeyPress(*key))?;
            }
            UpDownAction::LuaScript { script } => {
                log::info!(target: "actions", "running lua script");
                interface.lua.load(script.as_str()).exec()?;
            }
        }
        Ok(())
    }

    fn up(&self, interface: &ActionInterface) -> Result<(), ActionError> {
        match self {
            UpDownAction::Click(button) => match button {
                MouseButton::Left => unsafe {
                    kbm::mouse_event(kbm::MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
                },
                MouseButton::Right => unsafe {
                    kbm::mouse_event(kbm::MOUSEEVENTF_RIGHTUP, 0, 0, 0, 0);
                },
                MouseButton::Middle => unsafe {
                    kbm::mouse_event(kbm::MOUSEEVENTF_MIDDLEUP, 0, 0, 0, 0);
                },
            },
            UpDownAction::SpeedUp => {
                let config = &mut *interface.config.lock().unwrap();
                config.speed_mult /= config.speed_up;
            }
            UpDownAction::SpeedDown => {
                let config = &mut *interface.config.lock().unwrap();
                config.speed_mult *= config.speed_up;
            }
            UpDownAction::KeyPress { key, modifiers } => {
                rdev::simulate(&rdev::EventType::KeyRelease(*key))?;

                for modifier in modifiers.iter().rev() {
                    rdev::simulate(&rdev::EventType::KeyRelease(modifier.into()))?;
                }
            }
            UpDownAction::LuaScript { script } => {}
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub enum ModifierKey {
    Alt,
    Ctrl,
    Win,
    Shift,
}

impl Into<rdev::Key> for ModifierKey {
    fn into(self) -> rdev::Key {
        <&ModifierKey>::into(&self)
    }
}

impl Into<rdev::Key> for &ModifierKey {
    fn into(self) -> rdev::Key {
        match self {
            ModifierKey::Alt => rdev::Key::Alt,
            ModifierKey::Ctrl => rdev::Key::ControlLeft,
            ModifierKey::Win => rdev::Key::MetaLeft,
            ModifierKey::Shift => rdev::Key::ShiftLeft,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}
