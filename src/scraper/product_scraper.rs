pub use scraper::finalize_products;
use serde::Deserialize;

pub mod scraper {
    use super::*;

    pub async fn finalize_products(
        cat_code: &str,
        cat_id: i32,
    ) -> Result<Vec<ProductTemplate>, Box<dyn std::error::Error>> {
        let products = get_products(cat_code).await?.data.products;
        let products = products
            .into_iter()
            .map(|mut x| {
                x.cat_id = cat_id;
                x
            })
            .collect();
        Ok(products)
    }

    async fn get_products(cat_code: &str) -> Result<PTemplate, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.digikala.com/v1/categories/{}/search/",
            cat_code
        );
        let client = reqwest::Client::new();
        let body = client.get(url).send().await?.json::<PTemplate>().await?;
        Ok(body)
    }
}

#[derive(Deserialize, Debug)]
struct PTemplate {
    status: i32,
    data: PData,
}
#[derive(Deserialize, Debug)]
struct PData {
    products: Vec<ProductTemplate>,
}
#[derive(Deserialize, Debug)]
pub struct ProductTemplate {
    pub id: i32,
    pub title_fa: String,
    pub rating: Rating,
    #[serde(default)]
    pub cat_id: i32,
}

#[derive(Deserialize, Debug)]
pub struct Rating {
    #[serde(default)]
    pub rate: f64,
    #[serde(default)]
    pub count: i32,
}
