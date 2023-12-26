use std::path::PathBuf;

use clap::Args;

#[derive(Args)]
pub struct Install {
    /// Specify which command-set to use.
    ///
    #[arg(long, default_value_t = String::from("default"))]
    using: String,

    /// File where package can be found.
    ///
    file: PathBuf,
}

impl Install {
    pub fn using(&self) -> &str {
        self.using.as_ref()
    }

    pub fn file(&self) -> &PathBuf {
        &self.file
    }
}
