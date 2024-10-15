//! This is the runnable command as defined in the package specification. If `executable` is given
//! (aka is defined in the spec), this will check to see if that executable is available in the
//! current environment.
//!
use std::path::PathBuf;

use log::{debug, info, warn};

use crate::specification::{command, Command};

use super::Action;

enum ExecutableStatus {
    Found(PathBuf),
    NotFound,
}

#[derive(Debug, Clone)]
pub(super) enum IsInstalledOutput {
    InstalledAt(PathBuf),
    Installed,
    NotInstalled,
}

/// This type handles checking to see if the package is installed by looking to see if it's
/// available as an executable in the current environment.
///
#[derive(Debug)]
pub(super) struct ViaExecutable<'a> {
    executable: &'a str,
}

impl<'a> ViaExecutable<'a> {
    pub(super) fn new(executable: &'a str) -> Self {
        Self { executable }
    }
}

impl<'a> Action for ViaExecutable<'a> {
    type RunOutput = IsInstalledOutput;
    type DryRunOutput = ();

    /// If the executable is found, this logs the path of that executable and returns it; otherwise
    /// the package is not installed, as far as this method can tell.
    ///
    fn run(&self) -> anyhow::Result<Self::RunOutput> {
        fn check_executable(executable: &str) -> ExecutableStatus {
            match command::which(executable) {
                Some(path) => ExecutableStatus::Found(path),
                None => {
                    info!("Executable for package not found.");
                    ExecutableStatus::NotFound
                }
            }
        }

        info!(
            "[is-installed.executable.run] Checking if `{}` is available in current environment...",
            self.executable
        );
        let output = match check_executable(self.executable) {
            ExecutableStatus::Found(path) => {
                info!("Package already installed at `{}`", path.display());
                IsInstalledOutput::InstalledAt(path)
            }
            ExecutableStatus::NotFound => {
                warn!("Package executable not found.");
                IsInstalledOutput::NotInstalled
            }
        };
        info!("[is-installed.executable.run] Done.");

        Ok(output)
    }

    fn dry_run(&self) -> anyhow::Result<Self::DryRunOutput> {
        info!(
            "[is-installed.executable.dry-run] Checking if `{}` is available in current environment...",
            self.executable
        );
        info!("[is-installed.executable.dry-run] Done (dry-run).");

        Ok(())
    }
}

/// This type handles checking to see if the package is installed by running `command`.
///
#[derive(Debug)]
pub(super) struct ViaCommand<'a> {
    command: &'a Command,
}

impl<'a> ViaCommand<'a> {
    pub(super) fn new(command: &'a Command) -> Self {
        Self { command }
    }
}

impl<'a> Action for ViaCommand<'a> {
    type RunOutput = IsInstalledOutput;
    type DryRunOutput = ();

    /// If that command succeeds, we assume the package is installed; if the command fails, we
    /// assume it's not.
    ///
    fn run(&self) -> anyhow::Result<Self::RunOutput> {
        info!(
            "[is-installed.command.run] Using command to check if package is available in current environment: {}",
            self.command.to_string()
        );

        let result = match self.command.exec(None) {
            Ok(status) => {
                dbg!(&status);

                if status.success() {
                    IsInstalledOutput::Installed
                } else {
                    // Probably shouldn't be able to get here?
                    IsInstalledOutput::NotInstalled
                }
            }
            Err(e) => {
                warn!("is-installed command failed; maybe package is not installed? {e:?}");
                IsInstalledOutput::NotInstalled
            }
        };
        info!("[is-installed.command.run] Done.");

        Ok(result)
    }

    fn dry_run(&self) -> anyhow::Result<Self::DryRunOutput> {
        info!(
            "[is-installed.command.dry-run] Running command to see if package is available in current environment: `{}`",
            self.command
        );
        info!("[is-installed.command.dry-run] Done (dry-run).");
        Ok(())
    }
}

#[derive(Debug)]
pub(super) struct ViaExecutableThenCommand<'a> {
    executable: &'a str,
    command: &'a Command,
}

impl<'a> ViaExecutableThenCommand<'a> {
    pub(super) fn new(executable: &'a str, command: &'a Command) -> Self {
        Self {
            executable,
            command,
        }
    }
}

impl<'a> Action for ViaExecutableThenCommand<'a> {
    type RunOutput = IsInstalledOutput;
    type DryRunOutput = ();

    fn run(&self) -> anyhow::Result<Self::RunOutput> {
        match ViaExecutable::new(self.executable).run()? {
            IsInstalledOutput::NotInstalled => ViaCommand::new(self.command).run(),
            t => Ok(t),
        }
    }

    fn dry_run(&self) -> anyhow::Result<Self::DryRunOutput> {
        todo!()
    }
}
