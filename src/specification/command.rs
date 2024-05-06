pub(crate) mod command_map;

use std::{
    ffi::OsStr,
    fmt,
    path::PathBuf,
    process::{Command as StdCommand, ExitStatus, Stdio},
};

use anyhow::bail;
use serde::Deserialize;

use self::command_map::CommandMap;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub(crate) enum Command {
    String(String),
    Array(Vec<String>),
    Map(CommandMap),
}

impl Command {
    /// Run a command, where you don't care about `stdout`, only the exit status.
    ///
    pub(crate) fn exec(
        &self,
        input_child: Option<std::process::Child>,
    ) -> anyhow::Result<ExitStatus> {
        let (mut cmd, args) = match self {
            Command::String(command_str) => {
                if command_str.contains('>') {
                    bail!("Command string cannot contain redirection; use a map instead")
                } else {
                    build_single(command_str)?
                }
            }
            Command::Array(command_vec) => {
                if command_vec.iter().any(|e| e.contains('>')) {
                    bail!("Command array cannot contain redirection; use a map instead")
                } else {
                    build_single_from_vec(command_vec)?
                }
            }
            Command::Map(command_map) => return command_map.exec(None),
        };

        match input_child {
            Some(ic) => Ok(cmd
                .args(args)
                .stdin(Stdio::from(ic.stdout.expect("FIXME")))
                .status()?),
            None => Ok(cmd.args(args).status()?),
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::String(s) => s.fmt(f),
            Command::Array(strings) => strings.join(" ").fmt(f),
            Command::Map(map) => {
                if map.args().is_empty() {
                    map.program().fmt(f)
                } else {
                    write!(f, "{} ", map.program())?;
                    map.args().join(" ").fmt(f)
                }
            }
        }
    }
}

/// `which {executable}`. If that returns a path, return it as a `PathBuf`.
///
pub(crate) fn which(executable: &str) -> Option<PathBuf> {
    StdCommand::new("which")
        .arg(executable)
        .output()
        .ok()
        .map(|output| {
            let output_string = std::str::from_utf8(&output.stdout).unwrap();
            PathBuf::from(output_string.trim())
        })
}

pub(crate) fn build_single(command_str: &str) -> anyhow::Result<(StdCommand, Vec<String>)> {
    let chunks: Vec<&str> = command_str.trim().split(' ').collect();

    build_single_from_vec(&chunks)
}

pub(crate) fn build_single_from_vec<T>(
    command_vec: &[T],
) -> anyhow::Result<(StdCommand, Vec<String>)>
where
    T: AsRef<OsStr> + AsRef<str> + std::fmt::Display,
{
    match command_vec.len() {
        0 => bail!(
            "Bad command given: {}",
            command_vec
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        ),
        1 => Ok((StdCommand::new(&command_vec[0]), vec![])),
        _ => {
            let prog = &command_vec[0];

            Ok((
                StdCommand::new(prog),
                command_vec[1..].iter().map(ToString::to_string).collect(),
            ))
        }
    }
}
