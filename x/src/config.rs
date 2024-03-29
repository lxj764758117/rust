use crate::{utils::project_root, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    /// Package exceptions which need to be run special
    package_exceptions: HashMap<String, Package>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct Package {
    path: PathBuf,
    #[serde(default = "default_as_true")]
    pub all_features: bool,
    #[serde(default)]
    pub system: bool,
}

// Workaround for https://github.com/serde-rs/serde/issues/368
fn default_as_true() -> bool {
    true
}

impl Config {
    pub fn from_file(f: impl AsRef<Path>) -> Result<Self> {
        let contents = fs::read(f)?;
        Self::from_toml(&contents).map_err(Into::into)
    }

    pub fn from_toml(bytes: &[u8]) -> Result<Self> {
        toml::from_slice(bytes).map_err(Into::into)
    }

    pub fn from_project_root() -> Result<Self> {
        Self::from_file(project_root().join("x.toml"))
    }

    pub fn is_exception(&self, p: &str) -> bool {
        self.package_exceptions.iter().any(|(pkg, _)| pkg == p)
    }

    pub fn package_exceptions(&self) -> &HashMap<String, Package> {
        &self.package_exceptions
    }
}
