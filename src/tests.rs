use super::*;
use crate::index::{index, HelpResponse};
use actix_web::http::header::ContentType;
use actix_web::test;
use actix_web::test::{call_service, read_body_json};
use lazy_static::lazy_static;
use log::debug;

lazy_static! {
    #[derive(Copy, Clone, Debug)]
    static ref RSS_FEED: String = "http://www.mocky.io/v2/59fcfedb310000cb1b4fc7a9".to_string();
}

#[actix_web::test]
async fn test_main_route() {
    let req = test::TestRequest::default()
        .uri("/")
        .insert_header(ContentType::json())
        .to_request();

    let app = test::init_service(App::new().route("/", web::get().to(index))).await;
    let resp: HelpResponse = test::call_and_read_body_json(&app, req).await;

    assert_eq!(resp.name, "rss-to-lametric");
}

async fn get_lametric_response(uri: String) -> LaMetricResponse {
    let req = test::TestRequest::get().uri(&uri).to_request();
    let app = test::init_service(App::new().route("/convert/", web::get().to(convert))).await;
    let res = call_service(&app, req).await;
    let json = read_body_json(res).await;
    debug!("json: {:?}", json);
    json
}

#[actix_web::test]
async fn test_custom_title_and_icon_with_empty_rss() {
    assert_eq!(
        get_lametric_response(format!(
            "/convert/?title=Custom&icon=icon&limit=0&url={}",
            &**RSS_FEED
        ))
        .await,
        LaMetricResponse {
            frames: vec![LaMetricFrame::TextFrame(TextFrame {
                text: "Custom".to_string(),
                icon: Some("icon".to_string()),
            }),],
        }
    );
}

/*
#[actix_web::test]
async fn test_invalid_rss() {
    let resp: LaMetricResponse = get_lametric_response(
        "/convert/?title=Custom&icon=icon&limit=1&url=http%3A//127.0.0.1.com/plop".to_string(),
    )
    .await;

    assert_eq!(resp.frames.len(), 2);

    let LaMetricFrame::TextFrame(text_frame) = &resp.frames[1];

    assert!(text_frame
        .text

        .contains("failed to lookup address information",))
}
*/

#[actix_web::test]
async fn test_valid_la_metric_data_output() {
    assert_eq!(
        get_lametric_response(format!(
            "/convert/?title=Ouest-France&icon=i14532&limit=4&url={}",
            &**RSS_FEED
        ))
        .await,
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

#[actix_web::test]
async fn test_valid_la_metric_json_output() {
    assert_eq!(
        serde_json::to_string(&get_lametric_response(format!(
            "/convert/?title=Ouest-France&icon=i14532&limit=4&url={}",
            &**RSS_FEED
        )).await).unwrap(),
        "{\"frames\":[{\"text\":\"Ouest-France\",\"icon\":\"i14532\"},{\"text\":\"Stade Rennais. Le président du club René Ruello annonce sa démission\",\"icon\":null},{\"text\":\"Direction de LREM. 4 listes en compétition pour le bureau exécutif\",\"icon\":null},{\"text\":\"La police de New York a un \\\"vrai dossier\\\" sur Weinstein\",\"icon\":null},{\"text\":\"Ligue 1. Le Stade Rennais poursuit sa belle série face à Bordeaux\",\"icon\":null}]}"
    );
}
