use serde::Deserialize;

pub use scraper::{finalize_main_cats, finalize_sub_cats};

pub mod scraper {
    use super::{main, sub, CatTemplate};

    pub async fn finalize_main_cats() -> Result<Vec<CatTemplate>, Box<dyn std::error::Error>> {
        let main_cats = get_main_cats().await?.data.main_categories.categories;
        Ok(main_cats)
    }

    pub async fn finalize_sub_cats(
        category: &CatTemplate,
    ) -> Result<Vec<CatTemplate>, Box<dyn std::error::Error>> {
        let url = format!("https://api.digikala.com/v1/categories/{}/", category.code);
        let sub_cats: Vec<CatTemplate> = get_sub_cat(&url).await?.data.sub_categories;
        let sub_cats = sub_cats
            .into_iter()
            .map(|mut x| {
                x.parent_cat = category.id;
                x
            })
            .collect();
        Ok(sub_cats)
    }

    async fn get_main_cats() -> Result<main::MainTemplate, Box<dyn std::error::Error>> {
        let url = "https://api.digikala.com/v1/";
        let client = reqwest::Client::new();
        let body = client
            .get(url)
            .send()
            .await?
            .json::<main::MainTemplate>()
            .await?;

        Ok(body)
    }
    async fn get_sub_cat(url: &str) -> Result<sub::SubTemplate, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let body = client
            .get(url)
            .send()
            .await?
            .json::<sub::SubTemplate>()
            .await?;

        Ok(body)
    }
}
#[derive(Debug, Deserialize, Clone)]
pub struct CatTemplate {
    pub id: i32,
    pub title_fa: String,
    pub code: String,
    #[serde(default)]
    pub parent_cat: i32,
}

pub mod main {
    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct MainTemplate {
        pub status: i32,
        pub data: Data,
    }

    #[derive(Debug, Deserialize)]
    pub struct Data {
        pub main_categories: Categories,
    }

    #[derive(Debug, Deserialize)]
    pub struct Categories {
        pub categories: Vec<CatTemplate>,
    }
}

pub mod sub {
    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct SubTemplate {
        pub status: i32,
        pub data: Data,
    }

    #[derive(Debug, Deserialize)]
    pub struct Data {
        pub sub_categories: Vec<CatTemplate>,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn sub_cats() {
        let main_cats = finalize_main_cats().await.unwrap();
        let sub_cats = finalize_sub_cats(&main_cats[0]).await;
    }
}
