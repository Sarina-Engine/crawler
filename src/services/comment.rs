use crate::scraper::finalize_comments;
use crate::sender::comment::send;

pub async fn get_comments(p_id: i32) -> Result<(), Box<dyn std::error::Error>> {
    let comments = finalize_comments(p_id).await?;
    send(comments).await?;
    Ok(())
}
