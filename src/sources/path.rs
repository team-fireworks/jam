use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct PathSource {
    path: PathBuf,
}

impl PathSource {
    pub async fn fetch(&self) {}
}
