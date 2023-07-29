use chemicli::commands::element::*;
use chemicli::commands::metalloids::Metalloids;
use chemicli::commands::metals::Metals;
use chemicli::commands::non_metals::NonMetals;
use clap::{arg, Command, Arg};
use std::{collections::HashMap, hash::Hash};
// 
fn cli() -> Command {
    Command::new("chem")
        .about("A CLI app for querying information about the periodic table.")
        .subcommand(
            Command::new("ele")
                    .arg(arg!(<ELEMENT> "Element to query about"))
                    .subcommand(Command::new("prop")
                    .short_flag('p')
                    .arg(arg!(atmn: <ATOMIC_NUMBER> "atomic number").required(false))
                    .arg(arg!(elvl: <ENERGY_LEVELS> "energy levels" ).required(false))
                    .arg(arg!(wght: <WEIGHT> "weight of element").required(false))))
        .subcommand( 
                Command::new("mtl")
                        .arg(arg!(sg: <SUBGROUP> "Metal subgroup").required(false))
                        .arg(arg!(num: <NUMBER> "Number of metals").required(false))
                        .subcommand(Command::new("lanthanides")))
                    //     .subcommand(
                    // Command::new("sg")
                    //         .arg(arg!(lnth: <LANTHANIDES> "Lanthanides")))).args_conflicts_with_subcommands(true)
        
}


type Element = HashMap<ElementName, ElementValue>;
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct EnergyLevels(u8);
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct ElementValue {
    atomic_number: u16,
    energy_levels: EnergyLevels,
    element_name: ElementName,
    
}


#[derive(PartialEq, Eq, Hash, Clone, Debug)]

struct ElementName(String);
fn main () {
    let mut element_map = Element::new();
    let mut symbol_map: HashMap<String, Element> = HashMap::new();
    let hy = ElementName("Hydrogen".to_string());
    let hy_e_levels = EnergyLevels(1);
    
    let hydrogen = ElementValue {
        atomic_number: 1,
        energy_levels: hy_e_levels,
        element_name: hy.clone()
    };
    symbol_map.insert("H".to_string(), element_map.clone());
    element_map.insert(hy, hydrogen);


    let arg_list: Vec<_> = std::env::args().skip(2).collect();

    let matches = cli().get_matches();

    let sym = &arg_list[0];

    let e_prop_one = &arg_list[2];

    println!("sym is {:?}", sym);
    
    match matches.subcommand() {
        Some(("ele", sub_matches)) => {
            let ele_sym = Some(sym).unwrap();
      
            if symbol_map.contains_key(ele_sym) {
                let queried_element = symbol_map.get(sym).expect("Symbol not found in map.");
                let property_query_result = queried_element.get(&ElementName(e_prop_one.to_string())).expect("Element property not found.");
                
                println!("queried element is: {:?}", queried_element);
                println!("first requested element property is: {:?}", e_prop_one);
                println!("Queried property {:?} is {:?}", e_prop_one, property_query_result);
            }

        }
        None => {},
        Some((_, _)) => {}
    }

}