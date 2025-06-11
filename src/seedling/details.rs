use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Tabled, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Details {
    title: String,
    created: DateTime<Utc>,
    #[tabled(display = "display_updated")]
    updated: Option<DateTime<Utc>>,
    description: String,
    #[tabled(display = "display_tags")]
    tags: Vec<String>,
    slug: String,
    #[tabled(display = "display_author")]
    author: Option<String>,
}

fn display_author(o: &Option<String>) -> String {
    match o {
        Some(s) => s.to_string(),
        None => "".to_string(),
    }
}

fn display_updated(updated: &Option<DateTime<Utc>>) -> String {
    match updated {
        Some(time) => format!("{:?}", time),
        None => String::new(),
    }
}

fn display_tags(tags: &[String]) -> String {
    tags.join(";")
}
