use chemicli::commands::element::*;
use chemicli::commands::metalloids::Metalloids;
use chemicli::commands::metals::Metals;
use chemicli::commands::non_metals::NonMetals;
use clap::{arg, Command};
use std::{collections::HashMap, hash::Hash};
//
type Element = HashMap<ElementName, ElementValue>;
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct EnergyLevels(Vec<u8>);
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct ElementValue {
    atomic_number: u16,
    energy_levels: EnergyLevels,
    element_name: ElementName,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]

struct ElementName(pub String);
fn cli() -> Command {
    Command::new("chem")
        .about("A CLI app for querying information about the periodic table.")
        .subcommand(
            Command::new("ele")
                .arg(arg!(<ELEMENT> "Element to query about").required(false))
                .subcommand(
                    Command::new("prop")
                        .short_flag('p')
                        .arg(arg!(atmn: <ATOMIC_NUMBER> "atomic number").required(false))
                        .arg(arg!(elvl: <ENERGY_LEVELS> "energy levels" ).required(false))
                        .arg(arg!(wght: <WEIGHT> "weight of element").required(false)),
                ),
        )
        .subcommand(
            Command::new("mtl")
                .arg(arg!(sg: <SUBGROUP> "Metal subgroup").required(false))
                .arg(arg!(num: <NUMBER> "Number of metals").required(false))
                .arg(arg!(lnth: <LANTHANIDES> "Lanthanides")),
        )
}

fn main() {
    let matches = cli().get_matches();

    dbg!(matches);

}
