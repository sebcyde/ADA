pub mod sentry {

    use crate::plugins::control::control::Function;
    use std::process::Output;

    pub struct Sentry;

    impl Function for Sentry {
        fn name(&self) -> &str {
            "Start Sentry"
        }

        fn description(&self) -> &str {
            "Sentry is a file system cleaner. It scans through all of my directories and organises files into their correct places based on their file types. It also renames files to follow specific formats, for example removing spaces and replacing capital letters with their lower case equivalents. This function starts the Sentry program."
        }

        fn execute(&self, input: &str) -> String {
            format!("Sentry activated with input: {}", input);
            let output: Output = std::process::Command::new(
                "C:/Users/sebastian.cyde/Documents/Code/01-Programs/sentry_lite/target/debug/sentry_lite.exe",
            )
            .arg("clean")
            .output()
            .expect("Failed to start Sentry");

            println!("{}", String::from_utf8_lossy(&output.stdout));

            if !output.status.success() {
                return "Sentry encountered an error and has stopped running".to_string();
            }

            return "Sentry activation successful. File system is now being cleaned.".to_string();
        }
    }
}
