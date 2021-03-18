#[macro_use]
extern crate lazy_static;

use std::ops::Deref;
use std::string::String;
use std::sync::{Mutex, RwLock};
use std::time::Duration;

use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::Result;
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use color_eyre::eyre::Result as AppResult;
use feed_rs::Feed;
use log::error;
use lru_time_cache::LruCache;

use la_metric::FeedConvertCommand;
use la_metric::LaMetricFrame;
use la_metric::LaMetricFrames;
use la_metric::LaMetricResponse;
use la_metric::TextFrame;
use rssfeed::RssFeedConfig;
use rssfeed::RssFeedError;

#[cfg(test)]
mod tests;

mod la_metric;

mod index;
mod rssfeed;

lazy_static! {
    static ref APP_CACHE: Mutex<LruCache<String, Feed>> = Mutex::new(LruCache::<String, Feed>::with_expiry_duration(Duration::from_secs(60))); // 1min
}

async fn convert(web::Query(rss_feed): web::Query<RssFeedConfig>) -> impl Responder {
    HttpResponse::Ok().json(
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
                    Ok(vec![match err {
                        RssFeedError::CacheErr(error)
                        | RssFeedError::DownloadErr(error)
                        | RssFeedError::ParseErr(error) => LaMetricFrame::TextFrame(TextFrame {
                            text: error.to_string(),
                            icon: None,
                        }),
                    }])
                },
            )
            .map(|mut gen_frames: LaMetricFrames| {
                let mut frames = vec![LaMetricFrame::TextFrame(TextFrame::from(rss_feed.clone()))];
                frames.append(&mut gen_frames);
                LaMetricResponse { frames }
            })
            .unwrap(),
    )
}

#[actix_web::main]
async fn main() -> AppResult<()> {
    pretty_env_logger::init();

    // Ensure APP_CACHE is bound
    let _ = APP_CACHE.deref();

    // index::index, convert
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index::index))
            .route("/convert", web::get().to(convert))
    })
    .bind(format!(
        "{}:{}",
        option_env!("HOST").unwrap_or("0.0.0.0"),
        option_env!("PORT").unwrap_or("8080")
    ))?
    .run()
    .await
    .map_err(|err| err.into())
}
