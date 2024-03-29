//! asdasd
use crate::data::{NewerReleaseVersion, Vendor};
use crate::template::new_version_available;
use anyhow::anyhow;
use anyhow::Result as AnyResult;
use curl::easy::Easy;
use semver::Version;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::runtime::{Builder, Runtime};
/// holds the vendor type and the base version context
pub struct CheckVersion {
    client: Arc<Mutex<Easy>>,
    runtime: Runtime,
    vendor: Arc<Mutex<Box<dyn Vendor + Send>>>,
    app_name: String,
    result: Arc<Mutex<HashMap<String, NewerReleaseVersion>>>,
}

/// Default message template when newer version is detected
static DEFAULT_TEMPLATE: &str = r#"
==> 🙆‍♂️ Newer {{ app_name }} version available: {{ new_version }} (currently running: {{ current_version }}) {% if download_link %}| Link: {{ download_link }} {% endif %}
"#;

impl CheckVersion {
    /// Create a new check version instance
    ///
    /// # Errors
    ///
    /// Will return `Err` if runtime multi thread could not be build
    pub fn new(app_name: &str, vendor: Box<dyn Vendor + Send>, timeout: u64) -> AnyResult<Self> {
        let easy = {
            let mut easy = Easy::new();
            easy.timeout(Duration::from_secs(timeout))?;
            easy.useragent(format!("User-Agent: upversion-{}", app_name).as_str())?;
            easy
        };

        Ok(Self {
            client: Arc::new(Mutex::new(easy)),
            runtime: Builder::new_multi_thread()
                .worker_threads(1)
                .enable_all()
                .build()?,
            vendor: Arc::new(Mutex::new(vendor)),
            app_name: app_name.to_string(),
            result: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    /// Run version check in the background
    ///
    /// # Errors
    ///
    /// Will return `Err` if runtime multi thread could not be build
    pub fn run(&self, version: &str) -> AnyResult<()> {
        let version = match Self::parse_version(version) {
            Ok(v) => v,
            Err(e) => {
                log::debug!("invalid version: {}. err: {:?}", version, e);
                return Err(anyhow!("unsupported version: {}. err: {:?}", version, e));
            }
        };
        let res = self.result.clone();
        let vendor = self.vendor.clone();
        let client = self.client.clone();

        self.runtime.spawn(async move {
            let mut r = match res.lock() {
                Ok(r) => r,
                Err(e) => {
                    log::debug!("cloud not lock result. err: {:?}", e);
                    return;
                }
            };

            let vendor_mutex = match vendor.lock() {
                Ok(v) => v,
                Err(e) => {
                    log::debug!("cloud not lock vendor. err:: {:?}", e);
                    return;
                }
            };

            let client = match client.lock() {
                Ok(v) => v,
                Err(e) => {
                    log::debug!("cloud not lock vendor. err:: {:?}", e);
                    return;
                }
            };

            let release = match vendor_mutex.get(client) {
                Ok(r) => r,
                Err(e) => {
                    log::debug!("could not get release details. err: {:?}", e);
                    return;
                }
            };

            let release_version = if release.version.starts_with('v') {
                release.version.trim_start_matches('v').to_string()
            } else {
                release.version
            };

            let release_version = match Self::parse_version(release_version.as_ref()) {
                Ok(v) => v,
                Err(e) => {
                    log::debug!("invalid release version: {}. err: {:?}", release_version, e);
                    return;
                }
            };

            if version >= release_version {
                log::debug!(
                    "newer version not found. current version: {:?} latest version: {:?}",
                    version,
                    release_version
                );
                return;
            }

            r.insert(
                "result".to_string(),
                NewerReleaseVersion {
                    current_version: version,
                    new_version: release_version,
                    release_url: Self::extract_release_link(&release.downloads_releases),
                },
            );
        });

        Ok(())
    }

    pub fn printstd(&self) {
        match self.render(DEFAULT_TEMPLATE) {
            Ok(r) => println!("{}", r),
            Err(r) => log::debug!("render error {:?}", r),
        };
    }

    /// create custom template
    ///
    ///
    /// ## Supported fields:
    /// - `{{ app_name }}`: Application name
    /// - `{{ new_version }}`: Newest version number
    /// - `{{ current_version }}`: Current version
    /// - `{{ download_link }}`: Link to the new release file
    pub fn printstd_with_template(&self, template: &str) {
        match self.render(template) {
            Ok(r) => println!("{}", r),
            Err(r) => log::debug!("render error {:?}", r),
        };
    }

    fn render(&self, template: &str) -> AnyResult<String> {
        let r = self.result.lock();
        let newer_release_version = match r {
            Ok(ref r) => match r.get("result") {
                Some(v) => v,
                None => {
                    log::debug!("result is empty");
                    return Err(anyhow!("result not found"));
                }
            },
            Err(e) => {
                log::debug!("lock error: {:?}", e);
                return Err(anyhow!("lock error: {:?}", e));
            }
        };

        new_version_available(
            template,
            self.app_name.as_ref(),
            &newer_release_version.new_version,
            &newer_release_version.current_version,
            newer_release_version.release_url.clone(),
        )
    }

    /// parse text version to Version struct
    fn parse_version(version: &str) -> AnyResult<Version> {
        match Version::parse(version) {
            Ok(v) => Ok(v),
            Err(e) => return Err(anyhow!("invalid version: {}. err:; {}", version, e)),
        }
    }

    fn extract_release_link(links: &[String]) -> Option<String> {
        let os = env::consts::OS;
        let arch = env::consts::ARCH;

        let find = links
            .iter()
            .filter(|link| {
                let os_names = match os {
                    "macos" => vec!["macos", "darwin"],
                    _ => vec![os],
                };

                let link_to_lower = link.to_lowercase();

                for is_name in os_names {
                    if link_to_lower.contains(is_name) {
                        return true;
                    }
                }
                false
            })
            .filter(|link| {
                let link_to_lower = link.to_lowercase();
                if link_to_lower.contains(arch) {
                    return true;
                }
                false
            })
            .collect::<Vec<_>>();

        if find.is_empty() {
            return None;
        }

        if find.len() > 1 {
            log::debug!("found then one download link: {:?}", find);
            return None;
        }

        Some(find[0].clone())
    }
}
