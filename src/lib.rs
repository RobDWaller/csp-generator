mod csp;
pub mod directives;
pub mod domains;
mod parse;

use csp::generate;
use directives::GetDirectives;
use serde_json::error;

pub struct Csp {
    pub header: String,
    pub csp: String,
}

fn parse_csp_result(header: String, result: Result<String, error::Error>) -> Csp {
    match result {
        Ok(result) => Csp {
            header,
            csp: result,
        },
        Err(e) => panic!("Could not parse JSON: {}", e),
    }
}

pub fn enforce(directives: impl GetDirectives, json: &str) -> Csp {
    parse_csp_result(
        "Content-Security-Policy".to_string(),
        generate(directives, json),
    )
}

pub fn report_only(directives: impl GetDirectives, json: &str) -> Csp {
    parse_csp_result(
        "Content-Security-Policy-Report-Only".to_string(),
        generate(directives, json),
    )
}

pub fn csp_only(directives: impl GetDirectives, json: &str) -> String {
    parse_csp_result(
        "Content-Security-Policy".to_string(),
        generate(directives, json),
    )
    .csp
}

// -----
// Tests
// -----
#[cfg(test)]
mod csp_generator_test {
    #[test]
    fn test_csp_struct() {
        let csp = super::Csp {
            header: String::from("header"),
            csp: String::from("csp"),
        };

        assert_eq!(csp.header, String::from("header"));
        assert_eq!(csp.csp, String::from("csp"));
    }

    #[test]
    fn test_parse_csp_result() {
        let csp = super::parse_csp_result(String::from("CSP"), Ok(String::from("Hello World")));

        assert_eq!(csp.header, String::from("CSP"));
        assert_eq!(csp.csp, String::from("Hello World"));
    }
}
