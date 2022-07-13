use upversion::vendors::GitHubVendor;
use upversion::CheckVersion;

fn main() {
    let github = Box::new(GitHubVendor::new("owner", "repo"));
    let version_context = CheckVersion::new("app-name", github).unwrap();
    version_context.run("0.0.1");

    std::thread::sleep(std::time::Duration::from_secs(5));
    version_context.printstd()
}
