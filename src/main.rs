use crate::utils::ai::ai::primary_ada_builder;

pub mod config;
pub mod plugins;
pub mod utils;

#[tokio::main]
async fn main() {
    println!("\nLoading ADA...\n");

    primary_ada_builder().await;
    secondary_ada_builder().await;
}
