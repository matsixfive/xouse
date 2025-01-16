use crate::actions2::ActionMap;
use anyhow::{anyhow, Result};
use std::{io::Write, path::PathBuf};
use tauri::{AppHandle, Manager};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(default = "speed_default")]
    pub speed: f32, // base speed
    #[serde(skip, default = "speed_mult_default")]
    pub speed_mult: f32, // current speed multiplier
    #[serde(default = "speed_up_default")]
    pub speed_up: f32, // speed up multiplier
    #[serde(default = "speed_up_default")]
    pub speed_down: f32, // speed down multiplier
    #[serde(default = "speed_step_default")]
    pub speed_step: f32, // speed increment

    #[serde(skip, default)]
    pub gamepad_id: Option<gilrs::GamepadId>, // gamepad id

    #[serde(default)]
    pub actions: ActionMap, // map of actions to button presses

    #[serde(skip)]
    config_dir: Option<PathBuf>,
}

const fn speed_default() -> f32 {
    70.0
}

const fn speed_mult_default() -> f32 {
    1.0
}

const fn speed_up_default() -> f32 {
    3.0
}

const fn speed_step_default() -> f32 {
    5.0
}

impl Default for Config {
    fn default() -> Self {
        Self {
            speed: speed_default(),
            speed_mult: speed_mult_default(),
            speed_up: speed_up_default(),
            speed_down: speed_up_default(),
            speed_step: speed_step_default(),
            gamepad_id: None,
            actions: ActionMap::default(),
            config_dir: None,
        }
    }
}

impl Config {
    fn config_dir(app_handle: &AppHandle) -> PathBuf {
        app_handle
            .path()
            .config_dir()
            .expect("Could not get config directory")
            .join(if cfg!(windows) { "Xouse" } else { "xouse" })
    }

    fn config_file(&self) -> Option<PathBuf> {
        Some(self.config_dir.as_ref()?.join("config.toml"))
    }

    fn with_config_file(config_dir: &PathBuf) -> PathBuf {
        config_dir.join("config.toml")
    }

    pub fn save(&self) -> Result<()> {
        println!("Saving config");

        let config_dir = &self.config_dir.as_ref().ok_or(anyhow!("Config directory not set"))?;
        println!("Config path: {:?}", &self.config_dir);
        std::fs::create_dir_all(&config_dir)?;
        let mut config_file = std::fs::File::create(Self::with_config_file(config_dir))?;

        let stringified = toml::to_string(&self)?;
        config_file.write_all(stringified.as_bytes())?;

        println!("Saved config");
        Ok(())
    }

    pub fn load(app_handle: &AppHandle) -> Result<Self> {
        println!("Loading config");

        let config_dir_path = Self::config_dir(app_handle);
        println!("Config path: {:?}", config_dir_path);
        std::fs::create_dir_all(&config_dir_path)?;

        let config_file_path = Self::with_config_file(&config_dir_path);
        let config_text = std::fs::read_to_string(config_file_path)?;
        let config: Self = toml::from_str(&config_text)?;

        println!("Loaded config");
        Ok(config)
    }
}
