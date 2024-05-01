#![allow(unused_variables)]

use std::os::windows::thread;

use crate::config::user_config::user_config::create_user_config;
use crate::plugins::backup::backup::{backup_all, backup_all_multithreaded};
use crate::utils::ai::ai::primary_ada_builder;

pub mod config;
pub mod plugins;
pub mod utils;

#[tokio::main]
async fn main() {
    println!("\nLoading ADA...\n");

    // create_user_config(); // Leave in

    // Single Threaded
    println!("Running single threaded");
    let single_start_time = std::time::Instant::now();
    backup_all(); // Currently testing
    let single_end_time = std::time::Instant::now();
    let single_elapsed_time = single_end_time - single_start_time;
    println!("Elapsed time: {:?}\n\n", single_elapsed_time);

    std::thread::sleep(std::time::Duration::from_secs(1));

    // Multi-threaded
    println!("Running multi threaded");
    let multi_start_time = std::time::Instant::now();
    backup_all_multithreaded(); // Currently testing
    let multi_end_time = std::time::Instant::now();
    let multi_elapsed_time = multi_end_time - multi_start_time;
    println!("\nMulti Elapsed time: {:?}", multi_elapsed_time);

    // Need building
    // primary_ada_builder().await;
    // secondary_ada_builder().await;
}
