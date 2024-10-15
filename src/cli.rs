pub(crate) mod install;

use std::borrow::Cow;

use clap::Parser;

use crate::runner::Runner;

use self::install::Install;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    ///
    // #[arg(short, long, action = clap::ArgAction::Count)]
    // debug: u8,

    #[arg(short, long)]
    dry_run: bool,

    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
pub enum Subcommand {
    /// Installs a package from a package file.
    ///
    Install(Install),
}

/// Main entry point into the app. This parses the CLI command & args and executes accordingly.
///
pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.subcommand {
        Subcommand::Install(cmd) => {
            let requested_command_set = cmd.using().trim();
            let runner = Runner::try_new(requested_command_set, Cow::Borrowed(cmd.file()))?;
            runner.install(cli.dry_run)?;
        }
    }

    Ok(())
}
