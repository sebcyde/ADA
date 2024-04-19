pub mod user_config {

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

    fn create_user_config() -> Option<PathBuf> {
        let mut config_dir: PathBuf = get_user_config_dir();
        config_dir.push("ADA");

        std::fs::create_dir_all(&config_dir).expect("Failed to create config dir");

        let serialised_data: String = serde_json::to_string(struct_instance)?;

        let mut default_user_config_file: File =
            File::create(config_dir).expect("Failed to create user config file");
        let res: Result<(), std::io::Error> =
            default_user_config_file.write_all(serialised_data.as_bytes());

        if res.is_err() {
            eprintln!("Error writing JSON file: {}", err);
        } else {
            println!("JSON file successfully created!");
        }
    }

    // pub fn create_user_config() -> UserConfig {}
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
}
