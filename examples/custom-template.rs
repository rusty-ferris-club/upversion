use anyhow::Result;
use upversion::vendors::GitHubVendor;
use upversion::CheckVersion;

const CUSTOM_TEMPLATE: &str = r#"==> [CUSTOM_TEMPLATE]:: 🙆‍♂️ Newer {{ app_name }} version available: {{ new_version }} (currently running: {{ current_version }}) {% if download_link %}| Link: {{ download_link }} {% endif %}"#;

fn main() -> Result<()> {
    let github = Box::new(GitHubVendor::new("owner", "repo"));
    let version_context = CheckVersion::new("app-name", github, 2)?;

    // run command execute upversion check in the background and finish immediately.
    version_context.run("0.0.1")?;

    // sleep here simulator your program
    std::thread::sleep(std::time::Duration::from_secs(3));

    // at the end of your program, you can call printstd_with_template to print to the STDOUT a alert information for a new version which released
    version_context.printstd_with_template(CUSTOM_TEMPLATE);
    Ok(())
}
