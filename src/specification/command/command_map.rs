use std::process::{Child, Command, ExitStatus, Stdio};

use anyhow::{anyhow, bail};
use serde::Deserialize;

/// A `CommandMap` is the most detailed and explicit way to describe a command to use for installing
/// a package. It is also the only way to pipe or redirect command output to another command or file
/// descriptor.
///
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub(crate) struct CommandMap {
    program: String,
    args: Vec<String>,
    redirect: Option<Redirect>,
    pipe: Option<Box<super::Command>>,
}

impl CommandMap {
    pub(super) fn program(&self) -> &str {
        self.program.as_ref()
    }

    pub(super) fn args(&self) -> &[String] {
        self.args.as_ref()
    }

    pub(super) fn redirect(&self) -> Option<&Redirect> {
        self.redirect.as_ref()
    }

    pub(crate) fn pipe(&self) -> Option<&super::Command> {
        self.pipe.as_deref()
    }

    pub(super) fn exec(&self, input: Option<Child>) -> anyhow::Result<ExitStatus> {
        if self.redirect.is_some() && self.pipe.is_some() {
            bail!("Can't give `redirect` and `pipe` in same command-set");
        }

        self._exec(input)
    }

    fn _exec(&self, input: Option<Child>) -> anyhow::Result<ExitStatus> {
        let mut command = Command::new(&self.program);
        let command = if let Some(input_child) = input {
            command.args(&self.args).stdin(Stdio::from(
                input_child
                    .stdout
                    .ok_or_else(|| anyhow!("Input process must have stdout piped"))?,
            ))
        } else {
            command.args(&self.args)
        };

        let command = command.stdout(Stdio::piped());

        if let Some(redirect) = self.redirect() {
            self.exec_with_redirect(command, redirect)
        } else if let Some(pipe) = self.pipe() {
            let child = command.args(&self.args).spawn()?;

            pipe.exec(Some(child))
        } else {
            Ok(command.status()?)
        }
    }

    fn exec_with_redirect(
        &self,
        command: &mut Command,
        redirect: &Redirect,
    ) -> anyhow::Result<ExitStatus> {
        fn redirect_stdout_to<'a>(
            command: &'a mut Command,
            stdout_to: &str,
        ) -> anyhow::Result<&'a mut Command> {
            if stdout_to == "/dev/null" {
                Ok(command.stdout(Stdio::null()))
            } else {
                bail!("Can't redirect stdout to {stdout_to}")
            }
        }

        fn redirect_stderr_to<'a>(
            command: &'a mut Command,
            stderr_to: &str,
        ) -> anyhow::Result<&'a mut Command> {
            if stderr_to == "/dev/null" {
                Ok(command.stderr(Stdio::null()))
            } else {
                bail!("Can't redirect stderr to {stderr_to}")
            }
        }

        match (redirect.stdout_to.as_deref(), redirect.stderr_to.as_deref()) {
            (None, None) => Ok(command.args(&self.args).status()?),
            (None, Some(stderr_to)) => Ok(redirect_stderr_to(command, stderr_to)?.status()?),
            (Some(stdout_to), None) => Ok(redirect_stdout_to(command, stdout_to)?.status()?),
            (Some(stdout_to), Some(stderr_to)) => {
                let command = redirect_stdout_to(command, stdout_to)?;
                Ok(redirect_stderr_to(command, stderr_to)?.status()?)
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub(super) struct Redirect {
    stdout_to: Option<String>,
    stderr_to: Option<String>,
}
