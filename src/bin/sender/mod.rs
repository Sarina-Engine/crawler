use digikala_scraper::services::get_categories;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    get_categories().await?;
    Ok(())
}
