pub mod sentry {

    use std::path::PathBuf;

    use crate::{
        config::user_config::user_config::get_user_config, utils::helpers::helpers::kill_process,
    };

    pub fn run_sentry() {
        // Kill any currently running Sentry processes
        kill_process(String::from("sentry"));

        // Start a new Sentry instance - remember to pass in clean paramater
        let sentry_path: PathBuf = get_user_config().plugins.sentry;
        println!("Sentry path from config: {:?}", &sentry_path);
    }
}
