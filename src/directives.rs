use std::thread;
use std::thread::JoinHandle;
use crate::domains;
use crate::parse;
use crate::GetDirectives;

fn directive_line(directive: String, csp: domains::Collection) -> String {
    let mut directive_line: String = directive.to_string();
    directive_line.push_str(":");

    for domain in csp.domains {
        if domain.directive.contains(&directive.to_string()) {
            directive_line.push_str(" ");
            directive_line.push_str(domain.domain.as_str());
        }
    }

    directive_line.push_str("; ");
    return directive_line;
}

fn something(directives: Vec<String>, json: &str) -> Vec<JoinHandle<String>> {
    let mut threads: Vec<JoinHandle<String>> = vec![];

    for directive in directives {
        let result: Result<domains::Collection, String> = parse::json(json);
        if !result.is_err() {
            let directive_string: String = directive.to_string();
            
            threads.push(
                thread::spawn(move || {
                    return self::directive_line(directive_string, result.unwrap());
                })
            );
        }
    }

    return threads;
}

pub fn build(directives_list: impl GetDirectives, json: &str) -> String {
    let result: Vec<JoinHandle<String>> = self::something(
        directives_list.get_directives(), 
        json
    );

    let mut directives: String = String::from("");

    for item in result {
        directives.push_str(item.join().unwrap().as_str());
    }

    return directives.trim().to_string();
}

#[cfg(test)]
mod directives_test {
    use crate::domains;
    use crate::config;

    #[test]
    fn test_directive_line() {

        let directive: Vec<String> = vec![
            String::from("connect-src"),
            String::from("script-src")
        ];

        let item = domains::Item{
            domain: String::from("*.example.com"),
            directive: directive
        };

        let mut domains: Vec<domains::Item> = Vec::new();
        domains.push(item);

        let json = domains::Collection{
            domains: domains
        };

        let connect_src: String = super::directive_line(String::from("connect-src"), json);

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

        let csp: String = super::build(config::get_directives(), json);

        assert_eq!(csp, String::from("script-src: test.com; connect-src: example.com test.com;"));
    }
}