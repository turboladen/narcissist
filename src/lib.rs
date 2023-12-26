pub mod cli;
pub(crate) mod command;
pub(crate) mod command_set;
pub(crate) mod package;
pub mod package_file;
pub mod package_installer;

pub use self::{cli::Cli, package_file::PackageFile};

use self::command::Command;

pub(crate) trait Execute {
    type Output;
    type Err;

    fn command(executable: Option<&String>, shell_cmd: &Command)
        -> Result<Self::Output, Self::Err>;
}
