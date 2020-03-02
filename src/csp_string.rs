use std::thread;
use std::thread::JoinHandle;
use crate::csp_json;

const DIRECTIVE_LIST: [&str; 2] = [
    "script-src", 
    "connect-src"
]; 

pub fn directive_line(directive: String, csp: csp_json::CspJson) -> String {
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
        let option: Option<csp_json::CspJson> = csp_json::json_to_csp(json);
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

pub fn build_directives(json: &str) -> String {
    let directive_results: Vec<JoinHandle<String>> = self::something(DIRECTIVE_LIST, json);

    let mut directives: String = String::from("");

    for item in directive_results {
        directives.push_str(item.join().unwrap().as_str());
    }

    return directives.trim().to_string();
}