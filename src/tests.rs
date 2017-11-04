extern crate serde_json;

use rocket::local::Client;
use rocket::http::Status;
use la_metric::LaMetricResponse;
use la_metric::LaMetricFrame;
use la_metric::TextFrame;

lazy_static! {
    #[derive(Copy, Clone, Debug)]
    static ref RSS_FEED: String = "http://www.mocky.io/v2/59fcfedb310000cb1b4fc7a9".to_string();
}

fn get(url: String) -> LaMetricResponse {
    let rocket = super::rocket();
    let client = Client::new(rocket).expect("Rocket client");
    let mut res = client.get(url).dispatch();
    res.body_string()
        .map(|x: String| serde_json::from_str(&x).unwrap())
        .unwrap()
}

#[test]
fn test_custom_title_and_icon_with_empty_rss() {
    assert_eq!(
        get(format!(
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

#[test]
fn test_valid_la_metric_output() {
    assert_eq!(
        get(format!(
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
