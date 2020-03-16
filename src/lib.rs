use serde_json::error;

#[macro_use]
extern crate serde_derive;

pub mod config;
mod directives;
mod domains;
mod parse;

pub struct Csp {
    pub header: String,
    pub csp: String,
}

pub trait GetDirectives {
    fn get_directives(&self) -> Vec<String>;
}

fn generate_csp(header: String, csp: String) -> Csp {
    Csp { header, csp }
}

fn parse_csp_result(header: String, result: Result<String, error::Error>) -> Csp {
    match result {
        Ok(result) => generate_csp(header, result),
        Err(e) => panic!("Could not parse JSON: {}", e),
    }
}

pub fn enforce(directives: impl GetDirectives, json: &str) -> Csp {
    parse_csp_result(
        String::from("Content-Security-Policy"),
        directives::build(directives, json),
    )
}

pub fn report_only(directives: impl GetDirectives, json: &str) -> Csp {
    parse_csp_result(
        String::from("Content-Security-Policy-Report-Only"),
        directives::build(directives, json),
    )
}

pub fn csp_only(directives: impl GetDirectives, json: &str) -> String {
    parse_csp_result(
        String::from("Content-Security-Policy"),
        directives::build(directives, json),
    )
    .csp
}
