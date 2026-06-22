use engine::ConnectionProfile;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const CONFIG_PATH: &str = "aktsql.config.json";
const CONFIG_DIR_ENV: &str = "AKTSQL_CONFIG_DIR";

#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    version: u32,
    connections: Vec<ConnectionProfile>,
    #[serde(default)]
    preferences: AppPreferences,
}

impl AppConfig {
    fn new(connections: &[ConnectionProfile], preferences: AppPreferences) -> Self {
        Self {
            version: 1,
            connections: connections.to_vec(),
            preferences,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppPreferences {
    pub language: String,
    pub theme: String,
    pub ui_font: String,
}

impl Default for AppPreferences {
    fn default() -> Self {
        Self {
            language: String::from("zh_cn"),
            theme: String::from("dark"),
            ui_font: String::from("platform_default"),
        }
    }
}

pub fn load_connection_profiles() -> Result<Vec<ConnectionProfile>, String> {
    let Some(path) = readable_config_path() else {
        return Ok(Vec::new());
    };

    let raw = fs::read_to_string(&path).map_err(|error| read_error(&path, error))?;
    let config: AppConfig = serde_json::from_str(&raw)
        .map_err(|error| format!("Failed to parse {}: {error}", path.display()))?;

    Ok(config.connections)
}

pub fn save_connection_profiles(profiles: &[ConnectionProfile]) -> Result<(), String> {
    let path = preferred_config_path();
    let preferences = load_preferences().unwrap_or_default();
    let config = AppConfig::new(profiles, preferences);
    write_config(&path, &config)
}

pub fn load_preferences() -> Result<AppPreferences, String> {
    let Some(path) = readable_config_path() else {
        return Ok(AppPreferences::default());
    };

    let raw = fs::read_to_string(&path).map_err(|error| read_error(&path, error))?;
    let config: AppConfig = serde_json::from_str(&raw)
        .map_err(|error| format!("Failed to parse {}: {error}", path.display()))?;

    Ok(config.preferences)
}

pub fn save_preferences(preferences: &AppPreferences) -> Result<(), String> {
    let path = preferred_config_path();
    let connections = load_connection_profiles().unwrap_or_default();
    let config = AppConfig::new(&connections, preferences.clone());
    write_config(&path, &config)
}

fn write_config(path: &Path, config: &AppConfig) -> Result<(), String> {
    let raw = serde_json::to_string_pretty(&config)
        .map_err(|error| format!("Failed to serialize {}: {error}", path.display()))?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| write_error(parent, error))?;
    }

    fs::write(&path, raw).map_err(|error| write_error(&path, error))
}

fn readable_config_path() -> Option<PathBuf> {
    let preferred = preferred_config_path();
    if preferred.exists() {
        return Some(preferred);
    }

    let fallback = fallback_config_path();
    fallback.exists().then_some(fallback)
}

fn preferred_config_path() -> PathBuf {
    if let Some(path) = env::var_os(CONFIG_DIR_ENV).filter(|value| !value.is_empty()) {
        return PathBuf::from(path).join(CONFIG_PATH);
    }

    platform_config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("aktsql")
        .join(CONFIG_PATH)
}

fn fallback_config_path() -> PathBuf {
    PathBuf::from(CONFIG_PATH)
}

fn platform_config_dir() -> Option<PathBuf> {
    if cfg!(target_os = "windows") {
        env::var_os("APPDATA").map(PathBuf::from)
    } else if cfg!(target_os = "macos") {
        env::var_os("HOME")
            .map(PathBuf::from)
            .map(|home| home.join("Library").join("Application Support"))
    } else {
        env::var_os("XDG_CONFIG_HOME")
            .filter(|value| !value.is_empty())
            .map(PathBuf::from)
            .or_else(|| {
                env::var_os("HOME")
                    .map(PathBuf::from)
                    .map(|home| home.join(".config"))
            })
    }
}

fn read_error(path: &Path, error: io::Error) -> String {
    format!("Failed to read {}: {error}", path.display())
}

fn write_error(path: &Path, error: io::Error) -> String {
    format!("Failed to write {}: {error}", path.display())
}
