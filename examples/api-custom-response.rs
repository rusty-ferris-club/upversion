use upversion::vendors::{ApiVendor, DeserializeResponse};
use upversion::VersionContext;

fn main() {
    let deserialize_response = DeserializeResponse {
        version: "custom_version".to_string(),
        download_url: "custom_release_downloads".to_string(),
    };

    let api = Box::new(ApiVendor::custom(
        "http://127.0.0.1:3000",
        Some(deserialize_response),
        None,
    ));

    let version_context = VersionContext::new("app-name", api);
    let version_template = version_context.run("0.0.1");

    if let Some(new_version) = version_template {
        println!("{}", new_version);
    };
}