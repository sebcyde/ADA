pub mod penny {
    use std::path::PathBuf;

    use crate::{
        config::user_config::user_config::get_user_config, utils::helpers::helpers::kill_process,
    };

    pub fn run_penny() {
        // Kill any currently running hyperlink processes
        kill_process(String::from("penny"));

        // Start a new hyperlink instance - remember to pass in clean paramater
        let penny_path: PathBuf = get_user_config().plugins.penny.unwrap();
        println!("Penny path from config: {:?}", &penny_path);
    }
}
