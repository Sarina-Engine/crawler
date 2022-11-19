use crate::scraper::{finalize_main_cats, finalize_sub_cats};
use crate::sender::category::send;

pub async fn get_categories() -> Result<(), Box<dyn std::error::Error>> {
    let main_cats = finalize_main_cats().await?;
    for category in main_cats {
        let sub_cats = match finalize_sub_cats(&category).await {
            Ok(res) => res,
            _ => continue,
        };
        send(vec![category.clone()]).await?;
        send(sub_cats).await?;
    }
    Ok(())
}
