pub mod scraper;
pub mod sender;
pub mod services;

use std;

pub mod product_send {
    tonic::include_proto!("product_send");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    services::get_categories().await?;
    Ok(())
}

fn get_product_id(url: &str) -> &str {
    url.split("/").nth(5).unwrap()
}
