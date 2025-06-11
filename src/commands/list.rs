use std::fs::{self};

use color_eyre::Result;
use tabled::Table;
use walkdir::WalkDir;

use crate::seedling;

pub fn list() -> Result<()> {
    let garden_path = "./test_garden/";

    let entries = WalkDir::new(garden_path)
        .into_iter()
        .filter_map(|e| match e {
            Err(e) => {
                eprintln!("{e}");
                None
            }
            Ok(entry) => Some(entry),
        })
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| entry.file_name().to_string_lossy().ends_with(".md"))
        .filter_map(|entry| match fs::read(entry.path()) {
            Err(e) => {
                eprintln!("{e}");
                None
            }
            Ok(contents) => Some(contents),
        })
        .map(|a| String::from_utf8_lossy(&a).into_owned())
        .filter_map(extract_frontmatter)
        .map(frontmatter_to_seedling_details)
        // .map(|entry| File::open(entry.path()))
        // .filter_map(|r| match r {
        //     Err(e) => {
        //         eprintln!("{e}");
        //         None
        //     }
        //     Ok(file) => Some(file),
        // })
        .collect::<Vec<_>>();

    let table = Table::new(entries);
    println!("{table}");

    Ok(())
}

fn frontmatter_to_seedling_details(frontmatter: String) -> seedling::Details {
    serde_yaml::from_str(&frontmatter).expect("all seedlings should have valid frontmatter")
}

fn extract_frontmatter(_file_contents: String) -> Option<String> {
    Some(
        r###"
    title: The Title of the seedling
    description: A long and perfect description.
    created: 2025-06-06T12:54:44.045320785Z
    author: Kyle Chamberlin
    tags: 
    slug: seedling_slug
    "###
        .to_string(),
    )
}
