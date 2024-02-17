mod request;

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

    pub async fn make_call(url: &str) {}
}

// token: PFgbbzTP0WvHZa8W0qm3RBzGzf
