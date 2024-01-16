use crate::actions::ActionMap;
use anyhow::Result;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Config {
    #[serde(default = "speed_default")]
    pub speed: f32, // base speed
    #[serde(skip, default = "speed_mult_default")]
    pub speed_mult: f32, // current speed multiplier
    #[serde(default = "speed_up_default")]
    pub speed_up: f32, // speed up multiplier
    #[serde(default = "speed_up_default")]
    pub speed_down: f32, // speed down multiplier
    #[serde(default = "speed_inc_default")]
    pub speed_inc: f32, // speed increment
    #[serde(default)]
    pub actions: ActionMap, // map of actions to button presses
}

const fn speed_default() -> f32 {
    50.0
}

const fn speed_mult_default() -> f32 {
    1.0
}

const fn speed_up_default() -> f32 {
    3.0
}

const fn speed_inc_default() -> f32 {
    5.0
}

impl Default for Config {
    fn default() -> Self {
        Self {
            speed: speed_default(),
            speed_mult: speed_mult_default(),
            speed_up: speed_up_default(),
            speed_down: speed_up_default(),
            speed_inc: speed_inc_default(),
            actions: ActionMap::default(),
        }
    }
}

impl Config {
    pub fn save(&self) -> Result<()> {
        println!("Saving config");
        let config_path = tauri::api::path::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not get config directory"))?;
        let config_path = config_path.join("Xouse");
        println!("Config path: {:?}", config_path);
        std::fs::create_dir_all(&config_path)?;
        let config_file = std::fs::File::create(config_path.join("config.json"))?;
        serde_json::to_writer_pretty(config_file, self)?;
        println!("Saved config");
        Ok(())
    }

    pub fn load() -> Result<Self> {
        println!("Loading config");
        let config_path = tauri::api::path::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not get config directory"))?;
        let config_path = config_path.join("Xouse");
        println!("Config path: {:?}", config_path);
        let config_file = std::fs::File::open(config_path.join("config.json"))?;
        let config: Self = serde_json::from_reader(config_file)?;
        println!("Loaded config");
        Ok(config)
    }
}
