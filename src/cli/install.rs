use std::{borrow::Cow, path::PathBuf};

use anyhow::bail;
use clap::Args;
use log::{debug, info};

use crate::PackageFile;

pub(crate) struct PackageInstaller<'a> {
    package_file: Cow<'a, PathBuf>,
}

impl<'a> PackageInstaller<'a> {
    pub(crate) fn new(package_file: Cow<'a, PathBuf>) -> Self {
        Self { package_file }
    }

    pub fn install(&self, is_dry_run: bool) -> anyhow::Result<()> {
        debug!("Maybe a package: {}", self.package_file.display());
        let contents = std::fs::read_to_string(&*self.package_file)?;
        let package_file: PackageFile = toml::from_str(&contents)?;
        debug!("Found package: {package_file:#?}");
        debug!("OS: {}", std::env::consts::OS);

        let command_set = package_file.command_set();

        if is_dry_run {
            info!("[dry-run] Install...");
            command_set.dry_run_is_installed();
            Ok(())
        } else {
            match command_set.run_is_installed() {
                Ok(is_installed) => {
                    if is_installed {
                        info!("Package already installed.")
                    } else {
                        command_set.run_install().expect("install failed")
                    }
                    Ok(())
                }
                Err(_) => todo!(),
            }
        }
    }
}

#[derive(Args)]
pub struct Install {
    /// File where package can be found.
    ///
    file: PathBuf,
}

impl Install {
    pub fn validate_package_file(&self) -> anyhow::Result<()> {
        debug!("Validating file {}...", self.file.display());

        if self.file.exists() && self.file.is_file() {
            debug!("File exists and is a file");
        } else {
            bail!("{} is not a file", self.file.display());
        }

        Ok(())
    }

    pub fn install(&self, is_dry_run: bool) -> anyhow::Result<()> {
        PackageInstaller::new(Cow::Borrowed(&self.file)).install(is_dry_run)
    }
}
