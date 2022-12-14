use std::convert::From;
use tonic::Request;

use super::ScraperServiceClient;
use super::{Feature, FeatureList, Product, ProductList};
use crate::scraper::finalize_features;
use crate::scraper::{Attr, ProductTemplate};

pub async fn send(products: Vec<ProductTemplate>) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ScraperServiceClient::connect("http://[::1]:50051").await?;
    let mut products = products;
    for product in &mut products {
        let features = match finalize_features(product).await {
            Ok(x) => x,
            Err(_) => continue,
        };
        let features = features.into_iter().map(|x| x.into()).collect();
        let ft_list = FeatureList { ft: features };
        let req = Request::new(ft_list);
        let response = client.send_feature(req).await?;
        println!("{:?}, For product: {}", response, product.id);
    }

    let products: Vec<Product> = products.into_iter().map(|x| x.into()).collect();
    let product_list = ProductList {
        product_vec: products,
    };
    let request = Request::new(product_list);
    let response = client.send_product(request).await?;
    println!("{:?}", response);

    Ok(())
}

impl From<Attr> for Feature {
    fn from(mut feature: Attr) -> Self {
        Self {
            product_id: feature.product_id,
            name: feature.title,
            value: feature.values.pop().unwrap(),
        }
    }
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
