use anyhow::Result;
use upversion::vendors::Api;
use upversion::CheckVersion;

fn main() -> Result<()> {
    // server json response: { "version": "", "release_downloads": [] }
    let api = Box::new(Api::new("http://127.0.0.1:3000"));
    let version_context = CheckVersion::new("app-name", api, 2)?;

    // run command execute upversion check in the background and finish immediately.
    version_context.run("0.0.1")?;

    // sleep here simulator your program
    std::thread::sleep(std::time::Duration::from_secs(3));

    // at the end of your program, you can call printstd to print to the STDOUT a alert information for a new version which released
    version_context.printstd();
    Ok(())
}
