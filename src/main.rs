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
    command: commands::Command,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    dbg!(&args);

    match Some(args.command) {
        None => todo!(),
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

fn resolve_command(command: Option<commands::Command>) -> commands::Command {
    match command {
        None => choose_command(),
        Some(c) => c,
    }
}

fn choose_command() -> commands::Command {
    todo!()
}
