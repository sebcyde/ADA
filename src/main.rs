use crate::plugins::backup::backup::backup_fc;
use crate::utils::ai::ai::primary_ada_builder;

pub mod config;
pub mod plugins;
pub mod utils;

#[tokio::main]
async fn main() {
    println!("\nLoading ADA...\n");

    backup_fc();

    // primary_ada_builder().await;
    // secondary_ada_builder().await;
}
