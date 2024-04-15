pub mod ada {
    use std::path::PathBuf;

    use crate::config::ada_config::ada_config::get_ada_config;

    pub fn open_ada_code() {
        let ada_code_path: PathBuf = get_ada_config().code_location;
        println!("ADA Code Location: {:?}", &ada_code_path);
    }
}
