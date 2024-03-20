use config::{Config, ConfigError, Environment, File};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Mutex;
use std::fs;
use serde_json::{Value, json};

lazy_static! {
    pub static ref SETTINGS: Mutex<Settings> = Mutex::new(Settings::new().expect("Failed to setup settings"));
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Server {
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Logger {
    pub level: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Settings {
    pub environment: String,
    pub server: Server,
    pub logger: Logger,
    pub projects: Vec<Project>,
    pub colors: BColors,
    #[serde(default)]
    pub settings: Option<AppCFG>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppCFG {
    #[serde(default = "default_intivial")]
    pub intivial: u64,
}

fn default_intivial() -> u64 {
    15
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Project {
    pub name: String,
    pub path: String,
    pub updated_at: String,
    pub github_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)] 
pub struct BColors {
    pub header: String,
    pub blue: String,
    pub cyan: String,
    pub cyan_green: String,
    pub warning: String,
    pub fail: String,
    pub endc: String,
    pub bold: String,
    pub underline: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let mut s = Config::default();
        s.merge(File::with_name("config/default"))?;
        s.merge(File::with_name("config"))?;
        s.merge(File::with_name(&format!("config/{}", run_mode)).required(false))?;
        s.merge(File::with_name("config/local").required(false))?;
        s.merge(Environment::with_prefix("app").separator("__"))?;

        let settings: Settings = s.try_into()?;

        if settings.projects.is_empty() {
            return Err(ConfigError::Message("At least one project is required".to_string()));
        }

        Ok(settings)
    }
}

pub fn update_project_updated_at(project_name: String, new_updated_at: String) {
    let file_path = "config.json";
    let config_content = fs::read_to_string(file_path)
        .expect("Failed to read config file");

    let mut config_json: Value = serde_json::from_str(&config_content)
        .expect("Failed to parse config content as JSON");

    if let Some(projects) = config_json["projects"].as_array_mut() {
        for project in projects.iter_mut() {
            if project["name"] == project_name {
                project["updated_at"] = json!(new_updated_at);
            }
        }
    }

    let serialized_settings = serde_json::to_string_pretty(&config_json)
        .expect("Failed to serialize updated settings");
    fs::write(file_path, serialized_settings.as_bytes())
        .expect("Failed to write updated settings to config file");
}