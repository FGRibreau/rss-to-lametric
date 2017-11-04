use rocket_contrib::{Json, Value};
use rssfeed::RssFeedConfig;


#[get("/")]
pub fn index() -> Json<Value> {
    Json(json!({
        "name": env!("CARGO_PKG_NAME"),
        "description": env!("CARGO_PKG_DESCRIPTION"),
        "homepage": env!("CARGO_PKG_HOMEPAGE"),
        "version":env!("CARGO_PKG_VERSION"),
        "usage":vec![json!({
            "path": "/convert?<query>",
            "query": RssFeedConfig {
                title:"title of your feed".to_string(),
                icon: "icon number, e.g. i14532".to_string(),
                url: "http://my-domain.com/my-rss.xml".to_string(),
                limit: 10
            }
      })]
  }))
}
