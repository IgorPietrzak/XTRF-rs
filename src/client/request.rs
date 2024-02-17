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
    method: Method,
    path: String,
    headers: Option<HashMap<String, String>>,
    body: Option<HashMap<String, Value>>,
}

impl Request {
    pub async fn call(&self, client: Client) {
        let response = match self.method {
            Method::GET => self.get(&client).await,
            Method::POST => self.post(&client).await,
        };
    }

    // these should return serde_json::Value

    async fn get(&self, client: &Client) -> Result<Value, Error> {
        let response = reqwest::get(format!("{}/{}", client.base_url, self.path))
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