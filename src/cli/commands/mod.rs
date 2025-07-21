mod generate;

use anyhow::Result;

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    Generate(generate::GenerateCommand),
}

impl Commands {
    pub async fn run(self) -> Result<()> {
        match self {
            Self::Generate(generate) => generate.run().await,
        }
    }
}
