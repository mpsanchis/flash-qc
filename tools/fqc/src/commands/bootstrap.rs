mod db;
mod pre_commit;

use db::check_db;

#[derive(Debug, Clone, clap::ValueEnum)]
enum PreCommitTool {
    Uvx,
    Pipx,
    Skip,
}

#[derive(Debug, clap::Args)]
pub struct Bootstrap {
    #[arg(long)]
    skip_db: bool,
    #[arg(long, value_enum)]
    pre_commit: Option<PreCommitTool>,
}

impl Bootstrap {
    pub fn run(self) -> Result<(), String> {
        println!("Bootstrapping your repo...");
        if !self.skip_db {
            check_db()?;
        }
        let installer = match self.pre_commit {
            None => None,
            Some(PreCommitTool::Pipx) => Some("pipx"),
            Some(PreCommitTool::Uvx) => Some("uvx"),
            Some(PreCommitTool::Skip) => return Ok(()),
        };

        pre_commit::install(installer)
    }
}
