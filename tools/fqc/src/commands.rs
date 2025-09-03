mod bootstrap;

use bootstrap::Bootstrap;

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    Bootstrap(Bootstrap),
}
