mod check_db;

use clap;
pub use check_db::CheckDB;


#[derive(clap::Subcommand, Debug)]
pub enum Commands {
  CheckDB(CheckDB),
}