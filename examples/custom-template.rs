use upversion::vendors::GitHubVendor;
use upversion::VersionContext;

const CUSTOM_TEMPLATE: &str = r#"==> [CUSTOM_TEMPLATE]:: 🙆‍♂️ Newer {{ app_name }} version available: {{ new_version }} (currently running: {{ current_version }}) {% if download_link %}| Link: {{ download_link }} {% endif %}"#;
fn main() {
    let github = Box::new(GitHubVendor::new("owner", "repo"));
    let version_context =
        VersionContext::new("app-name", github).set_template(CUSTOM_TEMPLATE.to_string());

    let version_template = version_context.run("0.0.1");

    if let Some(new_version) = version_template {
        println!("{}", new_version);
    };
}
