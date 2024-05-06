use std::{convert::Infallible, path::PathBuf};

use log::{debug, info, warn};

use crate::{
    specification::command::{self, Command},
    Execute,
};

#[derive(Debug, Clone)]
pub(crate) enum IsInstalledOutput {
    InstalledAt(PathBuf),
    Installed,
    NotInstalled,
}

pub(crate) struct IsInstalled;

impl Execute for IsInstalled {
    type Output = IsInstalledOutput;
    type Err = Infallible;

    fn command(executable: Option<&String>, cmd: &Command) -> Result<Self::Output, Self::Err> {
        debug!("Command is of type `Command`");
        info!("Running is_installed command: {cmd}");

        // If the command fails, that's what tells us the package
        // is not installed.
        // match command::exec(cmd) {
        match cmd.exec(None) {
            Ok(status) => {
                dbg!(&status);

                if !status.success() {
                    // Probably shouldn't be able to get here.
                    info!("Package is not installed");
                    return Ok(IsInstalledOutput::NotInstalled);
                };

                if let Some(executable) = executable {
                    match command::which(executable) {
                        Some(path) => {
                            info!("Package already installed at `{}`", path.display());
                            Ok(IsInstalledOutput::InstalledAt(path))
                        }
                        None => {
                            info!("Package already installed");
                            Ok(IsInstalledOutput::Installed)
                        }
                    }
                } else {
                    info!("Package already installed");
                    Ok(IsInstalledOutput::Installed)
                }
            }
            Err(e) => {
                warn!("Package already installed. {}", e);
                Ok(IsInstalledOutput::NotInstalled)
            }
        }
    }
}

pub(crate) struct IsInstalledDryRun;

impl Execute for IsInstalledDryRun {
    type Output = ();
    type Err = Infallible;

    fn command(_executable: Option<&String>, cmd: &Command) -> Result<Self::Output, Self::Err> {
        debug!("Command is of type `Command`");
        info!("Running is_installed command: {cmd}");
        Ok(())
    }
}
