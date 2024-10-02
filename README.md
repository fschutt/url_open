[<img alt="crates.io" src="https://img.shields.io/crates/v/url_open.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/url_open)

# `url_open`

A simple Rust crate to open URLs in the default web browser.

It uses [`ShellExecuteW()`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shellexecutew) with the `"open"` verb on Windows, an `open` subprocess on macOS, and an `xdg-open` subprocess on Linux.

Its public API depends on the [`url` crate](https://crates.io/crates/url).

## Usage

```rust
use url::Url;
use url_open::UrlOpen;

fn main() {
    Url::parse("https://www.example.com/")
        .expect("URL should be parsable")
        .open()
        .expect("should be able to open URL in web browser");
}
```

## Alternatives

The following other crates use different means to accomplish the same:

- https://crates.io/crates/webbrowser
- https://crates.io/crates/open
