use crate::rssfeed::{Feed, RssFeedConfig};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum LaMetricFrame {
    TextFrame(TextFrame),
}

pub type LaMetricFrames = Vec<LaMetricFrame>;

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct LaMetricResponse {
    pub frames: LaMetricFrames,
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct TextFrame {
    pub text: String,
    pub icon: Option<String>,
}

impl TextFrame {
    pub fn new(text: String, icon: String) -> TextFrame {
        TextFrame {
            text: text,
            icon: Some(icon),
        }
    }
}

impl From<RssFeedConfig> for TextFrame {
    fn from(feed: RssFeedConfig) -> Self {
        TextFrame::new(feed.title, feed.icon)
    }
}

pub struct FeedConvertCommand {
    pub feed: Feed,
    pub limit: usize,
}

impl From<FeedConvertCommand> for LaMetricFrames {
    fn from(feed_convert_command: FeedConvertCommand) -> Self {
        feed_convert_command
            .feed
            .entries
            .iter()
            .take(feed_convert_command.limit)
            .map(|entry| {
                LaMetricFrame::TextFrame(TextFrame {
                    text: entry.title.clone().unwrap_or("".to_string()),
                    icon: None,
                })
            })
            .collect::<Vec<LaMetricFrame>>()
    }
}
