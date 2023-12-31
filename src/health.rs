extern crate reqwest;
extern crate serde_json;

use serde_json::Value;

mod settings;

fn main() {
    let request_url = format!("{}/health", settings::jarvis());
    let resp = reqwest::blocking::get(request_url);
    if resp.is_ok() {
        if let Ok(text) = resp.unwrap().text() {
            let text_res = text.as_str();
            let response: Result<Value, serde_json::Error> = serde_json::from_str(text_res);
            match response {
                Ok(response) => {
                    if let Some(status) = response["detail"].as_str() {
                        if status == "OK" {
                            println!("Jarvis is Healthy!")
                        } else {
                            println!("Jarvis is UnHealthy")
                        }
                    } else {
                        println!("'detail' key not found!");
                        println!("{}", response);
                    }
                }
                Err(error) => {
                    println!("Error parsing JSON response {}", error);
                    println!("{}", text_res);
                }
            }
        } else {
            println!("Failed to read response text");
        }
    } else if resp.is_err() {
        println!("{:?}", resp.err());
    } else {
        println!("{:?}", resp.unwrap().error_for_status());
    }
}
