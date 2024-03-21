use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser, Clone)]
pub struct Metals {
    #[clap(subcommand)]
    pub metals_commands: MetalsCommands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum MetalsCommands {
    #[clap(alias = "num")]
    Number,
}
