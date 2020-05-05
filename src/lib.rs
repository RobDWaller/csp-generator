// Generate a Content Security Policy based on simple to define JSON config.
// This library will generate a CSP and return the correct header required to
// run the CSP in either enforcement mode or report-only mode.
//
// Note: Enforcement mode will block content and requests so please test your
// CSP setup first in report-only mode.
//
// See: https://content-security-policy.com/
mod csp;
pub mod directives;
pub mod domains;
mod parse;

use csp::generate;
use directives::GetDirectives;
use serde_json::error;

// To implement a Content Security Policy you need the correct header and CSP.
pub struct Csp {
    pub header: String,
    pub csp: String,
}

// Construct the Csp struct if the CSP generation is successful.
fn parse_csp_result(header: String, result: Result<String, error::Error>) -> Csp {
    match result {
        Ok(result) => Csp {
            header,
            csp: result,
        },
        Err(e) => panic!("Could not parse JSON: {}", e),
    }
}

// Generate a CSP with the standard header which will enforce the CSP and block
// and content or requests which do not comply.
pub fn enforce(directives: impl GetDirectives, json: &str) -> Csp {
    parse_csp_result(
        "Content-Security-Policy".to_string(),
        generate(directives, json),
    )
}

// Generate a CSP with the report only header, this will not enforce the CSP but
// just report any breaches to endpoint setup via the report-uri directive.
pub fn report_only(directives: impl GetDirectives, json: &str) -> Csp {
    parse_csp_result(
        "Content-Security-Policy-Report-Only".to_string(),
        generate(directives, json),
    )
}

// Generate the CSP only, don't return the header as well.
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
