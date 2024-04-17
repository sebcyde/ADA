pub mod control {

    use libloading::{Library, Symbol};
    use std::ffi::OsStr;
    use std::fs;
    use std::path::PathBuf;

    use crate::config::plugins_config::plugins_config::get_plugins_config_dir;

    pub trait Plugin {
        fn name(&self) -> &str;
        fn description(&self) -> &str;
        fn execute(&self, input: &str) -> String;
    }

    // Load a plugin from a shared library
    // fn load_plugin<P: AsRef<OsStr>>(path: P) -> Result<Box<dyn Plugin>, String> {
    //     unsafe {
    //         let lib = Library::new(path.as_ref()).map_err(|e| e.to_string())?;
    //         let symbol: Symbol<*mut dyn Plugin> =
    //             lib.get(b"create_plugin").map_err(|e| e.to_string())?;
    //         let plugin = Box::from_raw(symbol.into_raw());
    //         Ok(plugin)
    //     }
    // }

    // pub fn get_plugins() -> Result<Vec<Box<dyn Plugin>>, String> {
    //     let plugins_dir: PathBuf = get_plugins_config_dir();

    //     let mut plugins = Vec::new();
    //     for entry in fs::read_dir(&plugins_dir).map_err(|e| e.to_string())? {
    //         let entry = entry.map_err(|e| e.to_string())?;
    //         if let Some(extension) = entry.path().extension() {
    //             if extension == "so" || extension == "dll" || extension == "dylib" {
    //                 if let Ok(plugin) = load_plugin(entry.path()) {
    //                     plugins.push(plugin);
    //                 }
    //             }
    //         }
    //     }
    //     Ok(plugins)
    // }
}
