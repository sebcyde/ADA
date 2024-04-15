pub mod ada_config {

    use crate::config::structs::AdaConfig;

    use std::{fs::File, io::BufReader, path::PathBuf};

    fn get_ada_config_dir() -> PathBuf {
        let mut raw_config_dir: PathBuf = dirs::config_dir().unwrap();
        raw_config_dir.push("ADA");
        return raw_config_dir;
    }

    // pub fn create_user_config() -> UserConfig {}
    pub fn get_ada_config() -> AdaConfig {
        let mut raw_ada_config_dir: PathBuf = get_ada_config_dir();
        raw_ada_config_dir.push("ada_config.json");

        let ada_config_file: File =
            File::open(&raw_ada_config_dir).expect("Failed to open ada_config.json");
        let buffer_reader = BufReader::new(ada_config_file);

        let ada_config: AdaConfig =
            serde_json::from_reader(buffer_reader).expect("Failed to parse ada_config.json");

        println!("ADA Config Path: {:?}", ada_config.code_location);

        return ada_config;
    }
}
