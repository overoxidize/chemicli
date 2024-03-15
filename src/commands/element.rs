use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Args, Debug, Clone)]
pub struct ElementsArgs {
    pub atomic_symbol: String,
    #[arg(short = 'n', long, alias("atmn"))]
    pub atomic_number: bool,
    #[arg(long, short = 'w', alias("atmw"))]
    pub atomic_weight: bool,
    #[arg(long, short = 'l', alias("elvl"))]
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

#[derive(Debug, Clone, Deserialize, Serialize )]
pub struct Element {
    pub atomic_number: u64,
    pub atomic_weight: f64,
    pub energy_levels: Vec<String>, // should be Vec<u16>
    pub electronegativity: Option<f64>,
    pub series: String,
    pub date_discovered: String,
    pub discoverer: String,
    pub group: u64,
    pub period: u64
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
        group: u64,
        period: u64
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
            period
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
            group: 1,
            period: 1
        }
    }
}


// #[derive(Debug, Parser)]
// pub struct ElementCommands {
//     #[clap(subcommand)]
//     pub element_args: ElementArgs,
// }

// #[derive(Debug, Subcommand, Clone)]
// pub enum ElementArgs {
//     #[clap(alias = "atn")]
//     AtmNum { atmn: String },
//     #[clap(alias = "sym")]
//     Symbol { sym: String },
// }

// #[derive(Args, Debug, Clone)]

// pub struct Atmn {
//     pub atm_n: String,
// }
// #[derive(Args, Debug, Clone)]
// pub struct ElementSymbol {
//     pub sym: String,
// }
