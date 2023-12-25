mod install;
mod install_all;

use clap::{Parser, Subcommand};

use self::{install::Install, install_all::InstallAll};

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
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Installs a package from a package file.
    Install(Install),
    InstallAll(InstallAll),
}

pub fn parse() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Install(cmd) => {
            cmd.validate_package_file()?;
            cmd.install(cli.dry_run)?;
        }
        Commands::InstallAll(_install_all) => {
            todo!()
        }
    }

    Ok(())
}
