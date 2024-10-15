use log::info;
use serde::Deserialize;
use url::Url;

/// A `Package` contains the metadata for a `Specification`.
///
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Package {
    executable: Option<String>,
    homepage: Option<Url>,
    description: Option<String>,
    notes: Option<String>,
}

impl Package {
    /// The name of the executable that will exist on the system after the package is installed
    /// (ex. `bat`).
    ///
    pub fn executable(&self) -> Option<&str> {
        self.executable.as_deref()
    }

    /// The home page of the package.
    ///
    pub fn homepage(&self) -> Option<&Url> {
        self.homepage.as_ref()
    }

    /// Description of the package. I like to copy this from the package's GitHub home page.
    ///
    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// These are just any notes for myself that I might want to remember for later.
    ///
    pub fn notes(&self) -> Option<&String> {
        self.notes.as_ref()
    }

    /// Helper method for logging info about the package.
    ///
    pub(crate) fn log(&self) {
        info!("Package:");
        info!("\tExecutable: `{}`", self.executable().map_or("", |s| s));
        info!("\tHomepage: {}", self.homepage().map_or("", |s| s.as_str()));
        info!(
            "\tDescription: \"{}\"",
            self.description().map_or("", |s| s.as_str())
        );
        info!("\tnotes: \"{}\"", self.notes().map_or("", |s| s.as_str()));
    }
}
