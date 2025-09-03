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

impl Cli {
  pub fn new() -> Self {
    Cli::parse()
  }

  pub fn run(self) -> Result<(), String> {
    match self.command {
      Commands::CheckDB(cmd) => cmd.run()
    }
  }
}
