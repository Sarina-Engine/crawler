pub mod db;
pub mod deserializer;
pub mod model;
pub mod schema;

#[macro_use]
extern crate diesel;

use db::establish_connection;
use deserializer::{CTemplate, PTemplate};
use diesel::sqlite::SqliteConnection;
use model::{Comment, Product};

#[tokio::main]
async fn main() {
    let mut conn = establish_connection();
    let urlc = "https://api.digikala.com/v1/product/527621/comments/?page=1";
    let urlp = "https://api.digikala.com/v1/categories/men-socks-tights/search/";
    add_comments(urlc, &mut conn).await.unwrap();
    add_items(urlp, &mut conn).await.unwrap();
    let y = Comment::get(&mut conn, 1);

    println!("fact = {:#?}", y);
}

async fn add_comments(
    url: &str,
    conn: &mut SqliteConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let product_id: i32 = get_product_id(url).parse().unwrap();
    let product_comments = get_item_comments(url).await.unwrap();
    product_comments
        .data
        .comments
        .iter()
        .for_each(|x| Comment::add(x, product_id, conn));
    Ok(())
}

async fn add_items(
    url: &str,
    conn: &mut SqliteConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let products = get_items(url).await.unwrap();
    let products = products.data.products;
    products.iter().for_each(|x| Product::add(x, conn));
    Ok(())
}

async fn get_item_comments(url: &str) -> Result<CTemplate, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client.get(url).send().await?.json::<CTemplate>().await?;

    Ok(body)
}
async fn get_items(url: &str) -> Result<PTemplate, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client.get(url).send().await?.json::<PTemplate>().await?;

    Ok(body)
}

fn get_product_id(url: &str) -> &str {
    url.split("/").nth(5).unwrap()
}
