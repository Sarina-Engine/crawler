pub use scraper::finalize_comments;
use serde::Deserialize;

mod scraper {
    use super::{CTemplate, CommentTemplate};

    pub async fn finalize_comments(
        p_id: i32,
    ) -> Result<Vec<CommentTemplate>, Box<dyn std::error::Error>> {
        let comments = get_comments(p_id).await?.data.comments;
        let comments = comments
            .into_iter()
            .map(|mut x| {
                x.product_id = p_id;
                x
            })
            .collect();
        Ok(comments)
    }

    async fn get_comments(p_id: i32) -> Result<CTemplate, Box<dyn std::error::Error>> {
        let url = format!("https://api.digikala.com/v1/product/{}/comments/", p_id);
        let client = reqwest::Client::new();
        let body = client.get(url).send().await?.json::<CTemplate>().await?;

        Ok(body)
    }
}

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
    pub title: String,
    pub body: String,
    pub rate: i32,
    #[serde(default)]
    pub product_id: i32,
}
