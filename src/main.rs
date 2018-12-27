#![feature(custom_derive)]
#![feature(custom_attribute)]
#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use]
extern crate lazy_static;

extern crate reqwest;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde;


extern crate feed_rs;

extern crate lru_time_cache;


#[cfg(test)]
mod tests;

mod la_metric;

use la_metric::LaMetricFrame;
use la_metric::LaMetricResponse;
use la_metric::TextFrame;
use la_metric::FeedConvertCommand;
use la_metric::LaMetricFrames;

mod rssfeed;

use rssfeed::RssFeedConfig;
use rssfeed::RssFeedError;
use lru_time_cache::LruCache;

use rocket::Rocket;
use rocket_contrib::json::Json;
use rocket::request::Form;
use std::string::String;
use feed_rs::Feed;
use std::time::Duration;
use std::ops::Deref;
use std::sync::Mutex;


mod index;


lazy_static! {
    static ref APP_CACHE: Mutex<LruCache<String, Feed>> = Mutex::new(LruCache::<String, Feed>::with_expiry_duration_and_capacity(Duration::from_secs(3600), 50));
}


#[get("/convert?<rss_feed..>")]
fn convert(rss_feed: Form<RssFeedConfig>) -> Json<LaMetricResponse> {
    Json(
        rss_feed
            .load()
            .map(|feed| {
                LaMetricFrames::from(FeedConvertCommand {
                    feed: feed,
                    limit: rss_feed.limit,
                })
            })
            .or_else(
                |err: RssFeedError| -> Result<LaMetricFrames, RssFeedError> {
                    Ok(vec![
                        match err {
                            RssFeedError::DownloadErr(error) |
                            RssFeedError::ParseErr(error) => LaMetricFrame::TextFrame(TextFrame {
                                text: error.to_string(),
                                icon: None,
                            }),
                        },
                    ])
                },
            )
            .map(|mut gen_frames: LaMetricFrames| {
                let mut frames = vec![LaMetricFrame::TextFrame(TextFrame::from(rss_feed.clone()))];
                frames.append(&mut gen_frames);
                LaMetricResponse { frames: frames }
            })
            .unwrap(),
    )
}


fn rocket() -> Rocket {
    // Ensure APP_CACHE is bound
    let _ = APP_CACHE.deref();

    rocket::ignite().mount("/", routes![index::index, convert])
}

fn main() {
    rocket().launch();
}
