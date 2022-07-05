use anyhow::Result;
use serde_derive::{Deserialize, Serialize};

pub trait Vendor {
    fn get(&self) -> Result<Release>;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Release {
    pub version: String,
    pub downloads_releases: Vec<String>,
}
