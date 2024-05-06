pub(crate) mod builtin;
pub mod cli;
pub mod package_installer;
pub mod specification;

pub(crate) use self::specification::Specification;

use crate::specification::Command;

pub(crate) trait Execute {
    type Output;
    type Err;

    fn command(executable: Option<&String>, shell_cmd: &Command)
        -> Result<Self::Output, Self::Err>;
}
