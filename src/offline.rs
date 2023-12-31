extern crate reqwest;
extern crate serde_json;

use std::env;
use std::process::exit;

use serde_json::Value;

mod settings;

fn main() {
    let request_url = format!("{}/offline-communicator", settings::jarvis());
    let offline_pass = settings::getenv("offline_pass");
    if offline_pass.is_empty() {
        println!("ERROR\n\tRequires auth token set as environment variable!!");
        exit(1)
    }
    let command = env::args().last().unwrap().to_string();
    // If no args were passed, default will be last
    if command == env::args().next().unwrap().as_str() {
        println!("ERROR\n\tRequires a command as argument");
        exit(1)
    }
    let json_data = format!(r#"{{"command": "{}"}}"#, command);
    let mut auth_header = String::new();
    auth_header.push_str("Bearer ");
    auth_header.push_str(offline_pass.as_str());
    let client = reqwest::blocking::Client::new();
    let resp = client
        .post(request_url)
        .header("Authorization", auth_header)
        .body(json_data)
        .send();
    if resp.is_ok() {
        if let Ok(text) = resp.unwrap().text() {
            let text_res = text.as_str();
            let response: Result<Value, serde_json::Error> = serde_json::from_str(text_res);
            match response {
                Ok(response) => {
                    if let Some(status) = response["detail"].as_str() {
                        println!("{}", status);
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
