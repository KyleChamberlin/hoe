use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    New,
    List,
    Edit,
    Remove,
    Garden,
}
