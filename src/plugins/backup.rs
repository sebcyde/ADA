pub mod backup {
    use std::{
        ffi::OsStr,
        path::PathBuf,
        sync::{Arc, Mutex},
        thread::JoinHandle,
    };

    use crate::config::{structs::UserConfig, user_config::user_config::get_user_config};

    use chrono::{DateTime, Datelike, Local, Timelike};

    pub enum COMPANY {
        FC,
        ES_UK,
        ES_US,
        RE,
    }

    fn get_current_date_time_as_string() -> String {
        let now: DateTime<Local> = Local::now();

        let day: u32 = now.day();
        let month: u32 = now.month();
        let year: i32 = now.year() % 100;
        let hour: u32 = now.hour();
        let minute: u32 = now.minute();
        let second: u32 = now.second();

        return format!(
            "{:02}{:02}{:02}{:02}{:02}{:02}",
            day, month, year, hour, minute, second
        );
    }

    pub fn backup_db(company: COMPANY) -> bool {
        // println!("Backing up database...");

        let user_config: UserConfig = get_user_config();

        let backups_dir: Option<PathBuf> = user_config.db_paths.backups_dir;
        let database_path: Option<PathBuf> = match company {
            COMPANY::ES_UK => user_config.db_paths.es_uk_db_path,
            COMPANY::ES_US => user_config.db_paths.es_us_db_path,
            COMPANY::FC => user_config.db_paths.fc_db_path,
            COMPANY::RE => user_config.db_paths.re_db_path,
        };

        // Paths not set
        if backups_dir.is_none() || database_path.is_none() {
            println!("Directories for database backup have not been set.\n");
            return false;
        };

        // Move file into backups dir
        let old_db_path: PathBuf = database_path.unwrap();

        if !old_db_path.exists() {
            println!("Database location is invalid\n");
            return false;
        }

        let old_db_name: &OsStr = old_db_path.file_name().unwrap();

        // get backup location
        let mut backup_dir: PathBuf = backups_dir.unwrap();
        match company {
            COMPANY::ES_UK => backup_dir.push("electric_shuffle_uk"),
            COMPANY::ES_US => backup_dir.push("electric_shuffle_us"),
            COMPANY::FC => backup_dir.push("flight_club"),
            COMPANY::RE => backup_dir.push("red_engine"),
        };

        if !std::path::Path::exists(&backup_dir) {
            std::fs::create_dir_all(&backup_dir)
                .expect("Failed to create flight club backup directory");
        }

        // move file to backups dir
        backup_dir.push(old_db_name);
        let old: PathBuf = backup_dir.clone();

        std::thread::sleep(std::time::Duration::from_millis(1000));
        std::fs::copy(&old_db_path, &backup_dir).expect("Failed to backup FC database");

        // Rename to current time
        let current_date_time: String = get_current_date_time_as_string().to_ascii_lowercase();
        backup_dir.set_file_name(format!("{}.db", current_date_time));

        std::fs::rename(&old, &backup_dir).expect("Failed to rename FC database");

        println!("Backup Complete.");
        return true;
    }

    pub fn backup_all() {
        let mut result: Vec<bool> = Vec::new();

        for i in 0..4 {
            match i {
                0 => {
                    //
                    println!("Backing up Electric Shuffle UK");
                    result.push(backup_db(COMPANY::ES_UK));
                }
                1 => {
                    //
                    println!("Backing up Electric Shuffle US");
                    result.push(backup_db(COMPANY::ES_US));
                }
                2 => {
                    //
                    println!("Backing up Flight Club");
                    result.push(backup_db(COMPANY::FC));
                }
                3 => {
                    //
                    println!("Backing up Red Engine");
                    result.push(backup_db(COMPANY::RE));
                }
                _ => println!("Loop out of bounds."),
            }
            println!("Backup complete.\n");
        }

        println!("Backup Results: {:?}\n", result);
    }

    pub fn backup_all_multithreaded() {
        let shared_result_state: Arc<Mutex<Vec<bool>>> = Arc::new(Mutex::new(Vec::new()));

        let es_uk_thread = spawn_thread("ES UK", COMPANY::ES_UK, Arc::clone(&shared_result_state));
        let es_us_thread = spawn_thread("ES US", COMPANY::ES_US, Arc::clone(&shared_result_state));
        let fc_uk_thread = spawn_thread("FC", COMPANY::FC, Arc::clone(&shared_result_state));
        let re_uk_thread = spawn_thread("RE", COMPANY::RE, Arc::clone(&shared_result_state));

        es_uk_thread.join().expect("Bug in ES UK thread");
        es_us_thread.join().expect("Bug in ES US thread");
        fc_uk_thread.join().expect("Bug in FC UK thread");
        re_uk_thread.join().expect("Bug in RE UK thread");

        println!("\nMulti-threaded backup result: {:?}", &shared_result_state);
    }

    fn spawn_thread(
        name: &str,
        company: COMPANY,
        shared_result_state: Arc<Mutex<Vec<bool>>>,
    ) -> JoinHandle<()> {
        std::thread::Builder::new()
            .name(name.to_string())
            .spawn(move || {
                let result = backup_db(company);
                shared_result_state.lock().unwrap().push(result);
            })
            .unwrap()
    }
}
