pub mod user_config {

    use sysinfo::User;

    use crate::config::structs::UserConfig;

    use std::{
        fs::File,
        io::{BufReader, Write},
        path::PathBuf,
    };

    fn get_user_config_dir() -> PathBuf {
        let mut raw_config_dir: PathBuf = dirs::config_dir().unwrap();
        raw_config_dir.push("ADA");
        return raw_config_dir;
    }

    pub fn get_user_config() -> UserConfig {
        let mut raw_user_config_dir: PathBuf = get_user_config_dir();
        raw_user_config_dir.push("user_config.json");

        let user_config_file: File =
            File::open(&raw_user_config_dir).expect("Failed to open user_config.json");
        let buffer_reader: BufReader<File> = BufReader::new(user_config_file);

        let user_config: UserConfig =
            serde_json::from_reader(buffer_reader).expect("Failed to parse user_config.json");

        return user_config;
    }

    pub fn create_user_config() {
        println!("Checking user config...");

        // Get locations
        let config_dir: PathBuf = get_user_config_dir();
        std::fs::create_dir_all(&config_dir).expect("Failed to create config dir");

        // Create the file path
        let mut default_user_config_file_path = config_dir.clone();
        default_user_config_file_path.push("user_config.json");

        if default_user_config_file_path.exists() {
            println!("Successful.\n");
            return;
        }

        // Create default user config struct
        let default_user_config: UserConfig = UserConfig::default();
        let serialised_data: String =
            serde_json::to_string(&default_user_config).expect("Failed to serialize user config");

        println!("default_user_config: {:?}\n", &default_user_config);

        // Create the file
        let mut default_user_config_file: File =
            File::create(default_user_config_file_path).expect("Failed to create user config file");

        default_user_config_file
            .write_all(serialised_data.as_bytes())
            .expect("Error writing UserConfig to JSON file.");

        println!("JSON file successfully created!\n");
    }
}
