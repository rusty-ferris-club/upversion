use anyhow::Result;

pub trait Vendor {
    fn get(&self) -> Result<Release>;
}

#[derive(Clone, Debug)]
pub struct Release {
    pub version: String,
    pub downloads_releases: Vec<String>,
}
