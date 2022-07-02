use upversion::vendors::GitHubVendor;
use upversion::VersionContext;

fn main() {
    let github = Box::new(GitHubVendor::new("owner", "repo"));
    let version_context = VersionContext::new("app-name", github);
    let version_template = version_context.run("0.0.1");

    if let Some(new_version) = version_template {
        println!("{}", new_version);
    };
}
