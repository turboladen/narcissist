mod install;
mod is_installed;

use std::{borrow::Cow, path::PathBuf};

use anyhow::bail;
use log::{debug, info};

use crate::{
    runner::is_installed::*,
    specification::{command_set::IsInstalled, Command},
    Specification,
};

pub(crate) trait Execute {
    type Output;
    type Err;

    fn command(executable: Option<&String>, shell_cmd: &Command)
        -> Result<Self::Output, Self::Err>;
}

trait Action {
    type RunOutput;
    type DryRunOutput;

    fn run(&self) -> anyhow::Result<Self::RunOutput>;
    fn dry_run(&self) -> anyhow::Result<Self::DryRunOutput>;
}

pub(crate) struct Runner<'a> {
    specification: Cow<'a, Specification>,
    requested_command_set: &'a str,
}

impl<'a> Runner<'a> {
    pub(crate) fn try_new(
        requested_command_set: &'a str,
        package_path: Cow<'a, PathBuf>,
    ) -> anyhow::Result<Self> {
        debug!(
            "Opening potential package file: '{}'",
            package_path.display()
        );

        let specification = Specification::open(&*package_path)?;
        specification.package().log();

        if !specification
            .command_sets()
            .keys()
            .any(|key| key == requested_command_set)
        {
            bail!(
                "Requested command-sets.{} not found in package file: `{}`. Found labels: {:#?}",
                requested_command_set,
                package_path.display(),
                specification
                    .command_sets()
                    .keys()
                    .cloned()
                    .collect::<String>()
            );
        }

        Ok(Self {
            requested_command_set,
            specification: Cow::Owned(specification),
        })
    }

    pub fn install(&self, is_dry_run: bool) -> anyhow::Result<()> {
        let command_set = self
            .specification
            .command_sets()
            .get(self.requested_command_set)
            .expect("This was validated to exist earlier; needs to be refactored");

        debug!("Using command set: {:#?}", &command_set);

        let is_installed_action = Self::new_installed_action(
            self.specification.package().executable(),
            command_set.is_installed().map(|i| i.command()),
        );

        if is_dry_run {
            info!("[dry-run] Install...");
            // TODO: Use the Target here, if given.
            is_installed_action.dry_run()
        } else {
            match is_installed_action.run() {
                Ok(is_installed_output) => {
                    match is_installed_output {
                        IsInstalledOutput::InstalledAt(path) => {
                            info!("Package already installed at {}", path.display());
                        }
                        IsInstalledOutput::Installed => {
                            info!("Package already installed");
                        }
                        IsInstalledOutput::NotInstalled => {
                            info!("Package not installed");
                            // command_set.run_install(self.specification.package().executable())?;
                            todo!()
                        }
                    }
                    Ok(())
                }
                Err(e) => {
                    debug!("[is-installed] did not complete: {e:?}.");
                    bail!(e)
                }
            }
        }
    }

    fn new_installed_action<'b>(
        maybe_executable: Option<&'b str>,
        maybe_command_set: Option<&'b Command>,
    ) -> Box<dyn Action<RunOutput = IsInstalledOutput, DryRunOutput = ()> + 'b> {
        match (maybe_executable, maybe_command_set) {
            (None, None) => todo!(),
            (None, Some(command)) => Box::new(ViaCommand::new(command)),
            (Some(executable), None) => Box::new(ViaExecutable::new(executable)),
            (Some(executable), Some(command)) => {
                Box::new(ViaExecutableThenCommand::new(executable, command))
            }
        }
    }

    fn install_action<'b>(
        maybe_executable: Option<&'b str>,
        maybe_command_set: Option<&'b Command>,
    ) -> Box<dyn Action<RunOutput = (), DryRunOutput = ()> + 'b> {
        match (maybe_executable, maybe_command_set) {
            (None, None) => todo!(),
            (None, Some(command)) => Box::new(ViaCommand::new(command)),
            (Some(executable), None) => Box::new(ViaExecutable::new(executable)),
            (Some(executable), Some(command)) => {
                Box::new(ViaExecutableThenCommand::new(executable, command))
            }
        }
    }
}
