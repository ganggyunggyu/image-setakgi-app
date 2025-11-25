use std::{fs, path::PathBuf};

use dirs_next::config_dir;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    pub resize_min: f32,
    pub resize_max: f32,
    pub rotate_max_deg: f32,
    pub brightness_range: f32,
    pub contrast_range: f32,
    pub noise_sigma: f32,
    pub jpeg_quality: u8,
    pub webp_quality: u8,
    pub strip_exif: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            resize_min: 0.9,
            resize_max: 1.1,
            rotate_max_deg: 2.0,
            brightness_range: 5.0,
            contrast_range: 5.0,
            noise_sigma: 1.0,
            jpeg_quality: 90,
            webp_quality: 90,
            strip_exif: true,
        }
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("config dir unavailable")]
    MissingDir,
    #[error("io error {0}")]
    Io(#[from] std::io::Error),
    #[error("serialize error {0}")]
    Serialize(#[from] serde_json::Error),
}

fn preset_path(name: &str) -> Result<PathBuf, ConfigError> {
    let base = config_dir().ok_or(ConfigError::MissingDir)?;
    let dir = base.join("image_setakgi");
    fs::create_dir_all(&dir)?;
    Ok(dir.join(format!("{name}.json")))
}

pub fn save_preset(name: &str, options: &Options) -> Result<(), ConfigError> {
    let path = preset_path(name)?;
    let json = serde_json::to_string_pretty(options)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn load_preset(name: &str) -> Result<Options, ConfigError> {
    let path = preset_path(name)?;
    let raw = fs::read_to_string(path)?;
    let opts = serde_json::from_str(&raw)?;
    Ok(opts)
}
