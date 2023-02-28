mod new;

use clap::Subcommand;

pub use new::new;

#[derive(Debug, Subcommand)]
pub enum Commands {
    New(new::Args),
    List,
    Tag,
    Publish,
    Update,
    Manage,
}
