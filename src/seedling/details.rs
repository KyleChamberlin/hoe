use chrono::{DateTime, Utc};
use tabled::Tabled;

#[derive(Tabled, Debug, Eq, PartialEq)]
pub struct Details {
    name: String,
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
        Some(s) => format!("is valid thing = {}", s),
        None => format!("is not valid"),
    }
}

fn display_updated(updated: &Option<DateTime<Utc>>) -> String {
    match updated {
        Some(time) => format!("{:?}", time),
        None => format!(""),
    }
}

fn display_tags(tags: &Vec<String>) -> String {
    tags.join(";")
}
