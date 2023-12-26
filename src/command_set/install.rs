use std::convert::Infallible;

use anyhow::bail;
use log::{debug, info};

use crate::{command::Command, Execute};

pub(crate) struct Install;

impl Execute for Install {
    type Output = ();
    type Err = anyhow::Error;

    fn command(_executable: Option<&String>, cmd: &Command) -> anyhow::Result<Self::Output> {
        info!("Running install command in current shell: {}", &cmd);
        let status = cmd.exec(None)?;

        if status.success() {
            debug!("Install returned 0");
            Ok(())
        } else {
            bail!("Command failed: `{}`", &cmd)
        }
    }
}

pub(crate) struct InstallDryRun;

impl Execute for InstallDryRun {
    type Output = ();
    type Err = Infallible;

    fn command(_executable: Option<&String>, cmd: &Command) -> Result<Self::Output, Infallible> {
        info!("Running install command in current shell: {}", &cmd);
        Ok(())
    }
}
