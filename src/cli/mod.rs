use clap::{Parser, builder::styling::AnsiColor};

pub mod commands;

const STYLES: clap::builder::Styles = clap::builder::Styles::styled()
    .header(AnsiColor::Red.on_default().underline())
    .usage(AnsiColor::Red.on_default().underline())
    .literal(AnsiColor::Cyan.on_default().bold())
    .placeholder(AnsiColor::Yellow.on_default());

#[derive(Parser, Debug)]
#[command(version, about = "Pack images into spritesheets and easily reference them in code.", styles = STYLES)]
pub struct Cli {
    #[command(subcommand)]
    pub command: commands::Commands,
}
