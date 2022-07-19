//! List of supported vendors
mod api;
mod github;

pub use self::api::{Api, DeserializeResponse};
pub use self::github::GitHubVendor;
