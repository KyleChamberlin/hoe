use clap::Parser;
use color_eyre::Result;
use hoe_lib::commands::{
    self,
    Commands::{List, Manage, New, Publish, Tag, Update},
};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[command(subcommand)]
    command: commands::Commands,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    dbg!(&args);

    match args.command {
        New(new_args) => commands::new(new_args)?,
        List => todo!(),
        Tag => todo!(),
        Publish => todo!(),
        Update => todo!(),
        Manage => todo!(),
    };

    Ok(())
}
