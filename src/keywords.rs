extern crate reqwest;
extern crate serde_json;

mod settings;

fn main() {
    let request_url = format!("{}/keywords", settings::jarvis());
    let token = format!("Bearer {}", settings::getenv("offline_pass"));
    let client = reqwest::blocking::Client::new();
    let resp = client.get(request_url)
        .header("Authorization", token)
        .send();
    if resp.is_ok() {
        if let Ok(text) = resp.unwrap().text() {
            println!("{}", text.as_str())
        } else {
            println!("Failed to read response text");
        }
    } else if resp.is_err() {
        println!("{:?}", resp.err());
    } else {
        println!("{:?}", resp.unwrap().error_for_status());
    }
}
