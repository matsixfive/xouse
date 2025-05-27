use crate::config::Config;
use gilrs::Button;
use mouce::MouseActions;
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tauri::{Manager, WebviewWindow};
use thiserror::Error;

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
                vec![Action::LuaScript {
                    script: LuaScript("test.lua".to_string()),
                }],
                // vec![Action::Click(MouseButton::Left)],
            ),
            (Button::East, vec![Action::Click(MouseButton::Right)]),
            (
                Button::North,
                vec![Action::KeyPress {
                    key: rdev::Key::Space,
                    modifiers: vec![],
                }],
            ),
            (Button::DPadUp, vec![Action::SpeedInc, Action::Rumble]),
            (Button::DPadDown, vec![Action::SpeedDec, Action::Rumble]),
            (
                Button::RightTrigger,
                vec![
                    Action::KeyPress {
                        key: rdev::Key::Tab,
                        modifiers: vec![ModifierKey::Ctrl],
                    },
                    Action::Rumble,
                ],
            ),
            (
                Button::LeftTrigger,
                vec![
                    Action::KeyPress {
                        key: rdev::Key::Tab,
                        modifiers: vec![ModifierKey::Ctrl, ModifierKey::Shift],
                    },
                    Action::Rumble,
                ],
            ),
            (Button::RightTrigger2, vec![Action::SpeedUp]),
            (Button::LeftTrigger2, vec![Action::SpeedDown]),
            (Button::Select, vec![Action::ToggleVis]),
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LuaScript(String);

