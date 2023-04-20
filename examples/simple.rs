extern crate url;
extern crate url_open;

fn main() {
    url_open::open(&url::Url::parse("https://google.com").unwrap());
}
