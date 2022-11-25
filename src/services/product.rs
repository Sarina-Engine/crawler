use crate::scraper::finalize_products;
use crate::sender::product::send;

pub async fn get_products(cat_code: &str, cat_id: i32) -> Result<(), Box<dyn std::error::Error>> {
    let products = finalize_products(cat_code, cat_id).await?;
    send(products).await?;
    Ok(())
}
