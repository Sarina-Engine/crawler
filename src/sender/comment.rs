use std::convert::From;
use tonic::Request;

use super::ScraperServiceClient;
use super::{Comment, CommentList};
use crate::scraper::CommentTemplate;

pub async fn send(comments: Vec<CommentTemplate>) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ScraperServiceClient::connect("http://[::1]:50051").await?;
    let comment_list: Vec<Comment> = comments.into_iter().map(|x| x.into()).collect();
    let comment_list = CommentList {
        comment_vec: comment_list,
    };

    let request = Request::new(comment_list);
    let response = client.send_comment(request).await?;

    println!("{:?}", response);
    Ok(())
}

impl From<CommentTemplate> for Comment {
    fn from(c: CommentTemplate) -> Self {
        Comment {
            id: c.id,
            product_id: c.product_id,
            title: String::default(),
            body: c.body,
            rate: c.rate,
        }
    }
}
