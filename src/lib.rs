use serde_json::error;

#[macro_use]
extern crate serde_derive;

mod domains;
mod parse;
mod directives;
pub mod config;

pub trait GetDirectives {
    fn get_directives(&self) -> Vec<String>;
}

pub fn enforce(directives: impl GetDirectives, json: &str) -> String {
    let result: Result<String, error::Error> = directives::build(directives, json);
    
    match result {
        Ok(result) => {
            let mut directive: String = String::from("content-security-policy ");
            directive.push_str(result.as_str());
            directive
        },
        Err(e) => panic!("Could not parse JSON: {}", e)
    }
}

pub fn report_only(directives: impl GetDirectives, json: &str) -> String {
    let result: Result<String, error::Error> = directives::build(directives, json);
    
    match result {
        Ok(result) => {
            let mut directive: String = String::from("content-security-policy-report-only ");
            directive.push_str(result.as_str());
            directive
        },
        Err(e) => panic!("Could not parse JSON: {}", e)
    }
}
