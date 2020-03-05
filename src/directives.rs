use crate::domains;
use crate::parse;
use crate::GetDirectives;
use serde_json::error;
use std::thread;
use std::thread::JoinHandle;

fn build_line(directive: String, domains: domains::Collection) -> String {
    let mut directive_line: String = directive.to_string();
    directive_line.push_str(":");

    for domain in domains.domains {
        if domain.directive.contains(&directive.to_string()) {
            directive_line.push_str(" ");
            directive_line.push_str(domain.domain.as_str());
        }
    }

    directive_line.push_str("; ");
    directive_line
}

fn create_thread(directive: String, domains: domains::Collection) -> JoinHandle<String> {
    thread::spawn(move || self::build_line(directive, domains.clone()))
}

fn build_lines(directives: Vec<String>, domains: domains::Collection) -> Vec<JoinHandle<String>> {
    let mut threads: Vec<JoinHandle<String>> = vec![];

    for directive in directives {
        threads.push(self::create_thread(directive.to_string(), domains.clone()));
    }

    threads
}

pub fn build(directives_list: impl GetDirectives, json: &str) -> Result<String, error::Error> {
    let domains: Result<domains::Collection, error::Error> = parse::json(json);

    match domains {
        Ok(domains) => {
            let threads: Vec<JoinHandle<String>> =
                self::build_lines(directives_list.get_directives(), domains);

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
    fn test_build_line() {
        let directives: Vec<String> = vec![String::from("connect-src"), String::from("script-src")];

        let item = domains::Item {
            domain: String::from("*.example.com"),
            directive: directives,
        };

        let mut domain_list: Vec<domains::Item> = Vec::new();
        domain_list.push(item);

        let json = domains::Collection {
            domains: domain_list,
        };

        let connect_src: String = super::build_line(String::from("connect-src"), json);

        assert_eq!(connect_src, String::from("connect-src: *.example.com; "));
    }

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
            String::from("script-src: test.com; connect-src: example.com test.com;")
        );
    }
}
