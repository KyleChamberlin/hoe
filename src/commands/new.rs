use chrono::{DateTime, Utc};
use clap::Parser;
use color_eyre::eyre::Result;
use slugify::slugify;

use crate::prompt;

#[derive(Debug, Parser)]
pub struct ProvidedArgs {
    name: String,
    #[arg(short, long)]
    description: Option<String>,
    #[arg(short, long)]
    tags: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct Args {
    name: String,
    description: String,
    tags: Vec<String>,
}

#[derive(Debug, Eq, PartialEq)]
struct Seedling {
    title: String,
    slug: String,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
    description: String,
    revision: usize,
    tags: Vec<String>,
}

pub fn new(p_args: ProvidedArgs) -> Result<()> {
    let args = resolve_new_args(p_args);
    let _seedling = Seedling {
        title: args.name.to_string(),
        slug: slugify!(&args.name),
        created: Utc::now(),
        updated: Utc::now(),
        description: args.description,
        revision: 0,
        tags: args.tags,
    };

    dbg!(_seedling);

    Ok(())
}

fn resolve_new_args(new_args: ProvidedArgs) -> Args {
    Args {
        name: match Some(new_args.name) {
            None => prompt::seedling::name(),
            Some(name) => name,
        },
        description: match new_args.description {
            None => prompt::seedling::description(),
            Some(description) => description,
        },
        tags: match new_args.tags {
            None => prompt::seedling::tags(),
            Some(tags) => tags,
        },
    }
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
