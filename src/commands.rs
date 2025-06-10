mod list;
mod new;

pub use list::list;
pub use new::new;

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Command {
    New(new::ProvidedArgs),
    List,
    Tag,
    Publish,
    Update,
    Manage,
}
