use super::Client;
use crate::errors::RequestError as Error;
use reqwest;
use serde_json::{json, Value};
use std::collections::HashMap;

pub enum Method {
    GET,
    POST,
}

pub struct Request {
    pub method: Method,
    pub path: String,
    pub body: Option<HashMap<String, Value>>,
}

impl Request {
    pub async fn call(&self, client: &Client) -> Result<Value, Error> {
        let response = match self.method {
            Method::GET => self.get(&client).await,
            Method::POST => self.post(&client).await,
        };

        response
    }

    async fn get(&self, client: &Client) -> Result<Value, Error> {
        let reqwest_client = reqwest::Client::new();
        let response = reqwest_client
            .get(format!("{}/{}", client.base_url, self.path))
            .header("Content-Type", "application/json")
            .header("X-AUTH-ACCESS-TOKEN", client.api_key.clone())
            .json(&self.body)
            .send()
            .await?
            .text()
            .await?;

        Ok(json!(response))
    }

    async fn post(&self, client: &Client) -> Result<Value, Error> {
        if let Some(body) = self.body.clone() {
            let reqwest_client = reqwest::Client::new();
            let response = reqwest_client
                .post(format!("{}/{}", client.base_url, self.path))
                .json(&body)
                .header("Content-Type", "application/json")
                .header("X-AUTH-ACCESS-TOKEN", client.api_key.clone())
                .send()
                .await?
                .text()
                .await?;
            Ok(json!(response))
        } else {
            Err(Error::NoBodyError(String::from("No request body provided")))
        }
    }
}
