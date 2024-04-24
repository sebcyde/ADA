pub mod control {

    use libloading::{Library, Symbol};
    use std::ffi::OsStr;
    use std::fs::{self, DirEntry};
    use std::path::PathBuf;

    use crate::config::ada_config;
    use crate::config::plugins_config::plugins_config::get_plugins_config_dir;
    use crate::config::structs::UserConfig;
    use crate::config::user_config::user_config::get_user_config;
    use crate::plugins::ada::ada::ADA;
    use crate::plugins::hyperlink::hyperlink::Hyperlink;
    use crate::plugins::sentry::sentry::Sentry;

    pub trait Function {
        fn name(&self) -> &str;
        fn description(&self) -> &str;
        fn execute(&self, input: &str) -> String;
    }

    pub trait Plugin: Function {
        fn name(&self) -> &str;
        fn description(&self) -> &str;
    }

    pub enum PLUGINS {
        HYPERLINK,
        PENNY,
        BACKUP,
        SENTRY,
        ADA,
    }

    // External plugins from shared libs (.dll files)
    fn _load_plugin<P: AsRef<OsStr>>(path: P) -> Result<Box<dyn Plugin>, String> {
        unsafe {
            let lib: Library = Library::new(path.as_ref()).map_err(|e| e.to_string())?;
            let symbol: Symbol<*mut dyn Plugin> =
                lib.get(b"create_plugin").map_err(|e| e.to_string())?;
            let plugin: Box<dyn Plugin> = Box::from_raw(*symbol.into_raw());
            Ok(plugin)
        }
    }

    // Local functions / binaries locally referenced (from plugins dir)
    pub fn get_functions() -> Vec<Box<dyn Function>> {
        let mut internal_functions: Vec<Box<dyn Function>> = Vec::new();

        // Hyperlink - Settings sync program
        let hyperlink_function: Box<Hyperlink> = Box::new(Hyperlink {});
        internal_functions.push(hyperlink_function);

        // Sentry - File system cleaner
        let sentry_function: Box<Sentry> = Box::new(Sentry {});
        internal_functions.push(sentry_function);

        // ADA - Open ADA's source code
        let ada_function: Box<ADA> = Box::new(ADA {});
        internal_functions.push(ada_function);

        return internal_functions;
    }

    pub fn _get_plugins() -> Result<Vec<Box<dyn Plugin>>, String> {
        let plugins_dir: PathBuf = get_plugins_config_dir();

        let mut plugins: Vec<Box<dyn Plugin>> = Vec::new();
        for entry in fs::read_dir(&plugins_dir).map_err(|e| e.to_string())? {
            let entry: DirEntry = entry.map_err(|e| e.to_string())?;
            if let Some(extension) = entry.path().extension() {
                if extension == "so" || extension == "dll" || extension == "dylib" {
                    if let Ok(plugin) = _load_plugin(entry.path()) {
                        plugins.push(plugin);
                    }
                }
            }
        }
        Ok(plugins)
    }

    // pub fn execute_plugin(plugin_name: PLUGINS) {
    //     let user_config: UserConfig = get_user_config();

    //     println!("Activating {:?} with input", &plugin_name);

    //     let plugin_path: Option<PathBuf> = match plugin_name {
    //         PLUGINS::ADA => {}
    //         PLUGINS::BACKUP => {}
    //         PLUGINS::HYPERLINK => {}
    //         PLUGINS::PENNY => {}
    //         PLUGINS::SENTRY => {}
    //     };

    //     if plugin_path.is_none() {
    //         println!("The selected plugin in not setup in ADA's config and cannot be run.\nUpdate config and try again");
    //     }

    //     let output: std::process::Output = std::process::Command::new(
    //         "C:/Users/sebastian.cyde/Documents/Code/01-Programs/Hyperlink/target/debug/hyperlink.exe",
    //     )
    //     .output()
    //     .expect("Failed to execute Hyperlink");

    //     println!("{}", String::from_utf8_lossy(&output.stdout));

    //     if !output.status.success() {
    //         return "Hyperlink encountered an error and has stopped running".to_string();
    //     }

    //     return "Hyperlink activation successful. Files are now being synced.".to_string();
    // }
}
