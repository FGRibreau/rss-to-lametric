//! # Rss-to-lametric
//!
//! Your favorite news (RSS feed) directly from your [LaMetric 🎩](https://store.lametric.com/?rfsn=853404.8b38b6)

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/FGRibreau/rss-to-lametric/master/docs/lametric-app.jpg",
    test(attr(forbid(warnings)))
)]
// https://github.com/maidsafe/QA/blob/master/Documentation/Rust%20Lint%20Checks.md
#![forbid(
    arithmetic_overflow,
    mutable_transmutes,
    no_mangle_const_items,
    unknown_crate_types
)]
#![deny(
    deprecated,
    improper_ctypes,
    missing_docs,
    non_shorthand_field_patterns,
    overflowing_literals,
    stable_features,
    unconditional_recursion,
    unknown_lints,
    unsafe_code,
    unused,
    unused_allocation,
    unused_attributes,
    unused_comparisons,
    unused_features,
    unused_parens,
    while_true,
    warnings
)]
#![warn(
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use actix_web::Result;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use color_eyre::eyre::Result as AppResult;

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

async fn convert(web::Query(rss_feed): web::Query<RssFeedConfig>) -> impl Responder {
    HttpResponse::Ok().json(
        rss_feed
            .load()
            .map(|feed| {
                LaMetricFrames::from(FeedConvertCommand {
                    feed,
                    limit: rss_feed.limit,
                })
            })
            .or_else(
                |err: RssFeedError| -> Result<LaMetricFrames, RssFeedError> {
                    Ok(vec![match err {
                        RssFeedError::Cache(error)
                        | RssFeedError::Download(error)
                        | RssFeedError::Parse(error) => LaMetricFrame::TextFrame(TextFrame {
                            text: error,
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

    // index::index, convert
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Compress::default())
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
