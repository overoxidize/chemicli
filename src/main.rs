mod commands;
mod utilities;
use crate::commands::element::{exec, ElementsArgs};
use crate::commands::metalloids::Metalloids;
use crate::commands::non_metals::NonMetals;
use crate::utilities::utils::create_periodic_table;
use clap::{Args, Command, FromArgMatches, Parser, Subcommand};
// use commands::metals::AlkaliMetalsArgs;
use serde_json::{Map, Value};
use std::sync::atomic;
use std::{collections::HashMap, hash::Hash};

// Roadmap
// Change all Ancient date-discoverered values to AD/BC
// Extract commands into files
// Use comfy-table to add minimal tui capability
// Flesh out metal commmads
// Documentation
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
    #[clap(subcommand)]
    #[command(name = "metals")]
    Metals(MetalsSubcommands),
    #[command(name = "metalloids")]
    Metalloids,
    #[clap(subcommand)]
    #[command(name = "nonmetals")]
    NonMetals,
}

#[derive(Debug, Clone, Subcommand)]
pub enum MetalsSubcommands {
    Alkali(AlkaliMetalsArgs),
    Alkaline(AlkalineMetalsArgs),
    Lanthanoids(LanthanoidMetalsArgs),
    Actinoids(ActinoidMetalsArgs),
    Transition(TransitionMetalsArgs),
    PostTransition(PostTransitionMetalsArgs),
}
#[derive(Debug, Clone, Args)]
pub struct AlkaliMetalsArgs {
    #[clap(short, long)]
    members: bool,
    #[clap(short, long)]
    number: bool,
}

#[derive(Debug, Clone, Args)]
pub struct AlkalineMetalsArgs {
    #[clap(short, long)]
    members: bool,
    #[clap(short, long)]
    number: bool,
}
#[derive(Debug, Clone, Args)]
pub struct LanthanoidMetalsArgs {
    #[clap(short, long)]
    members: bool,
    #[clap(short, long)]
    number: bool,
}
#[derive(Debug, Clone, Args)]
pub struct ActinoidMetalsArgs {
    #[clap(short, long)]
    members: bool,
    #[clap(short, long)]
    number: bool,
}
#[derive(Debug, Clone, Args)]
pub struct TransitionMetalsArgs {
    #[clap(short, long)]
    members: bool,
    #[clap(short, long)]
    number: bool,
}
#[derive(Debug, Clone, Args)]
pub struct PostTransitionMetalsArgs {
    #[clap(short, long)]
    members: bool,
    #[clap(short, long)]
    number: bool,
}

fn main() {
    let periodic_table = create_periodic_table();
    let p_table_obj = periodic_table.as_object();

    let chemicli = Chemicli::parse();
    // Use comfy-table or Tui if you're feeling bold
    match &chemicli.commands {
        Commands::Element(args) => {
            exec(args);
        }
        Commands::Metals(args) => {
            // let members = |args| -> Vec<String> {
            //     let mut members_vec = Vec::new();

            //     match args {
            //         MetalsSubcommands::Alkali( args ) => {
            //             if args.members {

            //             }
            //         }
            //     }
            // }
            match args {
                MetalsSubcommands::Alkali(args) => {
                    let periodic_map = p_table_obj.unwrap();
                    // let members_vec  = collect_members("Alkali Metal".to_string(), args.clone(), p_table_obj);
                    let mut members: Vec<String> = vec!["".to_string()];
                    if args.members {
                       members = get_members("Alkali Metal".to_string(), periodic_map);
                    }
                    let mut num_members = 0;
                    if args.number {
                        num_members = members.len();
                    }

                    for (idx, ele) in members.iter().enumerate() {
                        println!("Member number {} is {}.", idx, ele);
                    }

                    println!("The number of members in the Alkali group is {}.", num_members);
                }

                MetalsSubcommands::Alkaline(args) => {
                    let periodic_map = p_table_obj.unwrap();
                    // let members_vec  = collect_members("Alkali Metal".to_string(), args.clone(), p_table_obj);
                    let mut members: Vec<String> = vec!["".to_string()];
                    if args.members {
                       members = get_members("Alkaline Metal".to_string(), periodic_map);
                    }
                    let mut num_members = 0;
                    if args.number {
                        num_members = members.len();
                    }

                    for (idx, ele) in members.iter().enumerate() {
                        println!("Member number {} is {}.", idx, ele);
                    }

                    println!("The number of members in the Alkali group is {}.", num_members);
                }
                MetalsSubcommands::Actinoids(args) => {}
                MetalsSubcommands::Lanthanoids(args) => {}
                MetalsSubcommands::PostTransition(args) => {}
                MetalsSubcommands::Transition(args) => {}

                _ => todo!(),
            }
        }
        _ => todo!(),
    }
    // dbg!(chemicli);
}

fn get_members(series: String, p_table: &Map<String, Value>) -> Vec<String> {
    let mut members = Vec::new();
    for i in p_table.iter() {
        let val = i.1["series"].to_string().replace('\"', "");
        if val == series {
            members.push(i.1["element_name"].clone().to_string())
        }
    }

    members
}

