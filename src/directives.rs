use std::thread;
use std::thread::JoinHandle;
use crate::domains;
use crate::parse;

const DIRECTIVE_LIST: [&str; 2] = [
    "script-src", 
    "connect-src"
]; 

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

fn something(list: [&str; 2], json: &str) -> Vec<JoinHandle<String>> {
    let mut directive_results: Vec<JoinHandle<String>> = vec![];

    for directive in list.iter() {
        let option: Option<domains::Collection> = parse::json(json);
        if !option.is_none() {
            let directive_string: String = directive.to_string();
            
            directive_results.push(
                thread::spawn(move || {
                    return self::directive_line(directive_string, option.unwrap());
                })
            );
        }
    }

    return directive_results;
}

pub fn build(json: &str) -> String {
    let directive_results: Vec<JoinHandle<String>> = self::something(DIRECTIVE_LIST, json);

    let mut directives: String = String::from("");

    for item in directive_results {
        directives.push_str(item.join().unwrap().as_str());
    }

    return directives.trim().to_string();
}

#[cfg(test)]
mod directives_test {
    use crate::domains;

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
}