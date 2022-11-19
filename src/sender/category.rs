use std::convert::From;
use tonic::Request;

use super::ScraperServiceClient;
use super::{Category, CategoryList};

use crate::scraper::CatTemplate;

pub async fn send(categories: Vec<CatTemplate>) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ScraperServiceClient::connect("http://[::1]:50051").await?;
    let category_list = categories.into_iter().map(|x| Category::from(x)).collect();
    let category_list = CategoryList {
        category_vec: category_list,
    };

    let request = Request::new(category_list);
    let response = client.send_category(request).await?;

    println!("{:?}", response);
    Ok(())
}

impl From<CatTemplate> for Category {
    fn from(cat: CatTemplate) -> Self {
        Category {
            id: cat.id,
            title_fa: cat.title_fa,
            code: cat.code,
            parent_cat: cat.parent_cat,
        }
    }
}
