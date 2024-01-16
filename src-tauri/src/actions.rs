use crate::config::Config;
use gilrs::Button;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tauri::Window;
use windows::Win32::UI::Input::KeyboardAndMouse as kbm;

pub type ActionFn =
    Box<dyn Fn(Arc<Mutex<Config>>, &Window, Arc<Box<dyn Fn() + Send + Sync>>) + Send + Sync>;

#[derive(Clone)]
pub enum ActionType {
    Simple(Arc<ActionFn>),                  // Triggered on button press
    UpDown((Arc<ActionFn>, Arc<ActionFn>)), // Triggered on button press and release
}

#[derive(Debug)]
pub struct ActionMap(pub HashMap<Button, Action>);

impl Default for ActionMap {
    fn default() -> Self {
        let map = HashMap::from([
            (Button::South, Action::LClick),
            (Button::East, Action::RClick),
            (Button::DPadUp, Action::SpeedInc),
            (Button::DPadDown, Action::SpeedDec),
            (Button::RightTrigger2, Action::SpeedUp),
            (Button::LeftTrigger2, Action::SpeedDown),
        ]);
        Self(map)
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum Action {
    LClick,
    RClick,
    MClick,
    SpeedUp,
    SpeedDown,
    SpeedInc,
    SpeedDec,
}

impl serde::Serialize for ActionMap {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = HashMap::new();
        for (button, action) in &self.0 {
            map.insert(serialize_button(button), action);
        }
        map.serialize(serializer)
    }
}

impl<'d> serde::Deserialize<'d> for ActionMap {
    fn deserialize<D: serde::Deserializer<'d>>(deserializer: D) -> Result<Self, D::Error> {
        let map = HashMap::<String, Action>::deserialize(deserializer)?;
        let mut action_map = HashMap::new();
        for (button, action) in map {
            action_map.insert(deserialize_button(button), action);
        }
        Ok(Self(action_map))
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

impl Into<ActionType> for Action {
    fn into(self) -> ActionType {
        match self {
            Action::LClick => ActionType::UpDown((
                Arc::new(Box::new(|_, _, _| unsafe {
                    kbm::mouse_event(kbm::MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
                })),
                Arc::new(Box::new(|_, _, _| unsafe {
                    kbm::mouse_event(kbm::MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
                })),
            )),
            Action::RClick => ActionType::UpDown((
                Arc::new(Box::new(|_, _, _| unsafe {
                    kbm::mouse_event(kbm::MOUSEEVENTF_RIGHTDOWN, 0, 0, 0, 0);
                })),
                Arc::new(Box::new(|_, _, _| unsafe {
                    kbm::mouse_event(kbm::MOUSEEVENTF_RIGHTUP, 0, 0, 0, 0);
                })),
            )),
            Action::SpeedUp => ActionType::UpDown((
                Arc::new(Box::new(|config, _, _| {
                    let config = &mut *config.lock().unwrap();
                    config.speed_mult *= config.speed_up;
                })),
                Arc::new(Box::new(|config, _, _| {
                    let config = &mut *config.lock().unwrap();
                    config.speed_mult /= config.speed_up;
                })),
            )),
            Action::SpeedDown => ActionType::UpDown((
                Arc::new(Box::new(|config, _, _| {
                    let config = &mut *config.lock().unwrap();
                    config.speed_mult /= config.speed_down;
                })),
                Arc::new(Box::new(|config, _, _| {
                    let config = &mut *config.lock().unwrap();
                    config.speed_mult *= config.speed_down;
                })),
            )),
            Action::SpeedInc => ActionType::Simple(Arc::new(Box::new(|config, window, rumble| {
                let config = &mut *config.lock().unwrap();
                config.speed += config.speed_inc;
                rumble();
                window.emit("speed-change", config.speed).unwrap();
            }))),
            Action::SpeedDec => ActionType::Simple(Arc::new(Box::new(|config, window, rumble| {
                let config = &mut *config.lock().unwrap();
                if config.speed > config.speed_inc {
                    config.speed -= config.speed_inc;
                    rumble();
                    window.emit("speed-change", config.speed).unwrap();
                }
            }))),
            Action::MClick => todo!(),
        }
    }
}
