use serde::Deserialize;

use crate::{command_set::CommandSet, package::Package};

#[derive(Debug, Deserialize)]
// #[serde(deny_unknown_fields)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct PackageFile {
    package: Package,

    /// What OSes do I use this on?
    command_set: CommandSet,
}

impl PackageFile {
    pub fn package(&self) -> &Package {
        &self.package
    }

    pub fn command_set(&self) -> &CommandSet {
        &self.command_set
    }
}
