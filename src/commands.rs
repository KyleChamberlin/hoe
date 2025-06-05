pub mod new;

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
