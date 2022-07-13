use anyhow::Result;
use tera::{Context, Tera};

pub fn new_version_available(
    templete: &str,
    app_name: &str,
    new_version: &semver::Version,
    current_version: &semver::Version,
    download_link: Option<String>,
) -> Result<String> {
    let mut tera = Tera::default();
    let mut ctx = Context::new();
    ctx.insert("app_name", app_name);
    ctx.insert("new_version", new_version.to_string().as_str());
    ctx.insert("current_version", current_version.to_string().as_str());
    if let Some(download_link) = download_link {
        ctx.insert("download_link", &download_link);
    }

    Ok(tera.render_str(templete, &ctx)?)
}

#[cfg(test)]
mod test_template {
    use super::*;
    use insta::assert_debug_snapshot;

    #[test]
    fn can_render() {
        let template  = "app_name:: {{ app_name }} version: {{ new_version }} current version: {{ current_version }} download_link: {{ download_link }}";
        assert_debug_snapshot!(new_version_available(
            template,
            "app-name-template",
            &semver::Version::parse("1.0.0").unwrap(),
            &semver::Version::parse("0.1.1").unwrap(),
            Some("https://foo.bar".to_string())
        ));
    }

    #[test]
    fn can_render_error() {
        let template = "{{ not_exists }}";
        assert_debug_snapshot!(new_version_available(
            template,
            "app-name-template",
            &semver::Version::parse("1.0.0").unwrap(),
            &semver::Version::parse("0.1.1").unwrap(),
            Some("https://foo.bar".to_string())
        ));
    }
}
