use upversion::vendors::ApiVendor;
use upversion::VersionContext;

fn main() {
    let api = Box::new(ApiVendor::new("http://127.0.0.1:3000"));
    let version_context = VersionContext::new("app-name", api);
    let version_template = version_context.run("0.0.1");

    if let Some(new_version) = version_template {
        println!("{}", new_version);
    };
}
