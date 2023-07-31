use chemicli::commands::element::*;
use chemicli::commands::metalloids::Metalloids;
use chemicli::commands::metals::Metals;
use chemicli::commands::non_metals::NonMetals;
use clap::{arg, Arg, Command};
use std::{collections::HashMap, hash::Hash};
//
type Element = HashMap<ElementName, ElementValue>;
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct EnergyLevels(pub u8);
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
                        .help_template(
                            "\
                        {before-help}{name} {version}
                        {author-with-newline}{about-with-newline}
                        {usage-heading} {usage}
                        
                        {all-args}{after-help}
                        ",
                        )
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
    let mut element_map = Element::new();
    let mut symbol_map: HashMap<String, Element> = HashMap::new();
    let hy = ElementName("Hydrogen".to_string());
    let hy_e_levels = EnergyLevels(1);

    let hydrogen = ElementValue {
        atomic_number: 1,
        energy_levels: hy_e_levels,
        element_name: hy.clone(),
    };
    element_map.insert(hy, hydrogen);
    symbol_map.insert("H".to_string(), element_map.clone());

    let arg_list: Vec<_> = std::env::args().skip(2).collect();

    let (sym, e_prop_one) = (&arg_list[0], &arg_list[2]);

    println!("symbol_map is {:?}", symbol_map);

    match matches.subcommand() {
        Some(("ele", sub_matches)) => {
            let ele_sym = sym;

            if symbol_map.contains_key(ele_sym) {
                let queried_element = symbol_map.get(sym).expect("Symbol not found in map.");

                let sym_match = match "H" {
                    "H" => "Hydrogen".to_string(),
                    "He" => "Helium".to_string(),
                    "Li" => "Lithium".to_string(),
                    "Be" => "Beryllium".to_string(),
                    "B" => "Boron".to_string(),
                    "C" => "Carbon".to_string(),
                    "N" => "Nitrogen".to_string(),
                    "O" => "Oxygen".to_string(),
                    "F" => "Fluorine".to_string(),
                    "Ne" => "Neon".to_string(),
                    "Na" => "Sodium".to_string(),
                    "Mg" => "Magnesium".to_string(),
                    "Al" => "Aluminum".to_string(),
                    "Si" => "Silicon".to_string(),
                    "P" => "Phosphorus".to_string(),
                    "S" => "Sulfur".to_string(),
                    "Cl" => "Chlorine".to_string(),
                    "Ar" => "Argon".to_string(),
                    "K" => "Potassium".to_string(),
                    "Ca" => "Calcium".to_string(),
                    "Sc" => "Scandium".to_string(),
                    "Ti" => "Titanium".to_string(),
                    "V" => "Vanadium".to_string(),
                    "Cr" => "Chromium".to_string(),
                    "Mn" => "Manganese".to_string(),
                    "Fe" => "Iron".to_string(),
                    "Ni" => "Nickel".to_string(),
                    "Co" => "Cobalt".to_string(),
                    "Cu" => "Copper".to_string(),
                    "Zn" => "Zinc".to_string(),
                    "Ga" => "Gallium".to_string(),
                    "Ge" => "Germanium".to_string(),
                    "As" => "Arsenic".to_string(),
                    "Se" => "Selenium".to_string(),
                    "Br" => "Bromine".to_string(),
                    "Kr" => "Krypton".to_string(),
                    "Rb" => "Rubidium".to_string(),
                    "Sr" => "Strontium".to_string(),
                    "Y" => "Yttrium".to_string(),
                    "Zr" => "Zirconium".to_string(),
                    "Nb" => "Niobium".to_string(),
                    "Mo" => "Molybdenum".to_string(),
                    "Tc" => "Technetium".to_string(),
                    "Ru" => "Ruthenium".to_string(),
                    "Rh" => "Rhodium".to_string(),
                    "Pd" => "Palladium".to_string(),
                    "Ag" => "Silver".to_string(),
                    "Cd" => "Cadmium".to_string(),
                    "In" => "Indium".to_string(),
                    "Sn" => "Tin".to_string(),
                    "Sb" => "Antimony".to_string(),
                    "I" => "Iodine".to_string(),
                    "Te" => "Tellurium".to_string(),
                    "Xe" => "Xenon".to_string(),
                    "Cs" => "Cesium".to_string(),
                    "Ba" => "Barium".to_string(),
                    "La" => "Lanthanum".to_string(),
                    "Ce" => "Cerium".to_string(),
                    "Pr" => "Praseodymium".to_string(),
                    "Nd" => "Neodymium".to_string(),
                    "Pm" => "Promethium".to_string(),
                    "Sm" => "Samarium".to_string(),
                    "Eu" => "Europium".to_string(),
                    "Gd" => "Gadolinium".to_string(),
                    "Tb" => "Terbium".to_string(),
                    "Dy" => "Dysprosium".to_string(),
                    "Ho" => "Holmium".to_string(),
                    "Er" => "Erbium".to_string(),
                    "Tm" => "Thulium".to_string(),
                    "Yb" => "Ytterbium".to_string(),
                    "Lu" => "Lutetium".to_string(),
                    "Hf" => "Hafnium".to_string(),
                    "Ta" => "Tantalum".to_string(),
                    "W" => "Tungsten".to_string(),
                    "Re" => "Rhenium".to_string(),
                    "Os" => "Osmium".to_string(),
                    "Ir" => "Iridium".to_string(),
                    "Pt" => "Platinum".to_string(),
                    "Au" => "Gold".to_string(),
                    "Hg" => "Mercury".to_string(),
                    "Tl" => "Thallium".to_string(),
                    "Pb" => "Lead".to_string(),
                    "Bi" => "Bismuth".to_string(),
                    "Th" => "Thorium".to_string(),
                    "Pa" => "Protactinium".to_string(),
                    "U" => "Uranium".to_string(),
                    "Np" => "Neptunium".to_string(),
                    "Pu" => "Plutonium".to_string(),
                    "Am" => "Americium".to_string(),
                    "Cm" => "Curium".to_string(),
                    "Bk" => "Berkelium".to_string(),
                    "Cf" => "Californium".to_string(),
                    "Es" => "Einsteinium".to_string(),
                    "Fm" => "Fermium".to_string(),
                    "Md" => "Mendelevium".to_string(),
                    "No" => "Nobelium".to_string(),
                    "Lr" => "Lawrencium".to_string(),
                    "Rf" => "Rutherfordium".to_string(),
                    "Db" => "Dubnium".to_string(),
                    "Sg" => "Seaborgium".to_string(),
                    "Bh" => "Bohrium".to_string(),
                    "Hs" => "Hassium".to_string(),
                    "Mt" => "Meitnerium".to_string(),
                    "Ds" => "Darmstadtium".to_string(),
                    "Rg" => "Roentgenium".to_string(),
                    "Cn" => "Copernicium".to_string(),
                    "Nh" => "Nihonium".to_string(),
                    "Fl" => "Flerovium".to_string(),
                    "Mc" => "Moscovium".to_string(),
                    "Lv" => "Livermorium".to_string(),
                    "Ts" => "Tennessine".to_string(),
                    "Og" => "Oganesson".to_string(),
                    _ => "Element symbol not found".to_string(),
                };
                let ele_symbols: Vec<String> = [
                    //  0   1   2   3   4   5   6   7   8   9
                    "H ", "He", "Li", "Be", "B ", "C ", "N", "O ", "F ", //  0
                    "Ne", "Na", "Mg", "Al", "Si", "P ", "S ", "Cl", "Ar", "K ", //  1
                    "Ca", "Sc", "Ti", "V ", "Cr", "Mn", "Fe", "Co", "Ni", "Cu", //  2
                    "Zn", "Ga", "Ge", "As", "Se", "Br", "Kr", "Rb", "Sr", "Y ", //  3
                    "Zr", "Nb", "Mo", "Tc", "Ru", "Rh", "Pd", "Ag", "Cd", "In", //  4
                    "Sn", "Sb", "Te", "I ", "Xe", "Cs", "Ba", "La", "Ce", "Pr", //  5
                    "Nd", "Pm", "Sm", "Eu", "Gd", "Tb", "Dy", "Ho", "Er", "Tm", //  6
                    "Yb", "Lu", "Hf", "Ta", "W ", "Re", "Os", "Ir", "Pt", "Au", //  7
                    "Hg", "Tl", "Pb", "Bi", "Po", "At", "Rn", "Fr", "Ra", "Ac", //  8
                    "Th", "Pa", "U ", "Np", "Pu", "Am", "Cm", "Bk", "Cf", "Es", //  9
                    "Fm", "Md", "No", "Lr", "Rf", "Db", "Sg", "Bh", "Hs", "Mt", // 10
                    "Ds", "Rg", "Cn", "Nh", "Fl", "Mc", "Lv", "Ts", "Og", // 11
                ]
                .map(String::from)
                .to_vec();
                let property_query_result = queried_element
                    .get(&ElementName(sym_match.clone()))
                    .expect("Element property not found.");

                // let sym_match: String = (*sym.clone()).to_string();

                println!("queried element is: {:?}", queried_element);
                println!("Symbol to match is: {:?}", sym);
                println!("Queried property {:?}", e_prop_one);
                println!("Property query result {:?}", property_query_result);
            }
        }
        None => {}
        Some((_, _)) => {}
    }
}
