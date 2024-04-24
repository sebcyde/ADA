pub mod helpers {

    // use sysinfo::{ProcessExt, System, SystemExt};

    use std::{ffi::OsStr, path::PathBuf};

    pub fn kill_process(target_process_name: String) {
        println!("Killing {}", target_process_name);
        // let mut system: System = System::new_all();
        // system.refresh_all();

        // for (_, process) in system.processes() {
        //     if process.name().to_lowercase().contains(&target_process_name) {
        //         process.kill();
        //     }
        // }
    }

    pub fn format_file_name(mut file_path: PathBuf) -> String {
        let original_file_name: &OsStr = file_path.file_name().unwrap();
        let string_file_name: String = original_file_name.to_str().unwrap().to_ascii_lowercase();
        let spaceless_name: String = string_file_name.replace(" ", "_");
        return spaceless_name;
    }
}
