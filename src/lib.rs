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
    let mut directive: String = String::from("content-security-policy ");
    directive.push_str(directives::build(directives, json).as_str());
    return directive;
}

pub fn report_only(directives: impl GetDirectives, json: &str) -> String {
    let mut directive: String = String::from("content-security-policy-report-only ");
    directive.push_str(directives::build(directives, json).as_str());
    return directive;
}
