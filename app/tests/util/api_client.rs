use reqwest::RequestBuilder;

#[derive(Clone)]
pub struct ApiClient {
    pub(crate) app_address: String
}

impl ApiClient {

    fn api_client() -> reqwest::Client {
        reqwest::Client::builder()
            .build()
            .expect("Failed to build rest client")
    }

    pub fn post(&self, slug: &str) -> RequestBuilder {
        Self::api_client()
            .post(format!("{}{}", self.app_address, slug))
    }

    pub fn put(&self, slug: &str) -> RequestBuilder {
        Self::api_client()
            .put(format!("{}{}", self.app_address, slug))
    }

    pub fn get(&self, slug: &str) -> RequestBuilder {
        Self::api_client()
            .get(format!("{}{}", self.app_address, slug))
    }

    pub fn delete(&self, slug: &str) -> RequestBuilder {
        Self::api_client()
            .delete(format!("{}{}", self.app_address, slug))
    }

}