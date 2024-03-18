use config::{Config, ConfigError, Environment, File};
use lazy_static::lazy_static;
use serde::Deserialize;
use std::env;

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().expect("Failed to setup settings");
}

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Logger {
    pub level: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub environment: String,
    pub server: Server,
    pub logger: Logger,
    pub projects: Vec<Project>,
    pub colors: BColors,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Project {
    pub name: String,
    pub path: String,
    pub updated_at: String,
    pub github_url: String,
}

#[derive(Debug, Clone, Deserialize)]
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

        s.try_into()
    }
}
