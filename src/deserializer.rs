use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CTemplate {
    pub status: i32,
    pub data: CData,
}

#[derive(Deserialize, Debug)]
pub struct CData {
    pub comments: Vec<CommentTemplate>,
}

#[derive(Deserialize, Debug)]
pub struct CommentTemplate {
    pub id: i32,
    pub title: Option<String>,
    pub body: String,
    pub rate: i32,
}

#[derive(Deserialize, Debug)]
pub struct PTemplate {
    pub status: i32,
    pub data: Pdata,
}

#[derive(Deserialize, Debug)]
pub struct Pdata {
    pub products: Vec<ProductTemplate>,
}

#[derive(Deserialize, Debug)]
pub struct ProductTemplate {
    pub id: i32,
    pub title_fa: String,
}
