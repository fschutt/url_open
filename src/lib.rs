//! # `url_open`
//!
//! A simple Rust crate to open URLs in the default web browser.
//!
//! It uses [`ShellExecuteA()`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shellexecutea) on Windows, an `open` subprocess on macOS, and an `xdg-open` subprocess on Linux.
//!
//! ## Usage
//!
//! ```rust
//! use url::Url;
//! use url_open::UrlOpen;
//!
//! fn main() {
//!     Url::parse("https://www.example.com/")
//!         .expect("URL should be parsable")
//!         .open();
//! }
//! ```
//!
//! Its public API depends on the [`url` crate](https://crates.io/crates/url).

use url::Url;

#[cfg(target_os = "windows")]
pub fn open(url: &Url) {
    use std::ffi::CString;
    use std::ptr;
    use winapi::um::shellapi::ShellExecuteA;

    let cs = CString::new("open").unwrap();
    let cs2 = CString::new(url.to_string().replace("\n", "%0A")).unwrap();

    unsafe {
        ShellExecuteA(
            ptr::null_mut(),
            cs.as_ptr(),
            cs2.as_ptr(),
            ptr::null(),
            ptr::null(),
            winapi::um::winuser::SW_SHOWNORMAL,
        );
    }
}

#[cfg(target_os = "macos")]
pub fn open(url: &Url) {
    let _ = std::process::Command::new("open")
        .arg(url.to_string())
        .output();
}

#[cfg(target_os = "linux")]
pub fn open(url: &Url) {
    let _ = std::process::Command::new("xdg-open")
        .arg(url.to_string())
        .output();
}

/// Convenience method to open URLs.
pub trait UrlOpen {
    fn open(&self);
}

impl UrlOpen for Url {
    fn open(&self) {
        open(self);
    }
}
