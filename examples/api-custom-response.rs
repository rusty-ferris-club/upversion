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

    let version_context = VersionContext::new("app-name", api).unwrap();
    version_context.run("0.0.1");

    std::thread::sleep(time::Duration::from_secs(5));
    version_context.printstd()
}
