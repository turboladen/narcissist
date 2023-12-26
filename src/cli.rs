pub(crate) mod install;

use std::borrow::Cow;

use clap::Parser;

use crate::package_installer::PackageInstaller;

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
pub fn parse() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.subcommand {
        Subcommand::Install(cmd) => {
            let using = cmd.using().trim().to_string();

            let installer = PackageInstaller::try_new(using, Cow::Borrowed(cmd.file()))?;
            installer.install(cli.dry_run)?;
        }
    }

    Ok(())
}
