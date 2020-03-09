use serde_json::error;

#[macro_use]
extern crate serde_derive;

pub mod config;
mod directives;
mod domains;
mod parse;

pub trait GetDirectives {
    fn get_directives(&self) -> Vec<String>;
}

pub fn enforce(directives: impl GetDirectives, json: &str) -> String {
    let result: Result<String, error::Error> = directives::build(directives, json);

    match result {
        Ok(result) => {
            let mut directive: String = String::from("Content-Security-Policy ");
            directive.push_str(result.as_str());
            directive
        }
        Err(e) => panic!("Could not parse JSON: {}", e),
    }
}

pub fn report_only(directives: impl GetDirectives, json: &str) -> String {
    let result: Result<String, error::Error> = directives::build(directives, json);

    match result {
        Ok(result) => {
            let mut directive: String = String::from("Content-Security-Policy-Report-Only ");
            directive.push_str(result.as_str());
            directive
        }
        Err(e) => panic!("Could not parse JSON: {}", e),
    }
}
