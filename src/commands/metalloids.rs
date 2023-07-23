use clap::{Parser, Args, Subcommand};

#[derive(Debug, Parser)]
pub struct Metalloids;
#[derive(Debug, Clone, Subcommand)]
pub enum MetalsCommands {
    #[clap(alias = "num")]
    Number
}