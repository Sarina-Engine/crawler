pub mod scraper;
pub mod sender;
pub mod services;

use std;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    services::get_categories().await?;
    Ok(())
}
