use anyhow::Result;
use upversion::vendors::{Api, DeserializeResponse};
use upversion::CheckVersion;

fn main() -> Result<()> {
    // server json response: { "custom_version": "", "custom_release_downloads": [] }
    let deserialize_response = DeserializeResponse {
        version: "custom_version".to_string(),
        download_url: "custom_release_downloads".to_string(),
    };

    let api = Box::new(Api::custom(
        "http://127.0.0.1:3000",
        Some(deserialize_response),
    ));

    let version_context = CheckVersion::new("app-name", api, 2)?;

    // run command execute upversion check in the background and finish immediately.
    version_context.run("0.0.1")?;

    // sleep here simulator your program
    std::thread::sleep(std::time::Duration::from_secs(3));

    // at the end of your program, you can call printstd to print to the STDOUT a alert information for a new version which released
    version_context.printstd();
    Ok(())
}
