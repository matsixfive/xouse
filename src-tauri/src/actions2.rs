#![allow(unused)]

use crate::config::Config;
use gilrs::Button;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tauri::{Emitter, WebviewWindow};

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
                    script: String::from("print('Hello from Lua!')"),
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
    fn rumble(&self);
}

pub struct ActionInterface {
    pub config: Arc<Mutex<Config>>,
    pub window: WebviewWindow,
    pub rumble: Option<Box<dyn Rumbler>>,
}

pub trait SimpleActionFn {
    fn call(&self, state: &ActionInterface);
}

impl SimpleActionFn for SimpleAction {
    fn call(&self, state: &ActionInterface) {
        match self {
            SimpleAction::SpeedInc => todo!(),
            SimpleAction::SpeedDec => todo!(),
            SimpleAction::Rumble => todo!(),
            SimpleAction::ToggleVis => todo!(),
        }
    }
}

pub trait UpDownActionFn {
    fn down(&self, state: &ActionInterface);
    fn up(&self, state: &ActionInterface);
}

impl UpDownActionFn for UpDownAction {
    fn down(&self, state: &ActionInterface) {
        match self {
            UpDownAction::Click(button) => todo!(),
            UpDownAction::SpeedUp => todo!(),
            UpDownAction::SpeedDown => todo!(),
            UpDownAction::KeyPress { key, modifiers } => todo!(),
            UpDownAction::LuaScript { script } => {
                let lua = mlua::Lua::new();

                let f = lua
                    .create_function(|_, ()| -> mlua::Result<i32> {
                        println!("running 69");
                        Ok(69)
                    })
                    .expect("Failed to create function");

                lua.globals()
                    .set("rust_func", f)
                    .expect("Failed to set global");

                let _ = lua.load(script.as_str()).exec();
            }
        }
    }

    fn up(&self, state: &ActionInterface) {
        match self {
            UpDownAction::Click(button) => todo!(),
            UpDownAction::SpeedUp => todo!(),
            UpDownAction::SpeedDown => todo!(),
            UpDownAction::KeyPress { key, modifiers } => todo!(),
            UpDownAction::LuaScript { script } => todo!(),
        }
    }
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub enum ModifierKey {
    Alt,
    Ctrl,
    Win,
    Shift,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}