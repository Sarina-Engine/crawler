pub mod category;
pub mod comment;
pub mod product;

use crate::scraper::{CatTemplate, CommentTemplate};
use scraper_rpc::scraper_service_client::ScraperServiceClient;
use scraper_rpc::DbResponse;
use scraper_rpc::{Category, CategoryList, Comment, CommentList, Product, ProductList};
use tonic::{Request, Response, Status};

pub mod scraper_rpc {
    tonic::include_proto!("scraper_rpc");
}

#[tonic::async_trait]
pub trait Send {
    async fn send(&self) -> Result<Response<DbResponse>, Status>;
}

pub async fn send() -> Result<(), Box<dyn std::error::Error>> {
    let client = ScraperServiceClient::connect("http://[::1]:50051").await?;

    Ok(())
}
