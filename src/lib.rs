//! upversion provide a simple way to alert information on new release version to all users.
//!
//! `upversion` provides a simple way to alert all users about new release versions.
//! The purpose of this lib is to inform the user that working with CLI tool or lib a message when has a new release, and give the a link to the new version.
//!
//! ## How it works
//! `upversion` running as a background process will not affect your tool performance. If your tool is finished before `upversion`,
//!  you can decide if you want to wait until `upversion` or complete your process.
//!
//!
//! ## Vendors
//! - GitHub releases
//! - Custom rest api
//!
//! ## GitHub Example:
//! ```
//! use anyhow::Result;
//! use upversion::vendors::GitHubVendor;
//! use upversion::CheckVersion;
//!
//! fn main() -> Result<()> {
//!     let github = Box::new(GitHubVendor::new("owner", "repo"));
//!     let timeout = 2; // in seconds
//!     let version_context = CheckVersion::new("app-name", github, timeout)?;
//!
//!     // run command execute upversion check in the background and finish immediately.
//!     version_context.run("0.0.1")?;
//!
//!     // sleep here simulator your program
//!     std::thread::sleep(std::time::Duration::from_secs(3));
//!
//!     // at the end of your program, you can call printstd to print to the STDOUT a alert information for a new version which released
//!     version_context.printstd();
//!     Ok(())
//! }
//! ```
//!
//! ## For more example
//! Run `cargo run --example` to see all the example files
//! - `github` - Check update version from GitHub releases
//! - `api` - Check update version from custom rest API
//! - `api-custom-response` - Example of customize of deserialize response the upversion template
//! - `custom-template` - Override the default alert information and create your custom message
//!
mod context;
mod data;
mod template;
pub mod vendors;

pub use self::context::CheckVersion;
