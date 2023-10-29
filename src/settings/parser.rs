extern crate reqwest;

use reqwest::Url;

// Allow dead code because compiler will raise a warning otherwise,
// when building modules that don't use this function
#[allow(dead_code)]
pub fn parse_url(request_url: &str) -> Result<Vec<String>, String> {
    if let Ok(url) = Url::parse(request_url) {
        let scheme = url.scheme().to_string();
        let host = url.host_str().unwrap().to_string();
        let domain = url.domain().unwrap().to_string();
        let path = url.path().to_string();
        Ok(vec![scheme, host, domain, path])
    } else {
        let err_msg = format!("'{}' is not a valid URL", request_url);
        Err(err_msg)
    }
}
