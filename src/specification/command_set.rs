use log::{debug, info};
use serde::Deserialize;

use crate::{builtin::is_installed::IsInstalledOutput, Execute};

use super::Command;

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
            Ok(crate::builtin::is_installed::IsInstalled::command(executable, cmd).unwrap())
        } else {
            // bail!("Must provide `command-set.is-installed`")
            Ok(IsInstalledOutput::Installed)
        }
    }

    /// This gets call for `is-installed` if the `--dry-run` flag was given.
    ///
    pub fn dry_run_is_installed(&self, executable: Option<&String>) {
        if let Some(cmd) = self.is_installed.as_ref() {
            crate::builtin::is_installed::IsInstalledDryRun::command(executable, cmd).unwrap();
        } else {
            todo!("Fail here to make sure the caller specifies a thing")
        }
    }

    /// Entry point to the `install` command.
    ///
    pub fn run_install(&self, executable: Option<&String>) -> anyhow::Result<()> {
        crate::builtin::install::Install::command(executable, &self.install)
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
