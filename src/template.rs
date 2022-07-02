use anyhow::Result;
use tera::{Context, Tera};

pub fn new_version_available(
    templete: &str,
    app_name: &str,
    new_version: &str,
    current_version: &str,
    download_link: Option<String>,
) -> Result<String> {
    let mut tera = Tera::default();
    let mut ctx = Context::new();
    ctx.insert("app_name", app_name);
    ctx.insert("new_version", new_version);
    ctx.insert("current_version", current_version);
    if let Some(download_link) = download_link {
        ctx.insert("download_link", &download_link);
    }

    Ok(tera.render_str(templete, &ctx)?)
}
