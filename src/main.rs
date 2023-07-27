use chemicli::commands::element::*;
use chemicli::commands::metalloids::Metalloids;
use chemicli::commands::metals::Metals;
use chemicli::commands::non_metals::NonMetals;
use clap::{arg, Command, Arg};
use std::collections::HashMap;


fn cli() -> Command {
    Command::new("chem")
        .about("A CLI app for querying information about the periodic table.")
        .subcommand_required(true)
        .subcommand(
            Command::new("ele")
                    .arg(arg!(<ELEMENT> "Element to query about"))
                    .subcommand(Command::new("prop").args_conflicts_with_subcommands(true)
                    .short_flag('p')
                    .arg(arg!(atmn: <ATOMIC_NUMBER> "atomic number").required(false))
                    .arg(arg!(atw: <ATOMIC_WEIGHT> "atomic weight" ).required(false))
                    .arg(arg!(wght: <WEIGHT> "weight of element").required(false))))
        .subcommand( 
                Command::new("mtl")
                        .arg(arg!(sg: <SUBGROUP> "Metal subgroup"))
                        .subcommand(Command::new("lanthanides").args_conflicts_with_subcommands(true)
                        .short_flag('l')))
                    //     .subcommand(
                    // Command::new("sg")
                    //         .arg(arg!(lnth: <LANTHANIDES> "Lanthanides")))).args_conflicts_with_subcommands(true)
        
}

fn main () {

    let matches = cli().get_matches();

    let symbol_map: HashMap<String, String> = HashMap::new();
    let arg_list: Vec<_> = std::env::args().collect();

    let sym = &arg_list[2];

    println!("sym is {:?}", sym);
    match matches.subcommand() {
        Some(("ele", sub_matches)) => {
                // let sym = sub_matches.get_raw("H").unwrap().to_owned();
            let sub_m = sub_matches.subcommand();

            // println!("args {:?}", sub_m);
            // match sub_m {
            //     ("H", sub) => {
            //         println!("Got it")
            //     }
            //     (_,_) => ()
            // }
            // match sub_m {
            //     ("atmn", sub) => {

            //     }
            //     ("atw", sub) => {

            //     }
            //     (_,_) => todo!()
            // }
        }
        None => {},
        Some((_, _)) => {}
    }

}