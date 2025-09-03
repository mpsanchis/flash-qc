mod commands;

use clap::Parser;
use commands::Commands;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}

impl Cli {
    pub fn new() -> Self {
        Cli::parse()
    }

    pub fn run(self) -> Result<(), String> {
        match self.command {
            Commands::Bootstrap(cmd) => cmd.run(),
        }
    }
}
