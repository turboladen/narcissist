use semver::Version;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Package {
    description: Option<String>,
    version: Version,
    dependencies: Vec<String>,
    notes: Option<String>,
    homepage: Option<Url>,
}

impl Package {
    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn dependencies(&self) -> &[String] {
        self.dependencies.as_ref()
    }

    pub fn notes(&self) -> Option<&String> {
        self.notes.as_ref()
    }

    pub fn homepage(&self) -> Option<&Url> {
        self.homepage.as_ref()
    }
}
