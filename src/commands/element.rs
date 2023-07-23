use clap::{Parser, Args, Subcommand};

#[derive(Debug, Parser)]
pub struct ElementCommands {
    #[clap(subcommand)]
    pub element_args: ElementArgs
}

#[derive(Debug, Subcommand, Clone)]
pub enum ElementArgs {
    #[clap(alias = "atn")]
    AtmNum {
        atmn: String
    },
    #[clap(alias = "sym")]
    Symbol {
        sym: String
    }
}


#[derive(Args, Debug, Clone)]

pub struct Atmn {
    pub atm_n: String
}
#[derive(Args, Debug, Clone)]
pub struct ElementSymbol {
    pub sym: String
}

