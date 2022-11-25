use std::convert::From;
use tonic::Request;

use super::ScraperServiceClient;
use super::{Product, ProductList};
use crate::scraper::ProductTemplate;

pub async fn send(products: Vec<ProductTemplate>) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ScraperServiceClient::connect("http://[::1]:50051").await?;
    let products: Vec<Product> = products.into_iter().map(|x| x.into()).collect();
    let product_list = ProductList {
        product_vec: products,
    };
    let request = Request::new(product_list);
    let response = client.send_product(request).await?;
    println!("{:?}", response);

    Ok(())
}

impl From<ProductTemplate> for Product {
    fn from(product: ProductTemplate) -> Self {
        Self {
            id: product.id,
            title_fa: product.title_fa,
            rate: product.rating.rate,
            count: product.rating.count,
            cat_id: product.cat_id,
            done: false,
        }
    }
}
