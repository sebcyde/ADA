use crate::utils::ai::ai::chat_with_ada;

pub mod config;
pub mod plugins;
pub mod utils;

#[tokio::main]
async fn main() {
    println!("\nLoading ADA...\n");

    chat_with_ada().await;
}
