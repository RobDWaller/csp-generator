// The core module for parsing the JSON config and generating the Content
// Security Policy.
//
// ToDo: Uses Threads, this may be overkill.
use crate::directives::GetDirectives;
use crate::domains;
use crate::parse;
use serde_json::error;
use std::thread::JoinHandle;

mod line;
mod threads;

// Collect the generated directives and compile them into a CSP string.
fn directives_to_csp(directives: Vec<JoinHandle<String>>) -> String {
    let mut csp: String = String::new();

    for directive in directives {
        csp.push_str(directive.join().unwrap().as_str());
    }

    csp.trim().to_string()
}

// Parse the JSON config and generate the Content Security Policy.
pub fn generate(directives_list: impl GetDirectives, json: &str) -> Result<String, error::Error> {
    let domains: Result<domains::Collection, error::Error> = parse::json(json);

    match domains {
        Ok(domains) => {
            let threads: Vec<JoinHandle<String>> =
                threads::build_lines(directives_list.get_directives(), domains);

            Ok(directives_to_csp(threads))
        }
        Err(e) => Err(e),
    }
}

// -----
// Tests
// -----
#[cfg(test)]
mod csp_test {
    use crate::directives;
    use serde_json::error;

    #[test]
    fn test_build_directives() {
        let json = r#"
            [
                {"domain": "example.com", "directives": ["connect-src"]},
                {"domain": "test.com", "directives": ["connect-src", "script-src"]}
            ]
        "#;

        let csp: Result<String, error::Error> = super::generate(directives::directives(), json);

        assert_eq!(
            csp.unwrap(),
            String::from("script-src test.com; connect-src example.com test.com;")
        );
    }
}
