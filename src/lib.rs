//! upversion
//!
//!
//! ## Get newest version from GitHub releases
//! ```rust
//! use upversion::vendors::GitHubVendor;
//! use upversion::VersionContext;
//!
//! fn main() {
//!     let github_client = Box::new(GitHubVendor::new("owner", "repo-name"));
//!     let version_context = VersionContext::new("app-name", github_client);
//!     let version_template = version_context.run("0.0.5");
//!     if let Some(new_version) = version_template {
//!         println!("{}", new_version);
//!     };
//! }
//! ```
mod context;
mod data;
mod template;
pub mod vendors;

pub use self::context::VersionContext;
