use anyhow::Result;
use upversion::vendors::Api;
use upversion::CheckVersion;

fn main() -> Result<()> {
    let api = Box::new(Api::new("http://127.0.0.1:3000"));
    let version_context = CheckVersion::new("app-name", api, 2)?;
    version_context.run("0.0.1")?;

    std::thread::sleep(std::time::Duration::from_secs(3));
    version_context.printstd();
    Ok(())
}
