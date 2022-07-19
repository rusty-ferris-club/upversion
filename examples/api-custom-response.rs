use anyhow::Result;
use upversion::vendors::{Api, DeserializeResponse};
use upversion::CheckVersion;

fn main() -> Result<()> {
    // server json response: { custom_version: '', custom_release_downloads: [] }
    let deserialize_response = DeserializeResponse {
        version: "custom_version".to_string(),
        download_url: "custom_release_downloads".to_string(),
    };

    let api = Box::new(Api::custom(
        "http://127.0.0.1:3000",
        Some(deserialize_response),
    ));

    let version_context = CheckVersion::new("app-name", api, 2)?;
    version_context.run("0.0.1")?;

    std::thread::sleep(std::time::Duration::from_secs(3));
    version_context.printstd();
    Ok(())
}
