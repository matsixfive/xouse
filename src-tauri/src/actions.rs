use crate::config::Config;
use gilrs::Button;
use rdev::Key;
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

#[derive(Debug, Clone)]
pub struct ActionMap(pub HashMap<Button, Vec<Action>>);

impl Default for ActionMap {
    fn default() -> Self {
        let map = HashMap::from([
            (Button::South, vec![Action::Click(MouseButton::Left)]),
            (Button::East, vec![Action::Click(MouseButton::Right)]),
            (Button::North, vec![Action::KeyPress(Key::Space, vec![])]),
            (Button::DPadUp, vec![Action::SpeedInc, Action::Rumble]),
            (Button::DPadDown, vec![Action::SpeedDec, Action::Rumble]),
            (
                Button::RightTrigger,
                vec![
                    Action::KeyPress(Key::Tab, vec![ModifierKey::Ctrl]),
                    Action::Rumble,
                ],
            ),
            (
                Button::LeftTrigger,
                vec![
                    Action::KeyPress(Key::Tab, vec![ModifierKey::Ctrl, ModifierKey::Shift]),
                    Action::Rumble,
                ],
            ),
            (Button::RightTrigger2, vec![Action::SpeedUp]),
            (Button::LeftTrigger2, vec![Action::SpeedDown]),
            (Button::Select, vec![Action::ToggleVis]),
        ]);
        Self(map)
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum Action {
    Click(MouseButton),
    SpeedUp,
    SpeedDown,
    SpeedInc,
    SpeedDec,
    KeyPress(Key, Vec<ModifierKey>),
    Rumble,
    ToggleVis,
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub enum ModifierKey {
    Alt,
    Ctrl,
    Win,
    Shift,
}

impl Into<Key> for ModifierKey {
    fn into(self) -> Key {
        match self {
            ModifierKey::Alt => Key::Alt,
            ModifierKey::Ctrl => Key::ControlLeft,
            ModifierKey::Win => Key::MetaLeft,
            ModifierKey::Shift => Key::ShiftLeft,
        }
    }
}

impl Into<Key> for &ModifierKey {
    fn into(self) -> Key {
        match self {
            ModifierKey::Alt => Key::Alt,
            ModifierKey::Ctrl => Key::ControlLeft,
            ModifierKey::Win => Key::MetaLeft,
            ModifierKey::Shift => Key::ShiftLeft,
        }
    }
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
        let map = HashMap::<String, Vec<Action>>::deserialize(deserializer)?;
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

impl From<Action> for ActionType {
    fn from(val: Action) -> Self {
        match val {
            Action::Click(button) => match button {
                MouseButton::Left => ActionType::UpDown((
                    Arc::new(Box::new(|_, _, _| unsafe {
                        println!("click");
                        kbm::mouse_event(kbm::MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
                    })),
                    Arc::new(Box::new(|_, _, _| unsafe {
                        println!("click up");
                        kbm::mouse_event(kbm::MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
                    })),
                )),
                MouseButton::Right => ActionType::UpDown((
                    Arc::new(Box::new(|_, _, _| unsafe {
                        println!("rclick");
                        kbm::mouse_event(kbm::MOUSEEVENTF_RIGHTDOWN, 0, 0, 0, 0);
                    })),
                    Arc::new(Box::new(|_, _, _| unsafe {
                        println!("rclick up");
                        kbm::mouse_event(kbm::MOUSEEVENTF_RIGHTUP, 0, 0, 0, 0);
                    })),
                )),
                MouseButton::Middle => ActionType::UpDown((
                    Arc::new(Box::new(|_, _, _| unsafe {
                        println!("mclick");
                        kbm::mouse_event(kbm::MOUSEEVENTF_MIDDLEDOWN, 0, 0, 0, 0);
                    })),
                    Arc::new(Box::new(|_, _, _| unsafe {
                        println!("mclick up");
                        kbm::mouse_event(kbm::MOUSEEVENTF_MIDDLEUP, 0, 0, 0, 0);
                    })),
                )),
            },
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
            Action::SpeedInc => ActionType::Simple(Arc::new(Box::new(|config, window, _| {
                let config = &mut *config.lock().unwrap();
                config.speed += config.speed_step;
                window.emit("speed_change", config.speed).unwrap();
            }))),
            Action::SpeedDec => ActionType::Simple(Arc::new(Box::new(|config, window, _| {
                let config = &mut *config.lock().unwrap();
                if config.speed > config.speed_step {
                    config.speed -= config.speed_step;
                    window.emit("speed_change", config.speed).unwrap();
                }
            }))),
            Action::KeyPress(key, modifiers) => {
                let modifiers2 = modifiers.clone();
                ActionType::UpDown((
                    Arc::new(Box::new(move |_, _, _| {
                        println!("pressing {:?} with modifiers {:?}", key, modifiers);

                        for modifier in &modifiers {
                            let _ = rdev::simulate(&rdev::EventType::KeyPress(modifier.into()));
                        }

                        let _ = rdev::simulate(&rdev::EventType::KeyPress(key));
                    })),
                    Arc::new(Box::new(move |_, _, _| {
                        println!("releasing {:?} with modifiers {:?}", key, modifiers2);

                        let _ = rdev::simulate(&rdev::EventType::KeyRelease(key));

                        for modifier in modifiers2.iter().rev() {
                            let _ = rdev::simulate(&rdev::EventType::KeyRelease(modifier.into()));
                        }
                    })),
                ))
            }
            Action::Rumble => ActionType::Simple(Arc::new(Box::new(|_, _, rumble| rumble()))),
            Action::ToggleVis => {
                ActionType::Simple(Arc::new(Box::new(|_, window, _| toggle_window(window))))
            }
        }
    }
}

pub fn toggle_window(window: &Window) {
    if window.is_visible().expect("winvis") {
        {
            window.hide().expect("winhide");
            return;
        }
    };
    window.show().unwrap();
    window.set_focus().unwrap();
}
