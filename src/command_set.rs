use std::process::{Command as StdCommand, Stdio};

use anyhow::bail;
use log::info;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct CommandSet {
    install: Command,
    is_installed: Option<Command>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub(crate) enum Command {
    /// A `Shell` command just runs a command in a subprocess of whatever shell you're currently
    /// running.
    ///
    Shell(String),
}

impl CommandSet {
    pub fn run_is_installed(&self) -> anyhow::Result<bool> {
        if let Some(cmd) = self.is_installed.as_ref() {
            match cmd {
                Command::Shell(shell_cmd) => {
                    info!("Running is_installed command: {}", shell_cmd);

                    match StdCommand::new(shell_cmd).status() {
                        Ok(status) => {
                            if status.success() {
                                Ok(true)
                            } else {
                                Ok(false)
                            }
                        }
                        Err(_) => Ok(false),
                    }
                }
            }
        } else {
            bail!("Must provide `command-set.is-installed`")
        }
    }

    pub fn dry_run_is_installed(&self) {
        if let Some(cmd) = self.is_installed.as_ref() {
            match cmd {
                Command::Shell(shell_cmd) => {
                    info!("[dry-run] Running is_installed command: {}", shell_cmd);
                }
            }
        } else {
            todo!("Fail here to make sure the caller specifies a thing")
        }
    }

    pub fn run_install(&self) -> anyhow::Result<()> {
        match &self.install {
            Command::Shell(cmd) => {
                info!("Running install command in current shell: {}", &cmd);

                let status = StdCommand::new(&cmd).stdout(Stdio::piped()).status()?;

                if status.success() {
                    Ok(())
                } else {
                    bail!("Shell install command failed: `{}`", &cmd)
                }
            }
        }
    }
}
