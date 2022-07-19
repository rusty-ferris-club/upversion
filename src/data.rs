use anyhow::Result;
use curl::easy::Easy;
use serde::{Deserialize, Serialize};
use std::sync::MutexGuard;

pub trait Vendor {
    fn get(&self, client: MutexGuard<Easy>) -> Result<Release>;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Release {
    pub version: String,
    pub downloads_releases: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct NewerReleaseVersion {
    pub current_version: semver::Version,
    pub new_version: semver::Version,
    pub release_url: Option<String>,
}
