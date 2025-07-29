use std::{collections::HashMap, fs};

use anyhow::Context;
use clap::{
    Parser,
    builder::{Styles, styling::AnsiColor},
};
use clap_verbosity_flag::Verbosity;
use itertools::Itertools;
use springroll::{Config, SpritesheetSpecifier, spritegen};
use tokio::task::JoinSet;

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Red.on_default().underline())
    .usage(AnsiColor::Red.on_default().underline())
    .literal(AnsiColor::Cyan.on_default().bold())
    .placeholder(AnsiColor::Yellow.on_default());

const CONFIG_FILE_NAME: &str = "Springroll.toml";

#[derive(Parser, Debug)]
#[command(version, about = "Pack images into spritesheets and easily reference them in code.", styles = STYLES)]
pub struct Args {
    /// Name of the person to greet
    #[arg(long)]
    include: Option<Vec<String>>,
    #[command(flatten)]
    verbosity: Verbosity,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    env_logger::Builder::from_env("SPRINGROLL_LOG")
        .filter_level(args.verbosity.log_level_filter())
        .init();

    let config: Config = toml::from_str(
        &fs::read_to_string(CONFIG_FILE_NAME).context("failed to read config file")?,
    )
    .context("failed to parse config file")?;

    let config_spritesheets = &config.spritesheets;

    let unsorted_specs = match args.include {
        None => config_spritesheets.clone(),
        Some(include) => {
            let mut result = HashMap::new();
            for (name, spec) in config_spritesheets {
                if include.contains(name) {
                    result.insert(name.clone(), spec.clone());
                }
            }
            result
        }
    };

    let specs: HashMap<String, SpritesheetSpecifier> = unsorted_specs
        .iter()
        .sorted_by(|a, b| Ord::cmp(a.0, b.0))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    let reqwest = reqwest::Client::builder()
        .build()
        .context("failed to create reqwest client")?;

    let mut outputs = JoinSet::new();

    for (key, spec) in specs {
        let reqwest = reqwest.clone();

        outputs.spawn(async move {
            let spritesheet = spritegen(&spec, reqwest)
                .await
                .context("failed to generate spritesheets")?;

            for output in &spec.outputs {
                output
                    .output(&key, &spritesheet)
                    .await
                    .context("failed to output")?;
            }

            anyhow::Ok(())
        });
    }

    for result in outputs.join_all().await {
        result.unwrap();
    }

    Ok(())
}
