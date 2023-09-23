use chrono::{DateTime, Utc};
use clap::Parser;
use color_eyre::eyre::Result;
use inquire::Text;
use slugify::slugify;

#[derive(Debug, Parser)]
pub struct Args {
    name: String,
    description: Option<String>,
    tags: Option<Vec<String>>,
}

#[derive(Debug)]
struct Seedling {
    title: String,
    slug: String,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
    description: String,
    revision: usize,
    tags: Vec<String>,
}

pub fn new(args: Args) -> Result<()> {
    let _seedling = Seedling {
        title: args.name.to_string(),
        slug: slugify!(&args.name),
        created: Utc::now(),
        updated: Utc::now(),
        description: match args.description {
            Some(desc) => desc,
            None => match Text::new("Description for new seedling:").prompt_skippable()? {
                Some(desc) => desc,
                None => String::new(),
            },
        },
        revision: 0,
        tags: match args.tags {
            Some(tags) => tags,
            None => match Text::new("tags:").prompt_skippable()? {
                Some(_) => todo!("parse tag list"),
                None => Vec::with_capacity(0),
            },
        },
    };

    dbg!(_seedling);

    Ok(())
}

// fn _create_seedling(_seedling: Seedling) -> Result<()> {
//     Ok(())
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn create_seedling_doesnt_prompt_when_all_params_provided() -> Result<()> {
//         create_seedling(Args {
//             name: "kyle".to_string(),
//             description: Some("longname".to_string()),
//             tags: Some(vec!["tag".to_string(), "here".to_string()]),
//         })
//     }
// }
