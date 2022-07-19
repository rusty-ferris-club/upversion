[![Build](https://github.com/rusty-ferris-club/upversion/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/rusty-ferris-club/upversion/actions/workflows/ci.yml)
<p align="center">
<br/>
<br/>
<br/>
   <img src="logo-here" width="300"/>
<br/>
<br/>
</p>
<p align="center">
<b>:label: keep your client up to date</b>
<br/>
<b>:see_no_evil: Run in the background</b>
<br/>
<b>:hourglass: Limit background task</b>
<br/>
<hr/>
</p>

# upversion

`upversion` provides a simple way to alert all users about new release versions.
The purpose of this lib is to inform the user that working with CLI tool or lib a message when has a new release, and give a link to the new version.

## How it works
`upversion` running as a background process will not affect your tool performance. If your tool is finished before `upversion`, you can decide if you want to wait until `upversion` or complete your process.

```sh
$ ./test-tool
==> 🙆‍♂️ Newer app-name version available: 0.1.0 (currently running: 0.5.2) | Link: <dynamic-link>
```

## Usage
Add this to Cargo.toml:
```toml
[dependencies]
upversion = { version = "0.1" }
```

## Vendor
* GitHub releases
* Custom RestAPI


## Github Example:
```rs
use anyhow::Result;
use upversion::vendors::GitHubVendor;
use upversion::CheckVersion;

fn main() -> Result<()> {
    let github = Box::new(GitHubVendor::new("kaplanelad", "shellfirm"));
    let timeout = 2; // in seconds
    let version_context = CheckVersion::new("app-name", github, timeout)?;

    // run command execute upversion check in the background and finish immediately.
    version_context.run("0.0.1")?;

    // sleep here simulator your program
    std::thread::sleep(std::time::Duration::from_secs(3));

    // at the end of your program, you can call printstd to print to the STDOUT a alert information for a new version which released
    version_context.printstd();
    Ok(())
}
```

## Custom API:
If you manage your program version internally, you allow to serve the new version with your custom logic via rest API, and `upversion` will query your endpoint.
```rs
use anyhow::Result;
use upversion::vendors::Api;
use upversion::CheckVersion;

fn main() -> Result<()> {
    // server json response: { "version": "", "release_downloads": [] }
    let api = Box::new(Api::new("http://127.0.0.1:3000"));
    let timeout = 2; // in seconds
    let version_context = CheckVersion::new("app-name", api, timeout)?;

    // run command execute upversion check in the background and finish immediately.
    version_context.run("0.0.1")?;

    // sleep here simulator your program
    std::thread::sleep(std::time::Duration::from_secs(3));

    // at the end of your program, you can call printstd to print to the STDOUT a alert information for a new version which released
    version_context.printstd();
    Ok(())
}
```

### More example
You can find more example [here](./examples/), or run via cargo `cargo run --example`


## Customize Template
Customize alert message with your owned template
```rs
    const CUSTOM_TEMPLATE: &str = r#"==> [CUSTOM_TEMPLATE]:: 🙆‍♂️ Newer {{ app_name }} version available: {{ new_version }} (currently running: {{ current_version }}) {% if download_link %}| Link: {{ download_link }} {% endif %}"#;
    ... 
    version_context.printstd_with_template(CUSTOM_TEMPLATE);
```


# Thanks
To all [Contributors](https://github.com/rusty-ferris-club/upversion/graphs/contributors) - you make this happen, thanks!

# Copyright
Copyright (c) 2022 [@kaplanelad](https://github.com/kaplanelad). See [LICENSE](LICENSE.txt) for further details.


