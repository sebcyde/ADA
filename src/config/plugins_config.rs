pub mod plugins_config {
    use std::path::PathBuf;

    pub fn get_plugins_config_dir() -> PathBuf {
        let mut plugins_config_dir: PathBuf = dirs::config_dir().unwrap();
        plugins_config_dir.push("ADA");
        plugins_config_dir.push("plugins");
        return plugins_config_dir;
    }

    // pub fn get_plugins_config() -> PluginsConfig {
    //     let plugins_config_dir: PathBuf = get_plugins_config_dir();

    //     return ada_config;
    // }
}
