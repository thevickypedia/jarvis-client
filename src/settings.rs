/*
Module to get environment variables in a case insensitive way
*/

use std::env;
use std::process::exit;

mod parser;

pub fn getenv(var_name: &str) -> String {
    // Convert the variable name to lowercase for case-insensitive comparison
    let var_name_lowercase = var_name.to_lowercase();
    // Check for the lowercase environment variable
    if let Ok(value) = env::var(var_name_lowercase) {
        return value.to_string();
    } else {
        // If the lowercase variable is not set, check the uppercase version
        if let Ok(value) = env::var(&var_name.to_uppercase()) {
            return value.to_string();
        }
    }
    return "".to_string();
}

// Allow dead code because compiler will raise a warning otherwise,
// when building modules that don't use this function
#[allow(dead_code)]
pub fn jarvis() -> String {
    let jarvis_endpoint = getenv("jarvis");
    // No need to check for None (!jarvis_endpoint) since 'get' always returns an empty value
    if jarvis_endpoint.is_empty() {
        eprintln!("ERROR\n\tJarvis endpoint is null");
        exit(1);
    } else {
        match parser::parse_url(&jarvis_endpoint) {
            Ok(parsed) => {
                let path = parsed.get(3).unwrap().as_str();
                if path != "/" {
                    println!("WARNING\n\tpath ['{}'] should never be specified, overriding..", path);
                    return format!("{}://{}",
                                   parsed.get(0).unwrap().as_str(),
                                   parsed.get(1).unwrap().as_str());
                }
                println!("URL OK: {}", jarvis_endpoint);
                return jarvis_endpoint;
            }
            Err(error) => {
                println!("Error: {}", error); // Handle error
                exit(1)
            }
        }
    }
}

// Allow dead code because compiler will raise a warning otherwise,
// when building modules that don't use this function
#[allow(dead_code)]
pub fn is_integer(input: &str) -> bool {
    match input.parse::<i32>() {
        Ok(_) => true,  // Successfully parsed as an integer
        Err(_) => false, // Failed to parse as an integer
    }
}
