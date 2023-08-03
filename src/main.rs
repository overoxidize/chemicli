use chemicli::commands::element::*;
use chemicli::commands::metalloids::Metalloids;
use chemicli::commands::metals::Metals;
use chemicli::commands::non_metals::NonMetals;
use clap::{Arg, Args, Parser, Subcommand, Command};
use std::collections::HashMap;


/// Type alias for a HashMap associating elements with their known properties and subsequent values.

type SymbolMap = HashMap<String, Option<ElementInfo>>;
/// Newtype wrapping the atomic symbol of a given element.

struct ElementInfo {
    atmn: Option<u8>
}

#[derive(Debug, Parser)]
#[clap(
    name = "chemicli",
    author,
    version,
    about = "A CLI for learning about chemistry"
)]
pub struct Chemicli {
    #[clap(subcommand)]
    pub commands: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Provides information about a specific element based on
    /// the properties queried.
    #[clap(alias = "ele")]
    Element {ele: String, props: Vec<Option<String>>}

    // #[clap(alias = "eln")]
    // ElementName { name: String },
    // #[clap(alias = "ele")]
    // Element(ElementCommands),
    // #[clap(alias = "met")]
    // /// Provides a list of elements categorized as metals
    // Metals(Metals),
    // #[clap(alias = "nmet")]
    // /// Provides a list of elements categorized as non-metals
    // NonMetals(NonMetals),
    // /// Provides a list of elements categorized as metalloids
    // #[clap(alias = "mtlo")]
    // Metalloids(Metalloids),
}

fn symbol_mapper(query_element: String, properties: Vec<String>) -> SymbolMap {

    let atmn = String::from("atmn");
    let atw = String::from("atw");
    let elvls = String::from("elvls");

    let mut element_info = ElementInfo {
        atmn: None,

    };
    if properties.is_empty() {
        todo!()
    } else {
        let mut property_iter = properties.iter();
        let mut symbol_map = SymbolMap::new();
        symbol_map.insert(query_element, None);

        while let Some(prop) = property_iter.next() {
            match prop {
                Atmn => {
                    // symbol_map.entry(query_element)
                }
            }
        }

        return symbol_map
    }
}

fn main() {
    let mut symbol_map: HashMap<String, ElementInfo> = HashMap::new();

    let hydrogen = ElementInfo {
        atmn: Some(1)
    };


    let chemicli = Chemicli::parse();
    let atmn = String::from("atmn");
    let atw = String::from("atw");
    let elvls = String::from("elvls");
    match &chemicli.commands {

        Some(Commands::Element { ele: sym, props }) => {
            let query_element = sym.to_string();

            
            
            if !props.is_empty() {

                match &props[..] {
                    [atmn] => {
                        let atmn = atmn.as_ref().unwrap();
                        let symbol_map = symbol_mapper(query_element, vec![String::from(atmn)]);
                    }
                    [atmn, atw] => {

                    }
                    [atmn, atw, elvls] => {

                    }
                    &[] => {
                        println!("no properties queried");
                    }
                    &[_, _, _, _, ..] =>  {

                    }

                    // _ => ()
                }
            } else {
                // return all information about the element
            }


        
        }
        None => ()
    }
    dbg!(&chemicli);
    // match chemicli.command {
    //     Commands::Element(element_commands) => {
    //         let query_type: Vec<String> = std::env::args().collect();
    //         let mut arg_vec: Vec<String> = Vec::new();
    //         println!("First arg is {:?}", query_type[1]);
    //         arg_vec.push(query_type[1].clone());
    //         let ElementCommands { element_args } = element_commands;
    //         match element_args {
    //             ElementArgs::AtmNum { atmn } => {
    //                 println!("{:?}", atmn);
    //             }
    //             ElementArgs::Symbol { sym } => {
    //                 if symbol_map.contains_key(&sym) {
    //                     todo!()
    //                 }
    //                 println!("{:?}", sym);
    //             }
    //         }
    //     }
    //     Commands::ElementName { name } => {
    //         todo!()
    //     }
    //     Commands::Metals(_) => {
    //         todo!()
    //     }
    //     Commands::NonMetals(_) => {
    //         todo!()
    //     }
    //     Commands::Metalloids(_) => {
    //         todo!()
    //     }
    // }
}
