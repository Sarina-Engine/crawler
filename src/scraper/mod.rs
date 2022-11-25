mod category_scraper;
pub mod comment_scraper;
mod product_scraper;

pub use category_scraper::{finalize_main_cats, finalize_sub_cats, CatTemplate};
pub use comment_scraper::{finalize_comments, CommentTemplate};
pub use product_scraper::{finalize_products, ProductTemplate};
