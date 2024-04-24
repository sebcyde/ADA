#![allow(unused_variables)]

use crate::config::user_config::user_config::create_user_config;
use crate::plugins::backup::backup::{backup_all, backup_db};
use crate::utils::ai::ai::primary_ada_builder;

pub mod config;
pub mod plugins;
pub mod utils;

#[tokio::main]
async fn main() {
    println!("\nLoading ADA...\n");

    create_user_config();

    backup_db(plugins::backup::backup::COMPANY::FC);
    backup_all();

    // primary_ada_builder().await;
    // secondary_ada_builder().await;
}
