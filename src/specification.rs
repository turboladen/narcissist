pub(crate) mod command;
pub(crate) mod command_set;
pub(crate) mod package;

use std::{collections::BTreeMap, path::Path, str::FromStr};

use serde::Deserialize;

pub(crate) use self::{command::Command, command_set::CommandSet, package::Package};

/// This struct represents the file format of our TOML files.
///
/// ```
/// let file_contents = r#"
/// [package]
/// executable = "eza"
/// homepage = "https://github.com/eza-community/eza"
/// description = "A modern, maintained replacement for ls"
///
/// [command-set.default]
/// install = "cargo install eza"
/// is-installed = "command -v eza"
/// "#;
///
/// let _: narcissist::Specification = std::str::FromStr::from_str(file_contents).unwrap();
/// ```
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub(crate) struct Specification {
    package: Package,

    command_set: BTreeMap<String, CommandSet>,
}

impl Specification {
    /// Reads the file at `package_path`, then parses it as TOML into a `Self`.
    ///
    pub fn open<P: AsRef<Path>>(package_path: P) -> anyhow::Result<Self> {
        let contents = std::fs::read_to_string(package_path)?;

        Ok(Self::from_str(contents.trim())?)
    }

    /// The `[package]` section of the file.
    ///
    pub fn package(&self) -> &Package {
        &self.package
    }

    /// The `[command-set]` section of the file.
    ///
    pub fn command_set(&self) -> &BTreeMap<String, CommandSet> {
        &self.command_set
    }
}

impl FromStr for Specification {
    type Err = toml::de::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
    }
}