impl LuaScript {
    pub fn contents(&self, config_dir: PathBuf) -> Result<String, std::io::Error> {
        let filename = self.0.as_str();
        let path = config_dir
            .join("scripts")
            .join(filename)
            .with_extension("lua");
        std::fs::read_to_string(path)
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum Action {
    SpeedInc,
    SpeedDec,
    Rumble,
    ToggleVis,
    Click(MouseButton),
    SpeedUp,
    SpeedDown,
    SetSpeed(f32),
    KeyPress {
        key: rdev::Key,
        modifiers: Vec<ModifierKey>,
    },
    LuaScript {
        script: LuaScript,
    },
}

//TODO: rumble implementation is stupid (uses two boxes etc)
pub struct ActionInterface<'lua, R>
where
    R: Fn() -> Result<(), gilrs::ff::Error> + Send + Sync + 'static,
{
    pub config: Arc<Mutex<Config>>,
    pub window: WebviewWindow,
    pub rumble: Option<Rumble<R>>,
    pub lua: Option<&'lua mlua::Lua>,
}

#[derive(Error, Debug)]
pub enum ActionError {
    #[error("Lua error: {0}")]
    Lua(#[from] mlua::Error),

    #[error("Keypress error: {0}")]
    Keypress(#[from] rdev::SimulateError),

    #[error("Rumble error: {0}")]
    Rumble(#[from] gilrs::ff::Error),

    #[error("Tauri error: {0}")]
    Tauri(#[from] tauri::Error),

    #[error("Mouce error: {0}")]
    Mouce(#[from] mouce::error::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Other error: {0}")]
    Other(String),
}

pub trait ActionFn<R>
where
    R: Fn() -> Result<(), gilrs::ff::Error> + Send + Sync + 'static,
{
    fn down(&self, interface: &ActionInterface<R>) -> Result<(), ActionError>;
    fn up(&self, interface: &ActionInterface<R>) -> Result<(), ActionError>;
}

impl<R> ActionFn<R> for Action
where
    R: Fn() -> Result<(), gilrs::ff::Error> + Send + Sync + 'static,
{
    fn down(&self, interface: &ActionInterface<R>) -> Result<(), ActionError> {
        log::info!(target: "actions", "action down: {:?}", self);
        match self {
            Action::SpeedInc => {
                log::info!(target: "actions", "speed inc");
                let config = &mut *interface.config.lock().unwrap();
                config.speed += config.speed_step;
            }
            Action::SpeedDec => {
                log::info!(target: "actions", "speed dec");
                let config = &mut *interface.config.lock().unwrap();
                if config.speed > config.speed_step {
                    config.speed -= config.speed_step;
                }
            }
            Action::Rumble => {
                log::info!(target: "actions", "rumble");
                if let Some(rumbler) = &interface.rumble {
                    (rumbler.rumble)()?;
                } else {
                    return Err(ActionError::Other("no rumbler".to_string()));
                }
            }
            Action::ToggleVis => {
                log::info!(target: "actions", "toggle vis");
                let webview_window = &interface.window;
                if let Ok(true) = webview_window.is_visible() {
                    webview_window.hide()?;
                } else {
                    webview_window.show()?;
                    webview_window.set_focus()?;
                }
            }
            Action::SpeedUp => {
                log::info!(target: "actions", "speed up");
                let config = &mut *interface.config.lock().unwrap();
                config.speed_mult *= config.speed_up;
            }
            Action::SpeedDown => {
                log::info!(target: "actions", "speed down");
                let config = &mut *interface.config.lock().unwrap();
                config.speed_mult /= config.speed_up;
            }
            Action::SetSpeed(speed) => {
                log::info!(target: "actions", "set speed to {}", speed);
                let config = &mut *interface.config.lock().unwrap();
                config.speed = *speed;
            }
            Action::Click(button) => mouce::Mouse::new().press_button(&button.into())?,
            Action::KeyPress { key, modifiers } => {
                log::info!(target: "actions", "pressing {:?} with modifiers {:?}", key, modifiers);
                for modifier in modifiers {
                    rdev::simulate(&rdev::EventType::KeyPress(modifier.into()))?;
                }

                rdev::simulate(&rdev::EventType::KeyPress(*key))?;
            }
            Action::LuaScript { script } => {
                if let Some(l) = interface.lua {
                    let config_dir = Config::config_dir(interface.window.app_handle());
                    l.load(script.contents(config_dir)?.as_str()).exec()?;
                }
            }
        }
        Ok(())
    }

    fn up(&self, interface: &ActionInterface<R>) -> Result<(), ActionError> {
        log::info!(target: "actions", "action up: {:?}", self);
        match self {
            Action::SpeedUp => {
                let config = &mut *interface.config.lock().unwrap();
                config.speed_mult /= config.speed_up;
            }
            Action::SpeedDown => {
                let config = &mut *interface.config.lock().unwrap();
                config.speed_mult *= config.speed_up;
            }
            Action::Click(button) => mouce::Mouse::new().release_button(&button.into())?,
            Action::KeyPress { key, modifiers } => {
                rdev::simulate(&rdev::EventType::KeyRelease(*key))?;

                for modifier in modifiers.iter().rev() {
                    rdev::simulate(&rdev::EventType::KeyRelease(modifier.into()))?;
                }
            }
            Action::LuaScript { .. } => {}
            Action::SpeedInc
            | Action::SpeedDec
            | Action::SetSpeed(_)
            | Action::Rumble
            | Action::ToggleVis => {}
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

impl From<ModifierKey> for rdev::Key {
    fn from(key: ModifierKey) -> rdev::Key {
        match key {
            ModifierKey::Alt => rdev::Key::Alt,
            ModifierKey::Ctrl => rdev::Key::ControlLeft,
            ModifierKey::Win => rdev::Key::MetaLeft,
            ModifierKey::Shift => rdev::Key::ShiftLeft,
        }
    }
}

impl From<&ModifierKey> for rdev::Key {
    fn from(key: &ModifierKey) -> rdev::Key {
        <ModifierKey>::into(*key)
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

impl From<&MouseButton> for mouce::common::MouseButton {
    fn from(button: &MouseButton) -> mouce::common::MouseButton {
        match button {
            MouseButton::Left => mouce::common::MouseButton::Left,
            MouseButton::Right => mouce::common::MouseButton::Right,
            MouseButton::Middle => mouce::common::MouseButton::Middle,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Rumble<R>
where
    R: Fn() -> Result<(), gilrs::ff::Error> + Send + Sync + 'static,
{
    rumble: Box<R>,
}

impl<R> Rumble<R>
where
    R: Fn() -> Result<(), gilrs::ff::Error> + Send + Sync + 'static,
{
    pub fn new(func: R) -> Self {
        let rumble = Box::new(func);
        Self { rumble }
    }
}
