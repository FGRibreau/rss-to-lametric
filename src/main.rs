#![feature(plugin, custom_derive)]
#![feature(advanced_slice_patterns)]
#![feature(match_default_bindings)]

#![plugin(rocket_codegen)]

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

extern crate reqwest;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate feed_rs;

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

use rocket::Rocket;
use rocket_contrib::Json;

mod index;


#[get("/convert?<rss_feed>")]
fn convert(rss_feed: RssFeedConfig) -> Json<LaMetricResponse> {
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
    rocket::ignite().mount("/", routes![index::index, convert])
}

fn main() {
    rocket().launch();
}
