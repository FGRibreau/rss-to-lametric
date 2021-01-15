use std::result::Result::Ok;
use std::sync::{Mutex, RwLock};
pub use feed_rs::Feed;
use feed_rs::parser::parse as ParseRSS;
use lru_time_cache::LruCache;
use reqwest::Response;
use reqwest::Url;
use serde_derive::{Serialize, Deserialize};
use crate::APP_CACHE;
use log::debug;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RssFeedConfig {
    pub url: String,
    pub title: String,
    pub icon: String,
    pub limit: usize,
}

#[derive(Debug, Serialize)]
pub enum RssFeedError {
    DownloadErr(String),
    ParseErr(String),
    CacheErr(String)
}


impl RssFeedConfig {
    pub fn load(&self) -> Result<Feed, RssFeedError> {
        {
            let mut cache = APP_CACHE.lock().unwrap();
            let cached_feed: Option<&Feed> = cache.get(&self.url);
            if cached_feed.is_some() {
                debug!("Found item {:?} in cache", self.url);
                return Ok(cached_feed.unwrap().clone());
            }
        }

        debug!("Downloading {:?}", self.url);
        self.download()
            .and_then(|rss| self.parse(rss))
            .and_then(|feed| {
                APP_CACHE.lock().unwrap().insert(self.url.clone(), feed.clone());
                Ok(feed)
            })
    }

    fn download(&self) -> Result<Response, RssFeedError> {
        let parsed_url = Url::parse(&self.url).ok().unwrap();

        let client = reqwest::Client::new();

        client
            .get(parsed_url.clone())
            .basic_auth(parsed_url.username(), parsed_url.password())
            .send()
            .map_err(|err| format!("{}", err))
            .map_err(RssFeedError::DownloadErr)
    }

    fn parse(&self, mut rss: reqwest::Response) -> Result<Feed, RssFeedError> {
        ParseRSS(&mut rss).ok_or(RssFeedError::ParseErr("Could not parse feed".to_string()))
    }
}
