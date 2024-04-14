use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct PluginConfig {
    pub sentry: PathBuf,
    pub plugin2: PathBuf,
}

#[derive(Deserialize)]
pub struct UserConfig {
    pub plugins: PluginConfig,
}
