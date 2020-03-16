use crate::domains;
use crate::parse;
use crate::GetDirectives;
use serde_json::error;
use std::thread::JoinHandle;

mod lines;
mod threads;

pub fn build(directives_list: impl GetDirectives, json: &str) -> Result<String, error::Error> {
    let domains: Result<domains::Collection, error::Error> = parse::json(json);

    match domains {
        Ok(domains) => {
            let threads: Vec<JoinHandle<String>> =
                threads::build_lines(directives_list.get_directives(), domains);

            let mut directives: String = String::new();

            for thread in threads {
                directives.push_str(thread.join().unwrap().as_str());
            }

            Ok(directives.trim().to_string())
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod directives_test {
    use crate::config;
    use crate::domains;
    use serde_json::error;

    #[test]
    fn test_build_directives() {
        let json = r#"
            {
                "domains": [
                    {"domain": "example.com", "directive": ["connect-src"]},
                    {"domain": "test.com", "directive": ["connect-src", "script-src"]}
                ]
            }
        "#;

        let csp: Result<String, error::Error> = super::build(config::get_directives(), json);

        assert_eq!(
            csp.unwrap(),
            String::from("script-src test.com; connect-src example.com test.com;")
        );
    }
}
