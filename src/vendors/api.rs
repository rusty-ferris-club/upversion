use crate::data;
use anyhow::anyhow;
use anyhow::Result;
use reqwest::blocking::Client;
use serde_json::Value;
use std::time::Duration;

/// Default timeout request
const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(2);
/// Version key when deserialize the response
const DESERIALIZE_VERSION_KEY: &str = "version";
/// Release download link key when deserialize the response
const DESERIALIZE_DOWNLOAD_URL_KEY: &str = "release_downloads";

pub struct ApiVendor {
    client: Client,
    url: String,
    deserialize_response: DeserializeResponse,
}

pub struct DeserializeResponse {
    pub version: String,
    pub download_url: String,
}

impl Default for DeserializeResponse {
    fn default() -> Self {
        Self {
            version: DESERIALIZE_VERSION_KEY.to_string(),
            download_url: DESERIALIZE_DOWNLOAD_URL_KEY.to_string(),
        }
    }
}

impl ApiVendor {
    pub fn new(url: &str) -> Self {
        Self::custom(url, None, None)
    }

    pub fn custom(
        url: &str,
        deserialize_response: Option<DeserializeResponse>,
        timeout: Option<Duration>,
    ) -> Self {
        let client = reqwest::blocking::Client::builder()
            .timeout(timeout.unwrap_or(DEFAULT_REQUEST_TIMEOUT))
            .build()
            .unwrap();
        Self {
            client,
            url: url.to_string(),
            deserialize_response: deserialize_response.unwrap_or_default(),
        }
    }

    fn get_value_with_error(&self, v: &Value, key: &str) -> Result<Value> {
        match v.get(key) {
            Some(value) => Ok(value.clone()),
            _ => return Err(anyhow!("key: {} not found", key)),
        }
    }
}
impl data::Vendor for ApiVendor {
    fn get(&self) -> Result<data::Release> {
        let response = self.client.get(&self.url).send()?;

        let v: Value = serde_json::from_str(&response.text()?)?;

        let download_releases: Vec<String> = serde_json::from_value(
            self.get_value_with_error(&v, &self.deserialize_response.download_url)?,
        )?;

        Ok(data::Release {
            version: self
                .get_value_with_error(&v, &self.deserialize_response.version)?
                .as_str()
                .unwrap()
                .to_string(),
            downloads_releases: download_releases,
        })
    }
}

#[cfg(test)]
mod state_context {
    use crate::data::Vendor;

    use super::*;
    use insta::assert_debug_snapshot;
    use mockito;
    use serde_json::json;

    #[test]
    fn can_get_value_with_error() {
        let url = &mockito::server_url();

        let api = ApiVendor::new(url.as_str());

        let json = json!({
            "version": "1.0.0",
        });
        assert_debug_snapshot!(api.get_value_with_error(&json, "version"));
        assert_debug_snapshot!(api.get_value_with_error(&json, "none"));
    }

    #[test]
    fn can_get_release_details() {
        let url = &mockito::server_url();
        println!("{:?}", url);

        let data = r#"
        {
            "version": "1.0.0",
            "release_downloads": [
                "https://foo.test",
                "https://bar.test"
            ]
        }"#;

        let _m = mockito::mock("GET", "/")
            .with_body(data)
            .with_status(200)
            .create();

        let api = ApiVendor::new(url.as_str());
        assert_debug_snapshot!(api.get());
    }

    #[test]
    fn can_get_release_details_with_custom_response() {
        let url = &mockito::server_url();

        let deserialize_response = DeserializeResponse {
            version: "custom_version".to_string(),
            download_url: "custom_release_downloads".to_string(),
        };

        let data = r#"
        {
            "custom_version": "1.0.0",
            "custom_release_downloads": [
                "https://foo.test",
                "https://bar.test"
            ]
        }"#;

        let _m = mockito::mock("GET", "/")
            .with_body(data)
            .with_status(200)
            .create();

        let api = ApiVendor::custom(url.as_str(), Some(deserialize_response), None);
        assert_debug_snapshot!(api.get());
    }

    #[test]
    fn can_get_release_details_with_timeout() {
        let url = &mockito::server_url();

        let deserialize_response = DeserializeResponse {
            version: "custom_version".to_string(),
            download_url: "custom_release_downloads".to_string(),
        };

        let data = r#"
        {
            "custom_version": "1.0.0",
            "custom_release_downloads": [
                "https://foo.test",
                "https://bar.test"
            ]
        }"#;

        let _m = mockito::mock("GET", "/")
            .with_body(data)
            .with_status(200)
            .create();

        let api = ApiVendor::custom(
            url.as_str(),
            Some(deserialize_response),
            Some(Duration::from_micros(1)),
        );
        assert_debug_snapshot!(api.get());
    }
}
