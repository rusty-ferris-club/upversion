use crate::data;
use anyhow::anyhow;
use anyhow::Result;
use curl::easy::{Easy, List};
use serde::{Deserialize, Serialize};
use std::sync::MutexGuard;

/// Default GitHub base URL
const DEFAULT_GITHUB_URL: &str = "https://api.github.com";

#[derive(Deserialize, Serialize, Debug)]
struct ReleasesResponse {
    #[serde(rename = "tag_name")]
    tag_name: String,
    #[serde(rename = "assets")]
    assets: Vec<ReleaseAssetResponse>,
}

#[derive(Deserialize, Serialize, Debug)]
struct ReleaseAssetResponse {
    #[serde(rename = "browser_download_url")]
    browser_download_url: String,
}

/// GitHub vendor
///
/// Check if there is a new version from releases page
///
/// ## Usage Example
/// ```
/// use anyhow::Result;
/// use upversion::vendors::GitHubVendor;
/// use upversion::CheckVersion;
///
/// fn main() -> Result<()> {
///     let github = Box::new(GitHubVendor::new("owner", "repo"));
///     let timeout = 2; // in seconds
///     let version_context = CheckVersion::new("app-name", github, timeout)?;
///
///     // run command execute upversion check in the background and finish immediately.
///     version_context.run("0.0.1")?;
///
///     // sleep here simulator your program
///     std::thread::sleep(std::time::Duration::from_secs(3));
///
///     // at the end of your program, you can call printstd to print to the STDOUT a alert information for a new version which released
///     version_context.printstd();
///     Ok(())
/// }
/// ```
pub struct GitHubVendor {
    base_url: String,
    owner: String,
    repo: String,
}

impl GitHubVendor {
    /// create Github instance
    ///
    /// # Arguments
    ///
    /// * `owner` - Github owner/organization
    /// * `repo` - GitHub repo name
    pub fn new(owner: &str, repo: &str) -> Self {
        Self::custom(owner, repo, None)
    }

    /// create Github instance
    ///
    /// # Arguments
    ///
    /// * `owner` - Github owner/organization
    /// * `repo` - GitHub repo name
    /// * `base_url` - GitHub custom URL
    ///
    pub fn custom(owner: &str, repo: &str, base_url: Option<String>) -> Self {
        Self {
            base_url: base_url.unwrap_or_else(|| DEFAULT_GITHUB_URL.to_string()),
            owner: owner.to_string(),
            repo: repo.to_string(),
        }
    }

    fn default_headers() -> Result<List> {
        let mut headers = List::new();
        headers.append("accept: application/vnd.github.v3+json")?;
        Ok(headers)
    }
}

impl data::Vendor for GitHubVendor {
    /// Get latest release version
    fn get(&self, client: MutexGuard<Easy>) -> Result<data::Release> {
        let mut json = Vec::new();
        let mut client = client;

        let url = format!(
            "{}/repos/{}/{}/releases?per_page=1",
            self.base_url, self.owner, self.repo
        );
        client.url(&url)?;

        client.http_headers(Self::default_headers()?)?;
        {
            let mut transfer = client.transfer();
            transfer
                .write_function(|data| {
                    json.extend_from_slice(data);
                    Ok(data.len())
                })
                .unwrap();
            transfer.perform()?;
        }

        let response: Vec<ReleasesResponse> = serde_json::from_slice(&json)?;

        if response.is_empty() {
            return Err(anyhow!("releases not found"));
        }

        // github request limited to 1 item response (see request quey parameter).
        let release_details = response.first().unwrap();

        let download_releases = &release_details
            .assets
            .iter()
            .map(|asset| asset.browser_download_url.to_string())
            .collect::<Vec<_>>();

        Ok(data::Release {
            version: release_details.tag_name.to_string(),
            downloads_releases: download_releases.clone(),
        })
    }
}

#[cfg(test)]
mod vendor_github_github {
    use crate::data::Vendor;

    use super::{Easy, GitHubVendor};
    use insta::assert_debug_snapshot;
    use std::sync::{Arc, Mutex};

    #[test]
    fn can_get_release_details() {
        let url = &mockito::server_url();

        let github = GitHubVendor::custom("owner", "repo", Some(url.to_string()));

        let data = r#"[
        {
                "tag_name": "v0.1.6",
                "assets": [
                    {
                        "browser_download_url": "https://github.com/foo"
                    },
                    {
                        "browser_download_url": "https://github.com/bar"
                    }
                ]
            }
        ]
        "#;

        let _m = mockito::mock("GET", "/repos/owner/repo/releases?per_page=1")
            .match_header("accept", "application/vnd.github.v3+json")
            .with_body(data)
            .with_status(200)
            .create();

        let easy = Easy::new();
        assert_debug_snapshot!(github.get(Arc::new(Mutex::new(easy)).lock().unwrap()));
    }

    #[test]
    fn can_get_release_details_without_releases() {
        let url = &mockito::server_url();

        let github = GitHubVendor::custom("owner", "repo", Some(url.to_string()));

        let _m = mockito::mock("GET", "/repos/owner/repo/releases?per_page=1")
            .match_header("accept", "application/vnd.github.v3+json")
            .with_body("[]")
            .with_status(200)
            .create();

        let easy = Easy::new();
        assert_debug_snapshot!(github.get(Arc::new(Mutex::new(easy)).lock().unwrap()));
    }
}
