use url::Url;
use url_open::UrlOpen;

fn main() {
    Url::parse("https://www.example.com/")
        .expect("URL should be parsable")
        .open()
        .expect("should be able to open URL in web browser");
}
