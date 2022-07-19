use anyhow::Result;
use upversion::vendors::GitHubVendor;
use upversion::CheckVersion;

fn main() -> Result<()> {
    let github = Box::new(GitHubVendor::new("owner", "repo"));
    let version_context = CheckVersion::new("app-name", github, 2)?;
    version_context.run("0.0.1")?;

    std::thread::sleep(std::time::Duration::from_secs(3));
    version_context.printstd();
    Ok(())
}
