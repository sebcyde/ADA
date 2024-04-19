pub mod backup {
    use std::path::PathBuf;

    use crate::config::{structs::UserConfig, user_config::user_config::get_user_config};

    use chrono::{DateTime, Datelike, Local, Timelike};

    fn get_current_date_time_as_string() -> String {
        let now: DateTime<Local> = Local::now();

        let day: u32 = now.day();
        let month: u32 = now.month();
        let year: i32 = now.year() % 100; // Get the last two digits of the year
        let hour: u32 = now.hour();
        let minute: u32 = now.minute();
        let second: u32 = now.second();

        return format!(
            "{:02}{:02}{:02}{:02}{:02}{:02}",
            day, month, year, hour, minute, second
        );
    }

    pub fn backup_fc() {
        let user_config: UserConfig = get_user_config();

        let backups_dir: Option<PathBuf> = user_config.db_paths.backups_dir;
        let fc_db_path: Option<PathBuf> = user_config.db_paths.fc_db_path;

        if backups_dir.is_none() || fc_db_path.is_none() {
            println!("Directories for FC database backup have not been set.");
            return;
        };

        let current_date_time: String = get_current_date_time_as_string();
        println!("Current Date Time: {}", &current_date_time);

        // Rename file and move into backups dir
    }
    pub fn backup_es() {}
    pub fn backup_re() {}
    pub fn backup_all() {}
}
