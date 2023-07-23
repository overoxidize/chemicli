use chemicli::commands::element::*;
use chemicli::commands::metalloids::Metalloids;
use chemicli::commands::metals::Metals;
use chemicli::commands::non_metals::NonMetals;
use clap::{Arg, Args, Parser, Subcommand};
use std::collections::HashMap;
#[derive(Debug, Parser)]
#[clap(
    name = "chemicli",
    author,
    version,
    about = "A CLI for learning about chemistry"
)]
pub struct Chemicli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Provides information about a specific element
    #[clap(alias = "eln")]
    ElementName { name: String },
    #[clap(alias = "ele")]
    Element(ElementCommands),
    #[clap(alias = "met")]
    /// Provides a list of elements categorized as metals
    Metals(Metals),
    #[clap(alias = "nmet")]
    /// Provides a list of elements categorized as non-metals
    NonMetals(NonMetals),
    /// Provides a list of elements categorized as metalloids
    #[clap(alias = "mtlo")]
    Metalloids(Metalloids),
}

fn main() {
    let symbol_map: HashMap<String, ElementInfo> = HashMap::new();
    struct ElementInfo;
    let chemicli = Chemicli::parse();

    match chemicli.command {
        Commands::Element(element_commands) => {
            let query_type: Vec<String> = std::env::args().collect();
            let mut arg_vec: Vec<String> = Vec::new();
            println!("First arg is {:?}", query_type[1]);
            arg_vec.push(query_type[1].clone());
            let ElementCommands { element_args } = element_commands;
            match element_args {
                ElementArgs::AtmNum { atmn } => {
                    println!("{:?}", atmn);
                }
                ElementArgs::Symbol { sym } => {
                    if symbol_map.contains_key(&sym) {
                        todo!()
                    }
                    println!("{:?}", sym);
                }
            }
        }
        Commands::ElementName { name } => {
            todo!()
        }
        Commands::Metals(_) => {
            todo!()
        }
        Commands::NonMetals(_) => {
            todo!()
        }
        Commands::Metalloids(_) => {
            todo!()
        }
    }
}
