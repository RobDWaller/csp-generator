// The core module for parsing the JSON config and generating the Content
// Security Policy.
//
// ToDo: Uses Threads, this may be overkill.
mod directive;
mod domains;

use crate::directives::GetDirectives;
use crate::domains::Collection;
use crate::parse;
use serde_json::error;
use std::thread;
use std::thread::JoinHandle;

// Collect the generated directives and compile them into a CSP string.
fn directives_to_csp(directives: Vec<JoinHandle<String>>) -> String {
    let mut csp: String = String::new();

    for directive in directives {
        let directive_string = directive.join().unwrap();

        if !directive_string.is_empty() {
            csp.push_str(directive_string.as_str());
            csp.push_str(" ");
        }
    }

    csp.trim().to_string()
}

// Make each directive based on the domains collection supplied and the
// directives config.
fn make_directives(directives_config: Vec<String>, domains: Collection) -> Vec<JoinHandle<String>> {
    let mut directives: Vec<JoinHandle<String>> = vec![];

    for directive_item in directives_config {
        let domains_clone = domains.clone();

        directives.push(
            thread::spawn(move || directive::generate(directive_item, domains_clone))
        );
    }

    directives
}

// Parse the JSON config and generate the Content Security Policy.
pub fn generate(directives_config: impl GetDirectives, json: &str) -> Result<String, error::Error> {
    let domains: Result<Collection, error::Error> = parse::json(json);

    match domains {
        Ok(domains) => {
            let config = directives_config.get_directives();

            let directives: Vec<JoinHandle<String>> = make_directives(config, domains);

            Ok(directives_to_csp(directives))
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
