use anyhow::Result;
use upversion::vendors::GitHubVendor;
use upversion::CheckVersion;

fn main() -> Result<()> {
    let github = Box::new(GitHubVendor::new("kaplanelad", "shellfirm"));
    let version_context = CheckVersion::new("app-name", github, 2)?;

    // run command execute upversion check in the background and finish immediately.
    version_context.run("0.0.1")?;

    // sleep here simulator your program
    std::thread::sleep(std::time::Duration::from_secs(3));

    // at the end of your program, you can call printstd to print to the STDOUT a alert information for a new version which released
    version_context.printstd();
    Ok(())
}
