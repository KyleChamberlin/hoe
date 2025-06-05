use std::path::PathBuf;

use clap::Parser;
use color_eyre::Result;
use hoe_lib::commands::{
    self,
    Command::{List, Manage, New, Publish, Tag, Update},
};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[command(subcommand)]
    command: Option<commands::Command>,
    #[arg(short, long)]
    config: Option<PathBuf>,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    println!("hk test 123");

    match args.command {
        None => interactive_mode(),
        Some(command) => match command {
            New(new_args) => commands::new::new(new_args)?,
            List => todo!(),
            Tag => todo!(),
            Publish => todo!(),
            Update => todo!(),
            Manage => todo!(),
        },
    };

    Ok(())
}

fn interactive_mode() {
    print!("interactive mode is not currently supported.")
}
