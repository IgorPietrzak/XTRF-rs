pub mod request;
use crate::errors::RequestError as Error;
use request::Request;
use serde_json::Value;

pub struct Client {
    api_key: String,
    base_url: String,
}

impl Client {
    pub fn new<T>(base_url: T, api_key: T) -> Self
    where
        T: Into<String>,
    {
        Client {
            base_url: base_url.into(),
            api_key: api_key.into(),
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
