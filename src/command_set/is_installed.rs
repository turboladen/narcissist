use std::{convert::Infallible, path::PathBuf};

use log::{debug, info};

use crate::{
    command::{self, Command},
    Execute,
};

#[derive(Debug, Clone)]
pub(crate) enum IsInstalledOutput {
    InstalledAt(PathBuf),
    Installed,
    NotInstalled,
}

pub(crate) struct IsInstalled;

impl IsInstalled {
    fn find_installed(executable: Option<&String>) -> IsInstalledOutput {
        if let Some(executable) = executable {
            match command::which(executable) {
                Some(path) => {
                    debug!("output path is empty? {}", path.as_os_str().is_empty());
                    info!("Package already installed at `{}`", path.display());
                    IsInstalledOutput::InstalledAt(path)
                }
                None => {
                    info!("Package already installed");
                    IsInstalledOutput::Installed
                }
            }
        } else {
            info!("Package already installed");
            IsInstalledOutput::Installed
        }
    }
}

impl Execute for IsInstalled {
    type Output = IsInstalledOutput;
    type Err = Infallible;

    fn command(executable: Option<&String>, cmd: &Command) -> Result<Self::Output, Self::Err> {
        info!("Running is_installed command: {cmd}");

        // If the command fails, that's what tells us the package
        // is not installed.
        // match command::exec(cmd) {
        match cmd.exec(None) {
            Ok(status) => {
                debug!("command return status: {}", &status);

                if !status.success() {
                    // Probably shouldn't be able to get here.
                    info!("Package is not installed");
                    return Ok(IsInstalledOutput::NotInstalled);
                };

                Ok(Self::find_installed(executable))
            }
            Err(_) => {
                debug!("is-installed command returned non-0");
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
