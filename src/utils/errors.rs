pub mod errors {

    pub fn plugin_path_not_set() -> String {
        let config_location: &str = get_user_config_dir();

        println!("The selected plugin in not setup in ADA's config and cannot be run.\nUpdate config and try again");
        println!("Config can be located at: {}", config_location);
    }
}
