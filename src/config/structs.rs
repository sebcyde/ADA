use std::path::PathBuf;

use serde::Deserialize;

//
#[derive(Deserialize)]
pub struct PluginConfig {
    pub sentry: PathBuf,
    pub hyperlink: PathBuf,
    pub penny: PathBuf,
}

#[derive(Deserialize)]
pub struct UserConfig {
    pub plugins: PluginConfig,
}

// ADA Config
#[derive(Deserialize)]
pub struct AdaConfig {
    pub code_location: PathBuf,
}
