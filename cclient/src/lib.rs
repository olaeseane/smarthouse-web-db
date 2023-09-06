use std::error::Error;

use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};

pub struct CClient {
    pub base_url: Url,
    pub client: Client,
}

impl CClient {
    pub fn new(addr: &str) -> Result<Self, Box<dyn Error>> {
        let base_url = reqwest::Url::parse(addr)?;

        Ok(CClient {
            base_url,
            client: reqwest::Client::new(),
        })
    }

    pub async fn send_request<T, U>(&self, path: &str, body: Option<T>) -> Result<U, Box<dyn Error>>
    where
        T: Serialize,
        U: for<'de> Deserialize<'de>,
    {
        let path = self.base_url.join(path)?;

        let response = match body {
            Some(body) => self.client.post(path).json(&body).send().await?,
            None => self.client.get(path).send().await?,
        };

        Ok(response.error_for_status()?.json().await?)
    }
}
