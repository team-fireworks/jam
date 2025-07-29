use std::{collections::HashMap, fs};

use anyhow::Context;
use clap::{
    Parser,
    builder::{Styles, styling::AnsiColor},
};
use clap_verbosity_flag::Verbosity;
use console::style;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use itertools::Itertools;
use springroll::{Config, SpritesheetSpecifier, spritegen};
use tokio::task::JoinSet;

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default().underline())
    .usage(AnsiColor::Green.on_default().underline())
    .literal(AnsiColor::Cyan.on_default().bold())
    .placeholder(AnsiColor::Magenta.on_default());

const CONFIG_FILE_NAME: &str = "Springroll.toml";

fn create_progress_style() -> ProgressStyle {
    ProgressStyle::with_template(format!(
        "{{prefix:18}}{{msg:32.green.dim}}{{bar:32.magenta/.white.dim}} {{pos:.green}}{}{{len:.green}} {{elapsed:.green.dim}}",
        style("/").green()
    ).as_str()).unwrap().progress_chars("━╸━")
}

fn create_finished_progress_style() -> ProgressStyle {
    ProgressStyle::with_template(format!(
        "{{prefix:18}}{{msg:32.green.dim}}{{bar:32.green}} {{pos:.green}}{}{{len:.green}} {{elapsed:.green.dim}}",
        style("/").green()
    ).as_str()).unwrap().progress_chars("━━━")
}

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
    let bars = MultiProgress::new();

    println!("{}", style("Generating spritesheets...").dim());

    for (key, spec) in specs {
        let reqwest = reqwest.clone();

        let progress = bars.add(
            ProgressBar::new((spec.sprites.len() * 2 + spec.outputs.len()) as u64)
                .with_style(create_progress_style())
                .with_prefix(key.clone()),
        );

        progress.tick();

        outputs.spawn(async move {
            let spritesheet = spritegen(&spec, reqwest, Some(&progress))
                .await
                .context("failed to generate spritesheets")?;

            for (index, output) in spec.outputs.iter().enumerate() {
                progress.set_message(format!(
                    "Outputting #{} ({})...",
                    index + 1,
                    output.output_type()
                ));
                progress.inc(1);

                output
                    .output(&key, &spritesheet)
                    .await
                    .context("failed to output")?;
            }

            progress.set_style(create_finished_progress_style());
            progress.finish_with_message("Finished!");
            anyhow::Ok(())
        });
    }

    for result in outputs.join_all().await {
        result.unwrap();
    }

    Ok(())
}
