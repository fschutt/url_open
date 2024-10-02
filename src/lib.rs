//! # `url_open`
//!
//! A simple Rust crate to open URLs in the default web browser.
//!
//! It uses [`ShellExecuteW()`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shellexecutew) with the `"open"` verb on Windows, an `open` subprocess on macOS, and an `xdg-open` subprocess on Linux.
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
//!         .open()
//!         .expect("should be able to open URL in web browser");
//! }
//! ```
//!
//! Its public API depends on the [`url` crate](https://crates.io/crates/url).

use std::error::Error;

use url::Url;

#[cfg(target_os = "windows")]
pub fn open(url: &Url) -> Result<(), Box<dyn Error>> {
    use std::ptr;
    use windows::{
        core::{h, HSTRING, PCWSTR},
        Win32::{
            Foundation::HWND,
            UI::{Shell::ShellExecuteW, WindowsAndMessaging::SW_SHOW},
        },
    };

    let hinst = unsafe {
        ShellExecuteW(
            HWND(ptr::null_mut()),
            h!("open"),
            &HSTRING::from(url.as_str()),
            PCWSTR::null(),
            PCWSTR::null(),
            SW_SHOW,
        )
    };

    (hinst.0 > 32 as _)
        .then_some(())
        .ok_or_else(|| windows::core::Error::from_win32().into())
}

#[cfg(target_os = "macos")]
pub fn open(url: &Url) -> Result<(), Box<dyn Error>> {
    std::process::Command::new("open")
        .arg(url.as_str())
        .output()?;
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn open(url: &Url) -> Result<(), Box<dyn Error>> {
    std::process::Command::new("xdg-open")
        .arg(url.as_str())
        .output()?;
    Ok(())
}

/// Convenience method to open URLs.
pub trait UrlOpen {
    fn open(&self) -> Result<(), Box<dyn Error>>;
}

impl UrlOpen for Url {
    fn open(&self) -> Result<(), Box<dyn Error>> {
        open(self)
    }
}
