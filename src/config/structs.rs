use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct UserConfig {
    pub plugins: PluginConfig,
    pub db_paths: DBConfig,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct PluginConfig {
    pub sentry: Option<PathBuf>,
    pub hyperlink: Option<PathBuf>,
    pub penny: Option<PathBuf>,
}

// ADA Config
#[derive(Deserialize, Serialize, Default, Debug)]
pub struct AdaConfig {
    pub code_location: Option<PathBuf>,
}

// DB Pathing Config
#[derive(Deserialize, Serialize, Default, Debug)]
pub struct DBConfig {
    pub es_uk_db_path: Option<PathBuf>,
    pub es_us_db_path: Option<PathBuf>,
    pub fc_db_path: Option<PathBuf>,
    pub re_db_path: Option<PathBuf>,
    pub backups_dir: Option<PathBuf>,
}
