pub mod helpers {

    use sysinfo::{ProcessExt, System, SystemExt};

    pub fn kill_process(target_process_name: String) {
        let mut system: System = System::new_all();
        system.refresh_all();

        for (_, process) in system.processes() {
            if process.name().to_lowercase().contains(&target_process_name) {
                process.kill();
            }
        }
    }
}
