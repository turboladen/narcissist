use std::{borrow::Cow, path::PathBuf};

use anyhow::bail;
use log::{debug, info};

use crate::{builtin::is_installed::IsInstalledOutput, Specification};

pub(crate) struct PackageInstaller<'a> {
    package_file: Cow<'a, Specification>,
    requested_command_set: String,
}

impl<'a> PackageInstaller<'a> {
    pub(crate) fn try_new(
        requested_command_set: String,
        package_path: Cow<'a, PathBuf>,
    ) -> anyhow::Result<Self> {
        debug!(
            "Opening potential package file: '{}'",
            package_path.display()
        );

        let package_file = Specification::open(&*package_path)?;
        package_file.package().log();

        if !package_file
            .command_set()
            .keys()
            .any(|key| key == &requested_command_set)
        {
            bail!(
                "Requested command-set (`{}`) not found in package file: `{}`. Found labels: {:#?}",
                &requested_command_set,
                package_path.display(),
                package_file
                    .command_set()
                    .keys()
                    .cloned()
                    .collect::<String>()
            );
        }

        Ok(Self {
            requested_command_set,
            package_file: Cow::Owned(package_file),
        })
    }

    pub fn install(&self, is_dry_run: bool) -> anyhow::Result<()> {
        let command_set = self
            .package_file
            .command_set()
            .get(&self.requested_command_set)
            .expect("This was validated to exist earlier; needs to be refactored");

        debug!("Using command set: {:#?}", &command_set);

        if is_dry_run {
            info!("[dry-run] Install...");
            // command_set.dry_run_is_installed(self.package_file.package().executable());
            // command_set.dry_run_install()?;
            // TODO: Use the Target here, if given.
            Ok(())
        } else {
            match command_set.is_installed(self.package_file.package().executable()) {
                Ok(is_installed) => {
                    match is_installed {
                        IsInstalledOutput::InstalledAt(path) => {
                            info!("Package already installed at {}", path.display());
                        }
                        IsInstalledOutput::Installed => {
                            info!("Package already installed");
                        }
                        IsInstalledOutput::NotInstalled => {
                            info!("Package not installed");
                            command_set.run_install(self.package_file.package().executable())?;
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
}
