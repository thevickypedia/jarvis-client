/*
Module to get environment variables in a case insensitive way
*/

use std::env;
use std::process::exit;

pub fn get(var_name: &str) -> String {
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

pub fn jarvis() -> String {
    // todo: parse URL as validation
    let jarvis_endpoint = get("jarvis");
    // No need to check for None (!jarvis_endpoint) since 'get' always returns an empty value
    if jarvis_endpoint.is_empty() {
        eprintln!("ERROR\n\tJarvis endpoint is null");
        exit(1);
    } else {
        return jarvis_endpoint;
    }
}
