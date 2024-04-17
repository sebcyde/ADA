pub mod hyperlink {

    use std::{path::PathBuf, process::Output};

    use crate::{
        config::user_config::user_config::get_user_config, plugins::control::control::Function,
        utils::helpers::helpers::kill_process,
    };

    pub struct Hyperlink;

    impl Function for Hyperlink {
        fn name(&self) -> &str {
            "Hyperlink"
        }

        fn description(&self) -> &str {
            "Hyperlink syncs all of my files that contain keyboard shortcuts for various programs such as VsCode, Katalon and some other programs."
        }

        fn execute(&self, input: &str) -> String {
            format!("Hyperlink activated with input: {}", input);
            let output: Output = std::process::Command::new(
                "C:/Users/sebastian.cyde/Documents/Code/01-Programs/Hyperlink/target/debug/hyperlink.exe",
            )
            .output()
            .expect("Failed to execute Hyperlink");

            println!("{}", String::from_utf8_lossy(&output.stdout));

            if !output.status.success() {
                return "Hyperlink encountered an error and has stopped running".to_string();
            }

            return "Hyperlink activation successful. Files are now being synced.".to_string();
        }
    }

    pub fn start_hyperlink() {
        kill_process(String::from("hyperlink"));

        // Start a new hyperlink instance - remember to pass in clean paramater
        let hyperlink_path: PathBuf = get_user_config().plugins.hyperlink;
        println!("Hyperlink path from config: {:?}", &hyperlink_path);
    }

    // Kill any currently running hyperlink processes
    pub fn stop_hyperlink() {
        kill_process(String::from("hyperlink"));
    }
}
