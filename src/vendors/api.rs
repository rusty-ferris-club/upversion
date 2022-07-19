use crate::data;
use anyhow::anyhow;
use anyhow::Result;
use curl::easy::Easy;
use serde_json::Value;
use std::sync::MutexGuard;

// Version key when deserialize the response
const DESERIALIZE_VERSION_KEY: &str = "version";
// Release download link key when deserialize the response
const DESERIALIZE_DOWNLOAD_URL_KEY: &str = "release_downloads";

///  Rest api vendor
///
/// If you manage your program version internally, you allow to serve the new version with your custom logic via rest API, and `upversion` will query your endpoint.
///  
/// ## Usage Example
///  ```
/// use anyhow::Result;
/// use upversion::vendors::Api;
/// use upversion::CheckVersion;
///
/// fn main() -> Result<()> {
///     let api = Box::new(Api::new("http://127.0.0.1:3000"));
///     let timeout = 2; // in seconds
///     let version_context = CheckVersion::new("app-name", api, timeout)?;
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
///
///  ```
pub struct Api {
    url: String,
    deserialize_response: DeserializeResponse,
}

/// Deserialize api response to version and download url
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

impl Api {
    /// Create a new API vendor with default deserialize response.
    ///
    ///
    /// # Arguments
    ///
    /// * `url` - Endpoint URL
    ///
    /// ## Expected response:
    /// ```json
    /// {
    ///     "version": "",
    ///      "release_downloads": ""
    /// }
    /// ```
    pub fn new(url: &str) -> Self {
        Self::custom(url, None)
    }

    /// Create a new API vendor with customize deserialize response.
    ///
    /// # Arguments
    ///
    /// * `url` - Endpoint URL
    /// * `deserialize_response` - describe via `DeserializeResponse` the response keys
    ///
    /// ## Expected response:
    /// ```json
    /// {
    ///     "custom_version": "",
    ///      "custom_release_downloads": []
    /// }
    /// ```
    ///
    /// ## Implementation
    /// ```
    /// use anyhow::Result;
    /// use upversion::vendors::{Api, DeserializeResponse};
    /// use upversion::CheckVersion;
    ///
    /// fn main() -> Result<()> {
    ///     // server json response: { custom_version: '', custom_release_downloads: [] }
    ///     let deserialize_response = DeserializeResponse {
    ///         version: "custom_version".to_string(),
    ///         download_url: "custom_release_downloads".to_string(),
    ///     };
    ///
    ///     let api = Box::new(Api::custom(
    ///         "http://127.0.0.1:3000",
    ///         Some(deserialize_response),
    ///     ));
    ///
    ///     let timeout = 2; // in seconds
    ///     let version_context = CheckVersion::new("app-name", api, timeout)?;
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
    ///
    pub fn custom(url: &str, deserialize_response: Option<DeserializeResponse>) -> Self {
        Self {
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
impl data::Vendor for Api {
    fn get(&self, client: MutexGuard<Easy>) -> Result<data::Release> {
        let mut json = Vec::new();
        let mut client = client;
        client.url(&self.url)?;

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

        let response: Value = serde_json::from_slice(&json)?;
        let download_releases: Vec<String> = serde_json::from_value(
            self.get_value_with_error(&response, &self.deserialize_response.download_url)?,
        )?;

        Ok(data::Release {
            version: self
                .get_value_with_error(&response, &self.deserialize_response.version)?
                .as_str()
                .unwrap()
                .to_string(),
            downloads_releases: download_releases,
        })
    }
}

#[cfg(test)]
mod vendor_api_test {
    use crate::data::Vendor;

    use super::*;
    use insta::assert_debug_snapshot;
    use mockito;
    use serde_json::json;
    use std::sync::{Arc, Mutex};

    #[test]
    fn can_get_value_with_error() {
        let url = &mockito::server_url();

        let api = Api::new(url.as_str());

        let json = json!({
            "version": "1.0.0",
        });
        assert_debug_snapshot!(api.get_value_with_error(&json, "version"));
        assert_debug_snapshot!(api.get_value_with_error(&json, "none"));
    }

    #[test]
    fn can_get_release_details() {
        let url = &mockito::server_url();

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

        let api = Api::new(url.as_str());

        let easy = Easy::new();
        assert_debug_snapshot!(api.get(Arc::new(Mutex::new(easy)).lock().unwrap()));
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

        let api = Api::custom(url.as_str(), Some(deserialize_response));
        let easy = Easy::new();
        assert_debug_snapshot!(api.get(Arc::new(Mutex::new(easy)).lock().unwrap()));
    }
}
