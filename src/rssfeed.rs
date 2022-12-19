use cached::proc_macro::once;
use feed_rs::model::Feed;
use feed_rs::parser::parse as ParseRSS;
use log::{debug, error};
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RssFeedConfig {
    pub url: String,
    pub title: String,
    pub icon: String,
    pub limit: usize,
}

#[derive(Debug, Serialize)]
pub enum RssFeedError {
    Download(String),
    Parse(String),
    Cache(String),
}

// cache each call for 1 minutes
#[once(time = 60, result = true, sync_writes = true)]
fn get_rss(url: String) -> Result<Feed, RssFeedError> {
    let parsed_url = Url::parse(&url).ok().unwrap();

    let client = reqwest::Client::new();

    debug!("Downloading {:?}", url);

    client
        .get(parsed_url.clone())
        .basic_auth(parsed_url.username(), parsed_url.password())
        .send()
        .map_err(|err| format!("{}", err))
        .map_err(RssFeedError::Download)
        .and_then(|mut rss| {
            debug!(
                "Downloaded feed url:{:?} headers:{:?}",
                rss.url(),
                rss.headers()
            );
            ParseRSS(&mut rss).map_err(|err| {
                error!("Could not parse feed:{:?} {:?}", rss.url(), err);
                RssFeedError::Parse("Could not parse feed".to_string())
            })
        })
}

impl RssFeedConfig {
    pub fn load(&self) -> Result<Feed, RssFeedError> {
        get_rss(self.url.clone())
    }
}
