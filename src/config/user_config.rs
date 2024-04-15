pub mod user_config {

    use crate::config::structs::UserConfig;

    use std::{fs::File, io::BufReader, path::PathBuf};

    fn get_user_config_dir() -> PathBuf {
        let mut raw_config_dir: PathBuf = dirs::config_dir().unwrap();
        raw_config_dir.push("ADA");
        return raw_config_dir;
    }

    // pub fn create_user_config() -> UserConfig {}
    pub fn get_user_config() -> UserConfig {
        let mut raw_user_config_dir: PathBuf = get_user_config_dir();
        raw_user_config_dir.push("user_config.json");

        let user_config_file: File =
            File::open(&raw_user_config_dir).expect("Failed to open user_config.json");
        let buffer_reader = BufReader::new(user_config_file);

        let user_config: UserConfig =
            serde_json::from_reader(buffer_reader).expect("Failed to parse user_config.json");

        println!("Sentry Path: {:?}", user_config.plugins.sentry);

        return user_config;
    }
}
