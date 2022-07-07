use upversion::vendors::ApiVendor;
use upversion::VersionContext;

fn main() {
    let api = Box::new(ApiVendor::new("http://127.0.0.1:3000"));
    let version_context = VersionContext::new("app-name", api).unwrap();
    version_context.run("0.0.1");

    std::thread::sleep(time::Duration::from_secs(5));
    version_context.printstd()
}
