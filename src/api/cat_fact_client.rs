use crate::models::CatFact;

pub struct CatFactClient {
    client: reqwest::Client,
    base_url: String
}

impl CatFactClient {
    pub fn new(client: reqwest::Client) -> Self {
        CatFactClient {
            client,
            base_url: String::from("https://catfact.ninja"),
        }
    }

    pub async fn get_fact(&self, max_length: Option<u32>) -> Result<CatFact, reqwest::Error> {
        let url = match max_length {
            Some(max_length) => format!("{}/fact?max_length={}", self.base_url, max_length),
            None => format!("{}/fact", self.base_url),
        };

        let response = self
            .client
            .get(url)
            .send()
            .await?
            .error_for_status()?;
        
        let cat_fact = response
            .json::<CatFact>()
            .await?;

        Ok(cat_fact)
    }
}