extern crate reqwest;

use reqwest::Url;
use reqwest::Response;
use feed_rs::parser::parse as ParseRSS;
use APP_CACHE;
use std::result::Result::Ok;

pub use feed_rs::Feed;

#[derive(FromForm, Clone, Serialize, Debug)]
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
}


impl RssFeedConfig {
    pub fn load(&self) -> Result<Feed, RssFeedError> {
        {
            let mut cache = APP_CACHE.lock().unwrap();
            let cached_feed: Option<&Feed> = cache.get(&self.url);
            if cached_feed.is_some() {
                return Ok(cached_feed.unwrap().clone());
            }
        }

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
