use std::collections::HashMap;

use actix_web::web::Json;
use actix_web::{get, web, HttpResponse, Responder, Result};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HelpResponse {
    pub name: String,
    pub description: String,
    pub homepage: String,
    pub version: String,
    pub usage: Vec<HelpRoute>,
}

#[derive(Serialize, Deserialize)]
pub struct HelpRoute {
    pub path: String,
    pub query: HashMap<String, String>,
}

pub async fn index() -> impl Responder {
    let mut convert_query = HashMap::new();
    convert_query.insert("title".into(), "title of your feed".into());
    convert_query.insert("icon".into(), "icon number, e.g. i14532".into());
    convert_query.insert("url".into(), "http://my-domain.com/my-rss.xml".into());
    convert_query.insert("limit".into(), "10".into());
    HttpResponse::Ok().json(HelpResponse {
        name: env!("CARGO_PKG_NAME").into(),
        description: env!("CARGO_PKG_DESCRIPTION").into(),
        homepage: env!("CARGO_PKG_HOMEPAGE").into(),
        version: env!("CARGO_PKG_VERSION").into(),
        usage: vec![HelpRoute {
            path: "/convert?<query>".to_string(),
            query: convert_query,
        }],
    })

    /*Json(HelpResponse {
          "name": env!("CARGO_PKG_NAME"),
          "description": env!("CARGO_PKG_DESCRIPTION"),
          "homepage": env!("CARGO_PKG_HOMEPAGE"),
          "version": env!("CARGO_PKG_VERSION"),
          "usage": vec![json!({
          "path": "/convert?<query>",
          "query": RssFeedConfig {
          title: "title of your feed".to_string(),
          icon: "icon number, e.g. i14532".to_string(),
          url: "http://my-domain.com/my-rss.xml".to_string(),
                  limit: 10
              }
        })]
    })*/
}
