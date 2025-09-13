use clap::builder::styling::{Styles, AnsiColor, Effects, Style};
use clap::{Parser, Subcommand};

const HEADER: Style = AnsiColor::Green.on_default().effects(Effects::BOLD);
const USAGE: Style = AnsiColor::Green.on_default().effects(Effects::BOLD);
const LITERAL: Style = AnsiColor::Cyan.on_default().effects(Effects::BOLD);
const PLACEHOLDER: Style = AnsiColor::Cyan.on_default();
const ERROR: Style = AnsiColor::Red.on_default().effects(Effects::BOLD);
const VALID: Style = AnsiColor::Cyan.on_default().effects(Effects::BOLD);
const INVALID: Style = AnsiColor::Yellow.on_default().effects(Effects::BOLD);
const CARGO_STYLING: Styles = Styles::styled()
    .header(HEADER)
    .usage(USAGE)
    .literal(LITERAL)
    .placeholder(PLACEHOLDER)
    .error(ERROR)
    .valid(VALID)
    .invalid(INVALID);

#[derive(Parser)]
#[clap(styles = CARGO_STYLING)]
#[command(name = "mcc")]
#[command(about = "A Meson-simplifying wrapper", long_about = None)]
#[command(disable_help_subcommand = true)] // removes `help` subcommand
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run project
    Run,
    /// Compile project
    Build,
    /// Run tests
    Test,
} 