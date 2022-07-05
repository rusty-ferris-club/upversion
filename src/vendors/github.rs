use crate::data;
use anyhow::anyhow;
use anyhow::Result;
use reqwest::{
    blocking::Client,
    header::{HeaderMap, ACCEPT, USER_AGENT},
    Url,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Default timeout request
const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(2);
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

pub struct GitHubVendor {
    client: Client,
    base_url: String,
    owner: String,
    repo: String,
}

impl GitHubVendor {
    /// create GitHubVendor instance
    pub fn new(owner: &str, repo: &str) -> Self {
        Self::custom(owner, repo, None, None)
    }

    /// create GitHubVendor instance with timeout request override
    pub fn with_timeout(owner: &str, repo: &str, timeout: Duration) -> Self {
        Self::custom(owner, repo, Some(timeout), None)
    }

    /// create GitHubVendor instance with custom settings
    pub fn custom(
        owner: &str,
        repo: &str,
        timeout: Option<Duration>,
        base_url: Option<String>,
    ) -> Self {
        let client = reqwest::blocking::Client::builder()
            .default_headers(Self::default_headers(format!("{}-{}", owner, repo)))
            .timeout(timeout.unwrap_or(DEFAULT_REQUEST_TIMEOUT))
            .build()
            .unwrap();

        Self {
            client,
            base_url: base_url.unwrap_or_else(|| DEFAULT_GITHUB_URL.to_string()),
            owner: owner.to_string(),
            repo: repo.to_string(),
        }
    }

    fn default_headers(agent: String) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, format!("upversion-{}", agent).parse().unwrap());
        headers.insert(ACCEPT, "application/vnd.github.v3+json".parse().unwrap());
        headers
    }
}

impl data::Vendor for GitHubVendor {
    /// Get latest release version
    fn get(&self) -> Result<data::Release> {
        let url = Url::parse_with_params(
            format!(
                "{}/repos/{}/{}/releases",
                self.base_url, self.owner, self.repo
            )
            .as_ref(),
            &[("per_page", "1")],
        )
        .unwrap();

        let response = self.client.get(url).send()?;

        // parse response
        let releases_response = response.json::<Vec<ReleasesResponse>>()?;
        if releases_response.is_empty() {
            return Err(anyhow!("releases not found"));
        }

        // github request limited to 1 item response (see request quey parameter).
        let release_details = releases_response.first().unwrap();

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

    use super::*;
    use insta::assert_debug_snapshot;
    use mockito::Matcher;

    #[test]
    fn can_get_release_details() {
        let url = &mockito::server_url();

        let github = GitHubVendor::custom("owner", "repo", None, Some(url.to_string()));

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

        let _m = mockito::mock("GET", "/repos/owner/repo/releases")
            .match_header("accept", "application/vnd.github.v3+json")
            .match_query(Matcher::UrlEncoded("per_page".into(), "1".into()))
            .with_body(data)
            .with_status(200)
            .create();

        assert_debug_snapshot!(github.get());
    }

    #[test]
    fn can_get_release_details_without_releases() {
        let url = &mockito::server_url();

        let github = GitHubVendor::custom("owner", "repo", None, Some(url.to_string()));

        let _m = mockito::mock("GET", "/repos/owner/repo/releases")
            .match_header("accept", "application/vnd.github.v3+json")
            .match_query(Matcher::UrlEncoded("per_page".into(), "1".into()))
            .with_body("[]")
            .with_status(200)
            .create();

        assert_debug_snapshot!(github.get());
    }
}
