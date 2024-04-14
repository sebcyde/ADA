pub mod navigation {
    use std::path::PathBuf;

    pub fn get_documents_dir() -> PathBuf {
        return dirs::document_dir().unwrap();
    }

    pub fn get_config_dir() -> PathBuf {
        return dirs::config_dir().unwrap();
    }

    pub fn get_downloads_dir() -> PathBuf {
        return dirs::download_dir().unwrap();
    }
}
