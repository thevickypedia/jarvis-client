extern crate reqwest;
extern crate serde_json;

use std::env;
use std::process::exit;

use serde_json::Value;

mod settings;

fn main() {
    let request_url = format!("{}/secure-send", settings::jarvis());
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 2 {
        println!("ERROR\n\tOnly ONE argument (one-time usable token) is required!!");
        exit(1)
    }
    let token = arguments.last().unwrap().to_string();
    let client = reqwest::blocking::Client::new();
    let resp = client
        .post(request_url)
        .header("access-token", token)
        .send();
    if resp.is_ok() {
        if let Ok(text) = resp.unwrap().text() {
            let text_res = text.as_str();
            let response: Result<Value, serde_json::Error> = serde_json::from_str(text_res);
            match response {
                Ok(response) => {
                    if let Some(secret) = response["detail"].as_object() {
                        for (key, value) in secret.iter() {
                            println!("Secret for '{}' is '{}'", key, value);
                        }
                    } else if let Some(authed) = response["detail"].as_str() {
                        println!("{}", authed);
                    } else {
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
