use clap::{Parser, Args, Subcommand};

#[derive(Debug, Parser)]
pub struct ElementArgs {
    #[clap(subcommand)]
    pub element_commands: ElementCommands
}

#[derive(Debug, Subcommand, Clone)]
pub enum ElementCommands {
    #[clap(alias = "atn")]
    AtmNum(Atmn)
}


#[derive(Args, Debug, Clone)]

pub struct Atmn {
    pub atm_n: String
}
pub struct ElementSymbol {
    pub sym: String
}

