//! Extract PEP 621 configuration settings from a pyproject.toml.

use pep440_rs::VersionSpecifiers;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub(crate) struct Project {
    #[serde(alias = "requires-python", alias = "requires_python")]
    pub(crate) requires_python: Option<VersionSpecifiers>,
}
