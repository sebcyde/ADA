pub mod ada {

    use crate::plugins::control::control::Function;
    use std::process::Output;

    pub struct ADA;

    impl Function for ADA {
        fn name(&self) -> &str {
            "Start ADA"
        }

        fn description(&self) -> &str {
            "Open your (ADA's) source code for the user in VsCode."
        }

        fn execute(&self, _input: &str) -> String {
            format!("Opening ADA source code");

            let output: Output = std::process::Command::new("code")
                .arg("C:/Users/sebastian.cyde/Documents/Code/01-Programs/ADA")
                .output()
                .expect("Failed to open ADA source code");

            if !output.status.success() {
                return "Failed to open ADA source code".to_string();
            }

            return "ADA source code opened succesfully".to_string();
        }
    }
}
