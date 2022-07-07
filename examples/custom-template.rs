use upversion::vendors::GitHubVendor;
use upversion::VersionContext;

const CUSTOM_TEMPLATE: &str = r#"==> [CUSTOM_TEMPLATE]:: 🙆‍♂️ Newer {{ app_name }} version available: {{ new_version }} (currently running: {{ current_version }}) {% if download_link %}| Link: {{ download_link }} {% endif %}"#;
fn main() {
    let github = Box::new(GitHubVendor::new("owner", "repo"));
    let version_context = VersionContext::new("app-name", github).unwrap();
    version_context.run("0.0.1");

    std::thread::sleep(time::Duration::from_secs(5));
    version_context.printstd_with_template(CUSTOM_TEMPLATE)
}
