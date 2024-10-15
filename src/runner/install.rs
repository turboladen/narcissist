use anyhow::bail;
use log::{debug, info};

use crate::specification::Command;

#[derive(Debug)]
pub(super) struct Install {
    command: Command,
}

impl Install {
    pub(super) fn run(&self) -> anyhow::Result<()> {
        info!("Running install command in current shell: {:?}", &self);
        let status = self.command.exec(None)?;

        if status.success() {
            debug!("Install returned 0");
            Ok(())
        } else {
            bail!("Command failed: `{}`", &self.command)
        }
    }

    pub(super) fn dry_run(&self) {
        info!(
            "[dry-run] Running install command in current shell: {}",
            &self.command
        );
    }
}
