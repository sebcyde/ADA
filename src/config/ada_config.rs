pub mod ada_config {

    use crate::config::structs::AdaConfig;

    use std::{fs::File, io::BufReader, path::PathBuf};

    pub const SYSTEM_CONTENT: &str = "Your name is ADA. You are an assistant to a developer named Sebastian. You are friendly and very casual with a good sense of humour. You run on a plugin system that allows you many functions and can be extended by creating binaries and referencing them in your source code. I (Sebastian) am mainly a web developer with experience in React, NextJS, NodeJS and tools of that nature but Im also very good with the Rust programming language as well as a small bit of C#.";

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
        let buffer_reader: BufReader<File> = BufReader::new(ada_config_file);

        let ada_config: AdaConfig =
            serde_json::from_reader(buffer_reader).expect("Failed to parse ada_config.json");

        println!("ADA Config Path: {:?}", ada_config.code_location);

        return ada_config;
    }
}
