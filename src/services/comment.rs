use crate::scraper::finalize_comments;
use crate::sender::comment::send;

pub async fn get_comments(p_id: i32) -> Result<(), Box<dyn std::error::Error>> {
    let comments = match finalize_comments(p_id).await {
        Ok(s) => s,
        Err(_) => return Ok(()),
    };
    send(comments).await?;
    Ok(())
}
