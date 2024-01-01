pub(crate) mod install;
pub(crate) mod is_installed;

use log::{debug, info};
use serde::Deserialize;

use crate::{command::Command, Execute};

use self::is_installed::IsInstalledOutput;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct CommandSet {
    install: Command,
    is_installed: Option<Command>,
}

impl CommandSet {
    /// Entry point to the `is-installed` command.
    ///
    pub(crate) fn is_installed(
        &self,
        executable: Option<&String>,
    ) -> anyhow::Result<IsInstalledOutput> {
        debug!("is-installed check starting.");

        if let Some(cmd) = self.is_installed.as_ref() {
            Ok(self::is_installed::IsInstalled::command(executable, cmd).unwrap())
        } else {
            // bail!("Must provide `command-set.is-installed`")
            Ok(IsInstalledOutput::Installed)
        }
    }

    /// This gets call for `is-installed` if the `--dry-run` flag was given.
    ///
    pub fn dry_run_is_installed(&self, executable: Option<&String>) {
        if let Some(cmd) = self.is_installed.as_ref() {
            self::is_installed::IsInstalledDryRun::command(executable, cmd).unwrap();
        } else {
            todo!("Fail here to make sure the caller specifies a thing")
        }
    }

    /// Entry point to the `install` command.
    ///
    pub fn run_install(&self, executable: Option<&String>) -> anyhow::Result<()> {
        self::install::Install::command(executable, &self.install)
    }

    /// This gets call for `install` if the `--dry-run` flag was given.
    ///
    pub fn dry_run_install(&self) -> anyhow::Result<()> {
        info!(
            "[dry-run] Running install command in current shell: {}",
            &self.install
        );

        Ok(())
    }
}
