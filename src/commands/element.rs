use crate::utilities::utils::create_periodic_table;
use clap::Args;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Args, Debug, Clone)]
pub struct ElementsArgs {
    pub atomic_symbol: String,
    #[arg(short = 'n', long, alias("atmn"))]
    pub atomic_number: bool,
    #[arg(long, short = 'w', alias("atmw"))]
    pub atomic_weight: bool,
    #[arg(long, short = 'l', alias("elvls"))]
    pub energy_levels: bool,
    #[arg(long, short = 'e', alias("eneg"))]
    pub electronegativity: bool,
    #[arg(long, short)]
    pub series: bool,
    #[arg(long, short = 'd', alias("ddisc"))]
    pub date_discovered: bool,
    #[clap(long, short = 'D', alias("dscvr"))]
    pub discoverer: bool,
    #[arg(long, short = 'g')]
    pub group: bool,
    #[arg(long, short = 'p')]
    pub period: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Element {
    pub atomic_number: u64,
    pub atomic_weight: f64,
    pub energy_levels: Vec<String>, // should be Vec<u16>
    pub electronegativity: Option<f64>,
    pub series: String,
    pub date_discovered: String,
    pub discoverer: String,
    pub group: String,
    pub period: u64,
}

impl Element {
    pub fn new(
        atomic_number: u64,
        atomic_weight: f64,
        energy_levels: Vec<String>,
        electronegativity: Option<f64>,
        series: String,
        date_discovered: String,
        discoverer: String,
        group: String,
        period: u64,
    ) -> Self {
        Self {
            atomic_number,
            atomic_weight,
            energy_levels: energy_levels,
            electronegativity,
            series,
            date_discovered,
            discoverer,
            group,
            period,
        }
    }
}

impl Default for Element {
    fn default() -> Self {
        let series = String::from("Nonmetal");
        Self {
            atomic_number: 1,
            atomic_weight: 1.008,
            energy_levels: vec![String::from("1s1")],
            electronegativity: Some(2.20),
            series,
            date_discovered: String::from("1776"),
            discoverer: String::from("Henry Cavendish"),
            group: String::from("1"),
            period: 1,
        }
    }
}

pub fn exec(args: &ElementsArgs) {
    let periodic_table = create_periodic_table();
    let atomic_sym = &args.atomic_symbol;
    let element = periodic_table.get(atomic_sym).unwrap();
    let element_name = periodic_table.get(atomic_sym).unwrap()["element_name"]
        .to_string()
        .replace('\"', "");

    let set_atomic_number = || -> u64 {
        if args.atomic_number {
            periodic_table.get(atomic_sym).unwrap()["atomic_number"]
                .as_u64()
                .unwrap()
        } else {
            0
        }
    };

    let set_atomic_weight = || -> f64 {
        if args.atomic_weight {
            periodic_table.get(atomic_sym).unwrap()["atomic_weight"]
                .as_f64()
                .unwrap()
        } else {
            0.0
        }
    };

    let set_electronegativity = || -> f64 {
        if args.electronegativity {
            periodic_table.get(atomic_sym).unwrap()["electronegativity"]
                .as_f64()
                .unwrap()
        } else {
            0.0
        }
    };

    let set_series = || -> String {
        if args.series {
            periodic_table.get(atomic_sym).unwrap()["series"].to_string()
        } else {
            String::from("None")
        }
    };

    let set_date_discovered = || -> String {
        if args.date_discovered {
            periodic_table.get(atomic_sym).unwrap()["date_discovered"].to_string()
        } else {
            String::from("None")
        }
    };
    let set_discoverer = || -> String {
        if args.discoverer {
            periodic_table.get(atomic_sym).unwrap()["discoverer"].to_string()
        } else {
            String::from("None")
        }
    };
    let set_group = || -> String {
        if args.group {
            periodic_table.get(atomic_sym).unwrap()["group"]
                .to_string()
                .replace('\"', "")
        } else {
            format!("The element {element_name} is not a member of any group.")
        }
    };
    let set_period = || -> u64 {
        if args.period {
            periodic_table.get(atomic_sym).unwrap()["period"]
                .as_u64()
                .unwrap()
        } else {
            0
        }
    };
    let element_atomic_number = set_atomic_number();
    let element_atomic_weight = set_atomic_weight();
    let energy_levels = element["energy_levels"]
        .as_array()
        .unwrap()
        .to_owned()
        .into_iter()
        .collect::<Vec<Value>>()
        .iter()
        .map(|e| e.to_string())
        .map(|e| e.replace(&['\"', '\\'], ""))
        .collect::<Vec<String>>();

    let element_electronegativity = set_electronegativity();
    let element_series = set_series();
    let element_date_discovered = set_date_discovered().replace('\"', "");
    let element_discoverer = set_discoverer();
    let element_group = set_group();
    let element_period = set_period();

    if args.atomic_number {
        println!(
            "The atomic number of {} is {}.",
            element_name, element_atomic_number
        );
    }

    if args.atomic_weight {
        println!(
            "The atomic weight of {} is {}.",
            element_name, element_atomic_weight
        );
    }

    if args.energy_levels {
        let mut e_lvls = String::new();
        let num_e_lvls = energy_levels.len();

        if num_e_lvls == 1 {
            e_lvls.push_str(&(energy_levels[0].to_owned()));
        } else if num_e_lvls == 2 {
            e_lvls.push_str(&(energy_levels[0].to_owned() + ", "));
            e_lvls.push_str("and ");
            e_lvls.push_str(&(energy_levels[1].to_owned()));
        } else {
            for (idx, ele) in energy_levels.iter().enumerate() {
                e_lvls.push_str(&(ele.to_owned() + ", "));
                if idx == num_e_lvls - 2 {
                    e_lvls.push_str("and ");
                    e_lvls.push_str(ele);
                    break;
                }
            }
        }
        println!("The energy levels of {} are {}.", element_name, e_lvls);
    }

    if args.electronegativity {
        println!(
            "The electronegativity of {} is {}.",
            element_name, element_electronegativity
        );
    }
    if args.series {
        println!(
            "The element {} is a member of the {} series.",
            element_name, element_series
        );
    }
    if args.date_discovered {
        println!(
            "The element {} was discovered in {}.",
            element_name, element_date_discovered
        );
    }
    if args.discoverer {
        println!(
            "The person who discovered {} is {}.",
            element_name, element_discoverer
        );
    }
    if args.group {
        if element_group == "None".to_string() {
            println!("The element {} is not a member of any group.", element_name);
        } else {
            println!(
                "The element {} is a member of group {}.",
                element_name, element_group
            );
        }
    }
    if args.period {
        println!(
            "The element {} is a member of period {}.",
            element_name, element_period
        );
    }
}
