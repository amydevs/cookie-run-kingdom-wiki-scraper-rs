use reqwest::Client;
use scraper::{element_ref::Select};

pub fn getf32fromsel(s: &mut Select) -> f32 {
    s.next().unwrap().inner_html().replace("\n", "").replace("%", "").parse().unwrap_or(0.0)
}

pub struct ClientWrapper {
    pub base_url: String,
    pub client: reqwest::Client,
}
impl ClientWrapper {
    pub fn new() -> Self {
        let inst = Self {
            base_url: "https://cookierunkingdom.fandom.com".to_owned(),
            client: reqwest::Client::new(),
        };
        return inst
    }
}