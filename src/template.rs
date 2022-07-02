use anyhow::Result;
use tera::{Context, Tera};

pub fn new_version_available(
    templete: &str,
    app_name: &str,
    new_version: &str,
    current_version: &str,
) -> Result<String> {
    let mut tera = Tera::default();
    let mut ctx = Context::new();
    ctx.insert("app_name", app_name);
    ctx.insert("new_version", new_version);
    ctx.insert("current_version", current_version);

    Ok(tera.render_str(templete, &ctx)?)
}
