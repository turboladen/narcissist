use std::{borrow::Cow, path::PathBuf};

use anyhow::bail;
use clap::Args;
use log::{debug, info};

use crate::cli::install::PackageInstaller;

#[derive(Args)]
pub struct InstallAll {
    /// File where package can be found.
    ///
    file: PathBuf,
}

impl InstallAll {
    pub fn validate_path(&self) -> anyhow::Result<()> {
        debug!("Checking file {}...", self.file.display());

        if self.file.exists() && self.file.is_file() {
            debug!("Found a real file!");
        } else {
            bail!("{} is not a file", self.file.display());
        }

        Ok(())
    }

    pub fn install(&self, is_dry_run: bool) -> anyhow::Result<()> {
        for entry in self
            .file
            .read_dir()
            .expect("read_dir call failed")
            .flatten()
        {
            let path = entry.path();
            info!("Checking path {}", path.display());

            let installer = PackageInstaller::new(Cow::Borrowed(&path));
            installer.install(is_dry_run)?
        }
        Ok(())
    }
}
