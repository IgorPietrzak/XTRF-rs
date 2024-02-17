mod request;
use crate::errors::RequestError as Error;
use request::Request;
use serde_json::Value;

pub struct Client {
    api_key: String,
    base_url: String,
}

impl Client {
    pub fn new(base_url: &str, api_key: &str) -> Client {
        Client {
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
        }
    }

    pub async fn make_call(&self, req: Request) -> Result<Value, Error> {
        let res = req.call(self).await;
        match res {
            Ok(res) => return Ok(res),
            Err(e) => return Err(Error::from(e)),
        }
    }
}

