//! asdasd
use crate::data::Vendor;
use crate::template::new_version_available;
use anyhow::anyhow;
use anyhow::Result;
use semver::Version;

/// holds the vendor type and the base version context
pub struct VersionContext {
    vendor: Box<dyn Vendor>,
    app_name: String,
    template: Option<String>,
}

/// Default message template when newer version is detected
static DEFAULT_TEMPLATE: &str = r#"
==> 🙆‍♂️ Newer {{ app_name }} version available: {{ new_version }} (currently running: {{ current_version }}) {% if download_link %}| Link: {{ download_link }} {% endif %}
"#;

impl VersionContext {
    /// create new version check context
    ///
    /// ```rust
    /// let vendor = Box::new(GitHubVendor::new("rusty-ferris-club", "shellclear"));
    /// // let vendor = Box::new(ApiVendor::new("http://127.0.0.1:3000"));
    /// let version_context = VersionContext::new("app-name", vendor);
    /// ```
    pub fn new(app_name: &str, vendor: Box<dyn Vendor>) -> VersionContext {
        VersionContext {
            vendor,
            app_name: app_name.to_string(),
            template: None,
        }
    }

    /// create custom template
    ///
    /// ```rust
    /// let custom_template = "==> [CUSTOM_TEMPLATE]:: 🙆‍♂️ Newer {{ app_name }} version available: {{ new_version }} (currently running: {{ current_version }}) {% if download_link %}| Link: {{ download_link }} {% endif %}";
    /// let version_context = VersionContext::new("app-name", github).set_template(custom_template.to_string());
    /// ```
    ///
    /// ## Supported fields:
    /// - `{{ app_name }}`: Application name
    /// - `{{ new_version }}`: Newest version number
    /// - `{{ current_version }}`: Current version
    /// - `{{ download_link }}`: Link to the new release file
    pub fn set_template(mut self, template: String) -> Self {
        self.template = Some(template);
        self
    }

    /// run version check.
    pub fn run(&self, version: &str) -> Option<String> {
        // make sure the the given version is parsed
        let version = match self.parse_version(version) {
            Ok(v) => v,
            Err(e) => {
                log::debug!("invalid version: {}. err: {:?}", version, e);
                return None;
            }
        };

        // get release details from client
        let release = match self.vendor.get() {
            Ok(r) => r,
            Err(e) => {
                log::debug!("could not get release details. err: {:?}", e);
                return None;
            }
        };

        // remove 'v' prefix if exists. consider doing something less naive
        let release_version = if release.version.starts_with('v') {
            release.version.trim_start_matches('v').to_string()
        } else {
            release.version
        };

        // parse release version
        let release_version = match self.parse_version(release_version.as_ref()) {
            Ok(v) => v,
            Err(e) => {
                println!("{}", release_version);
                log::debug!("invalid release version: {}. err: {:?}", release_version, e);
                return None;
            }
        };

        if version >= release_version {
            return None;
        }

        let template_str = match &self.template {
            Some(t) => t.as_ref(),
            _ => DEFAULT_TEMPLATE,
        };
        match new_version_available(
            template_str,
            self.app_name.as_ref(),
            release_version.to_string().as_str(),
            version.to_string().as_str(),
        ) {
            Ok(s) => Some(s),
            Err(e) => {
                log::debug!("template err: {:?}", e);
                None
            }
        }
    }

    /// parse text version to Version struct
    fn parse_version(&self, version: &str) -> Result<Version> {
        match Version::parse(version) {
            Ok(v) => Ok(v),
            Err(e) => return Err(anyhow!("invalid version: {}. err:; {}", version, e)),
        }
    }
}
