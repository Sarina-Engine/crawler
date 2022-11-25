use digikala_scraper::sender::scraper_rpc::scraper_service_server::{
    ScraperService, ScraperServiceServer,
};
use digikala_scraper::sender::scraper_rpc::{CategoryList, CommentList, DbResponse, ProductList};
use digikala_scraper::services::{get_categories, get_comments, get_products};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse()?;
    let service = ScraperRPCServer::default();

    let mut server = Server::builder();
    server
        .add_service(ScraperServiceServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}

#[derive(Default, Debug)]
struct ScraperRPCServer;

#[tonic::async_trait]
impl ScraperService for ScraperRPCServer {
    async fn send_comment(
        &self,
        req: Request<CommentList>,
    ) -> Result<Response<DbResponse>, Status> {
        unimplemented!()
    }

    async fn send_category(
        &self,
        req: Request<CategoryList>,
    ) -> Result<Response<DbResponse>, Status> {
        let cat_vec = req.into_inner().category_vec;
        for cat in cat_vec.iter() {
            get_products(&cat.code, cat.id).await.unwrap();
        }
        Ok(Response::new(DbResponse { status: true }))
    }

    async fn send_product(
        &self,
        req: Request<ProductList>,
    ) -> Result<Response<DbResponse>, Status> {
        let product_vec = req.into_inner().product_vec;
        for product in product_vec.iter() {
            println!("{}", product.id);
            match get_comments(product.id).await {
                Ok(_) => (),
                Err(err) => panic!("{}", err),
            }
        }
        Ok(Response::new(DbResponse { status: true }))
    }
}
