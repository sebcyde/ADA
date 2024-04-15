pub mod hyperlink {

    use std::path::PathBuf;

    use crate::{
        config::user_config::user_config::get_user_config, utils::helpers::helpers::kill_process,
    };

    pub fn run_hyperlink() {
        // Kill any currently running hyperlink processes
        kill_process(String::from("hyperlink"));

        // Start a new hyperlink instance - remember to pass in clean paramater
        let hyperlink_path: PathBuf = get_user_config().plugins.hyperlink;
        println!("Hyperlink path from config: {:?}", &hyperlink_path);
    }
}
