use upversion::vendors::GitHubVendor;
use upversion::VersionContext;

fn main() {
    let github = Box::new(GitHubVendor::new("owner", "repo"));
    let version_context = VersionContext::new("app-name", github).unwrap();
    version_context.run("0.0.1");

    std::thread::sleep(time::Duration::from_secs(5));
    version_context.printstd()
}
