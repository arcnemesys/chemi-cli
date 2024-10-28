use crate::element{exec, ElementsArgs, create_periodic_table, get_members};
use clap::{Args, Command, Parser, Subcommand};
use serde_json::{Map, Value};
#[derive(Debug, Parser)]
#[command(
    name = "chemicli",
    author = "Lorenzo Evans <lorenzo.evans94@gmail.com>",
    version = "0.1.0",
    about = "A CLI for querying the periodic table of elements."
)]
pub struct Chemicli {
    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    #[command(name = "element", override_usage = " chemicli.exe element <ATOMIC_SYMBOL> [OPTIONS]")]
    Element(ElementsArgs),
}
fn main() {
    let periodic_table = create_periodic_table();
    let periodic_table_obj = periodic_table.as_object();
    let chemi_cli - Chemicli::parse();

    match &chemi_cli.commands {
        Commands::Element(args) => {
            exec(args);
        }
    }
    println!("Hello, world!");
}
