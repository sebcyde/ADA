pub mod sentry {

    use sysinfo::{Pid, PidExt, Process, ProcessExt, System, SystemExt};

    pub fn run_sentry() {
        // let sentry_executable_location = dirs::d
    }

    pub fn stop_sentry() {
        let mut system: System = System::new_all();
        system.refresh_all();

        let mut sentry_instances: Vec<(&Pid, &Process)> = Vec::new();
        let current_sentry: u32 = std::process::id();

        for (pid, process) in system.processes() {
            if process.name().eq_ignore_ascii_case("sentry_lite") {
                sentry_instances.push((pid, process));
            }
        }

        for (pid, process) in sentry_instances {
            if !pid.as_u32().eq(&current_sentry) {
                process.kill();
            }
        }

        println!("All Sentry instances stopped. Exiting...\n");
        std::process::exit(0);
    }
}
