extern crate serde_json;

use actix_web::{http, test, web};
use actix_web::client::Client;
use super::*;

use crate::la_metric::LaMetricFrame;
use crate::la_metric::LaMetricResponse;
use crate::la_metric::TextFrame;

use crate::index::{index, HelpResponse};

lazy_static! {
    #[derive(Copy, Clone, Debug)]
    static ref RSS_FEED: String = "http://www.mocky.io/v2/59fcfedb310000cb1b4fc7a9".to_string();
}

#[actix_rt::test]
async fn test_main_route() {
    let mut app = test::init_service(App::new().route("/", web::get().to(index))).await;
    let req = test::TestRequest::with_header("content-type", "application/json").to_request();
    let resp: HelpResponse = test::read_response_json(&mut app, req).await;
    assert_eq!(resp.name, "rss-to-lametric");
}

/*
#[actix_rt::test]
fn test_custom_title_and_icon_with_empty_rss() {
    assert_eq!(
        get_lametric_response(format!(
            "/convert/?title=Custom&icon=icon&limit=0&url={}",
            &**RSS_FEED
        )),
        LaMetricResponse {
            frames: vec![
                LaMetricFrame::TextFrame(TextFrame {
                    text: "Custom".to_string(),
                    icon: Some("icon".to_string()),
                }),
            ],
        }
    );
}

#[actix_rt::test]
fn test_invalid_rss() {
    let resp: LaMetricResponse = get_lametric_response(format!(
        "/convert/?title=Custom&icon=icon&limit=0&url=http://127.0.0.1.com/plop"
    ));

    assert_eq!(resp.frames.len(), 2);

    let last = &resp.frames[1];

    let text_frame = match last {
        LaMetricFrame::TextFrame(text_frame) => text_frame,
    };

    assert!(text_frame.text.contains(
        "failed to lookup address information",
    ))
}

#[actix_rt::test]
fn test_valid_la_metric_data_output() {
    assert_eq!(
        get_lametric_response(format!(
            "/convert/?title=Ouest-France&icon=i14532&limit=4&url={}",
            &**RSS_FEED
        )),
        LaMetricResponse {
            frames: vec![
                LaMetricFrame::TextFrame(TextFrame {
                    text: "Ouest-France".to_string(),
                    icon: Some("i14532".to_string()),
                }),
                LaMetricFrame::TextFrame(TextFrame {
                    text: "Stade Rennais. Le président du club René Ruello annonce sa démission"
                        .to_string(),
                    icon: None,
                }),
                LaMetricFrame::TextFrame(TextFrame {
                    text: "Direction de LREM. 4 listes en compétition pour le bureau exécutif"
                        .to_string(),
                    icon: None,
                }),
                LaMetricFrame::TextFrame(TextFrame {
                    text: "La police de New York a un \"vrai dossier\" sur Weinstein".to_string(),
                    icon: None,
                }),
                LaMetricFrame::TextFrame(TextFrame {
                    text: "Ligue 1. Le Stade Rennais poursuit sa belle série face à Bordeaux"
                        .to_string(),
                    icon: None,
                }),
            ],
        }
    );
}

#[actix_rt::test]
fn test_valid_la_metric_json_output() {
    assert_eq!(
        get(&format!(
            "/convert/?title=Ouest-France&icon=i14532&limit=4&url={}",
            &**RSS_FEED
        )),
        "{\"frames\":[{\"text\":\"Ouest-France\",\"icon\":\"i14532\"},{\"text\":\"Stade Rennais. Le président du club René Ruello annonce sa démission\",\"icon\":null},{\"text\":\"Direction de LREM. 4 listes en compétition pour le bureau exécutif\",\"icon\":null},{\"text\":\"La police de New York a un \\\"vrai dossier\\\" sur Weinstein\",\"icon\":null},{\"text\":\"Ligue 1. Le Stade Rennais poursuit sa belle série face à Bordeaux\",\"icon\":null}]}"
    );
}

*/
