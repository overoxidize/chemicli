use chemicli::commands::element::*;
use chemicli::commands::metalloids::Metalloids;
use chemicli::commands::metals::Metals;
use chemicli::commands::non_metals::NonMetals;
use clap::{Arg, Args, Parser, Subcommand, Command};
use std::collections::HashMap;


/// Type alias for a HashMap associating elements with their known properties and subsequent values.

type SymbolMap = HashMap<String, Option<ElementInfo>>;
struct ElementInfo {
    atmn: Option<u8>
}

#[derive(Debug, Parser)]
#[clap(
    name = "chemicli",
    author = "Lorenzo Evans <lorenzo.evans94@gmail.com>",
    version,
    about = "A CLI for querying the periodic table of elements."
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
 
}
