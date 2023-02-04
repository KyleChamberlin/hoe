use clap::Parser;
use hoe_lib::commands;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[command(subcommand)]
    command: commands::Commands,
}
fn main() {
    println!("Hello, world!");
}
