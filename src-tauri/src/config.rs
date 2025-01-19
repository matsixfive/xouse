use crate::actions::ActionMap;
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
    pub fn config_dir(app_handle: &AppHandle) -> PathBuf {
        app_handle
            .path()
            .config_dir()
            .expect("Could not get config directory")
            .join(if cfg!(windows) { "Xouse" } else { "xouse" })
    }

    pub fn set_config_dir(&mut self, app_handle: &AppHandle) {
        self.config_dir = Some(Self::config_dir(app_handle));
    }

    fn config_file(&self) -> Option<PathBuf> {
        Some(self.config_dir.as_ref()?.join("config.toml"))
    }

    pub fn with_config_file(config_dir: &PathBuf) -> PathBuf {
        config_dir.join("config.toml")
    }

    pub fn save(&self) -> Result<()> {
        let config_dir = self
            .config_dir
            .as_ref()
            .ok_or(anyhow!("Config directory not set"))?;

        log::info!("Saving config at {:?}", config_dir);

        std::fs::create_dir_all(&config_dir)?;

        let config_file_path = self.config_file().ok_or(anyhow!("Config file not set"))?;
        let mut config_file = std::fs::File::create(&config_file_path)?;
        if let Ok(contents) = std::fs::read_to_string(&config_file_path) {
            if let Ok(diff) = diff(self, &contents) {
                config_file.write_all(diff.as_bytes())?;
                return Ok(());
            } else {
                log::error!("Could not diff config file");
            }
        } else {
            log::error!("Could not read config file");
        }

        let stringified = toml::to_string(&self)?;
        config_file.write_all(stringified.as_bytes())?;

        log::info!("Saved config");
        Ok(())
    }

    pub fn load(app_handle: &AppHandle) -> Result<Self> {
        let config_dir_path = Self::config_dir(app_handle);
        log::info!("Loading config from {:?}", config_dir_path);
        std::fs::create_dir_all(&config_dir_path)?;

        let config_file_path = Self::with_config_file(&config_dir_path);
        let config_text = std::fs::read_to_string(config_file_path)?;
        let mut config: Self = toml::from_str(&config_text)?;
        config.config_dir = Some(config_dir_path);

        log::info!("Loaded config");
        Ok(config)
    }
}

fn diff<T: serde::Serialize>(config: &T, toml_content: &str) -> anyhow::Result<String> {
    let mut doc = toml_content.parse::<toml_edit::Document>()?;

    // Serialize the config object into a TOML string

    let serialized = toml::to_string(config)?;
    let new_doc = serialized.parse::<toml_edit::Document>()?;

    // Compare and update the document
    for (key, new_value) in new_doc.iter() {
        if let Some(old_value) = doc.get(key) {
            // If the value is different, update it
            dbg!(&old_value, &new_value);

            if old_value.to_string() != new_value.to_string() {
                doc[key] = new_value.clone();
            }
        } else {
            // If the key doesn't exist, add it
            doc[key] = new_value.clone();
        }
    }

    // Return the new TOML content as a string

    Ok(doc.to_string())
}
