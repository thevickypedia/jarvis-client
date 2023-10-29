extern crate reqwest;
extern crate serde_json;

use std::env;
use std::process::exit;

use serde_json::Value;

mod environ;

// todo: move this to environ.rs and rename it to settings.rs
fn is_integer(input: &str) -> bool {
    match input.parse::<i32>() {
        Ok(_) => true,  // Successfully parsed as an integer
        Err(_) => false, // Failed to parse as an integer
    }
}

fn main() {
    let request_url = format!("{}/get-signals", environ::jarvis());
    let arguments: Vec<String> = env::args().collect();
    // build query params
    let mut query_params = vec![];
    if let Some(ticker) = arguments.get(1) {
        query_params.push(("symbol", ticker));
    } else {
        println!("ERROR\n\tRequires at least one argument to get the stock signals");
        exit(1)
    }
    if let Some(bar_count) = arguments.get(2) {
        if is_integer(bar_count) {
            query_params.push(("bar_count", bar_count));
        } else {
            println!("'{}' is not an integer value", bar_count);
            exit(1);
        }
    }
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(request_url)
        .query(&query_params)
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
