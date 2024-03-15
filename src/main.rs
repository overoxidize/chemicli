mod commands;
mod periodic_table;
use crate::periodic_table::{create_periodic_table};
use crate::commands::element::{Element, ElementsArgs};
use crate::commands::metalloids::Metalloids;
use crate::commands::non_metals::NonMetals;
use clap::{Args, Command, Parser, Subcommand};
use serde_json::Value;
use std::sync::atomic;
use std::{collections::HashMap, hash::Hash};
#[derive(Debug, Parser)]
#[command(
    name = "chemicli",
    author = "Lorenzo Evans <lorenzo.evans94@gmail.com>",
    version = "0.1.0",
    about = "A CLI for querying the periodic table of elements."
)]
#[command(next_line_help = true)]
pub struct Chemicli {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    Element(ElementsArgs),
    Metals(MetalsArgs)
}

#[derive(Debug, Clone, Args)]
pub struct MetalsArgs {
    #[arg(long, short)]
    class: bool
}


fn main() {

    // We store the serde'd json file in a `periodic_table` variable.
    let periodic_table = create_periodic_table();
    // We query the variable for the value associated with the atomic symbol.
    let element = periodic_table.get("Be").unwrap().clone();

    
    
    let helium = Element::new(
        element["atomic_number"].as_u64().unwrap(),
        element["atomic_weight"].as_f64().unwrap(),
        energy_levels,
        element["electronegativity"].as_f64(),
        element["series"].to_string(),
        element["date_discovered"].to_string(),
        element["discoverer"].to_string(),
        element["group"].as_u64().unwrap(),
        element["period"].as_u64().unwrap()

    );
    let chemicli = Chemicli::parse();
    // Use comfy-table or Tui if you're feeling bold
    match &chemicli.commands {
        Commands::Element( args ) => {
            
            let atomic_sym = &args.atomic_symbol;
            // let atomic_number = &args.atomic_number;
            // let atomic_weight = &args.atomic_weight;
            // let energy_levels = &args.energy_levels;
            // let electronegativity = &args.electronegativity;
            // let series = &args.series;
            // let date_discovered = &args.date_discovered;
            // let discoverer = &args.discoverer;
            // let group = &args.group;
            // let period = &args.period;
            
            let element = periodic_table.get(atomic_sym).unwrap();
            
            
            let set_atomic_number = |atomic_number| -> u64 {
                if args.atomic_number {
                    periodic_table.get(atomic_sym).unwrap()["atomic_number"].as_u64().unwrap()
                } else {
                    0
                }
            };
            let mut element_atomic_number = set_atomic_number(*atomic_number);

            let set_atomic_weight = |atomic_weight| -> f64 {
                if args.atomic_weight {
                    periodic_table.get(atomic_sym).unwrap()["atomic_weight"].as_f64().unwrap()
                } else {
                    0.0
                }
            };
            let mut element_atomic_weight = set_atomic_weight(*atomic_weight);
            
            
            let energy_levels = element["energy_levels"]
                .as_array()
                .unwrap()
                .to_owned()
                .into_iter()
                .collect::<Vec<Value>>()
                .iter()
                .map(|e| e.to_string())
                .map(|e| e.replace(&['\"', '\\',], ""))
                .collect::<Vec<String>>();
            
            let set_electronegativity = |electronegativity: &bool| -> f64 {
                if args.electronegativity {
                    periodic_table.get(atomic_sym).unwrap()["electronegativity"].as_f64().unwrap()
                } else {
                    0.0
                }
            };
            let mut element_electronegativity = set_electronegativity(electronegativity);
            let set_series = |series: &bool| -> String {
                if args.series {
                    periodic_table.get(atomic_sym).unwrap()["series"].to_string()
                } else {
                    String::from("None")
                }
            };
            let mut element_series = set_series(series);
            let set_date_discovered = |date_discovered: &bool| -> String {
                if args.date_discovered {
                    periodic_table.get(atomic_sym).unwrap()["date_discovered"].to_string()
                } else {
                    String::from("None")
                }
            };
            let mut element_series = set_series(series);
            let set_group = |group| -> u64 {
                if args.group {
                    periodic_table.get(atomic_sym).unwrap()["group"].as_u64().unwrap()
                } else {
                    0
                }
            };
            let mut element_group = set_group(*group);
            let set_period = |period| -> u64 {
                if args.period {
                    periodic_table.get(atomic_sym).unwrap()["period"].as_u64().unwrap()
                } else {
                    0
                }
            };
            let mut element_period = set_period(*period);

            // let result = check_bool_arg(atomic_number);
            
            // let mut element_energy_levels;

            // if *energy_levels {
            //     element_energy_levels = element.energy_levels;
            // }
            
            // let element_electronegativity = if *electronegativity {
            //     element.electronegativity
            // } else {
            //     Some(0.0)
            // };

            // let element_series = if *series {
            //     element.series
            // } else {
            //     String::from("None")
            // };
            // let element_date_discovered = if *date_discovered {
            //     element.date_discovered 
            // } else {
            //     String::from("None")
            // };
            // let element_discoverer = if *discoverer {
            //     element.discoverer
            // } else {
            //     String::from("None")
            // };
            // let element_group = if *group {
            //     element.group
            // } else {
            //     0
            // };
            // let element_period = if *period {
            //     element.period
            // } else {
            //     0
            // };



            
        },
        Commands::Metals ( args ) => {

        }
    }
    // dbg!(chemicli);
}
fn check_bool_arg(arg: &bool) {

    if arg {

    } 
}
