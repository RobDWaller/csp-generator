use crate::csp_json;

const DIRECTIVE_LIST: [&str; 2] = [
    "script-src", 
    "connect-src"
]; 

pub fn directive_line(directive: &str, csp: csp_json::CspJson) -> String {
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

pub fn build_directives(json: &str) -> String {
    let mut directives: String = String::from("");

    for directive in DIRECTIVE_LIST.iter() {
        let option: Option<csp_json::CspJson> = csp_json::json_to_csp(json);
        if !option.is_none() {
            directives.push_str(self::directive_line(directive, option.unwrap()).as_str());
        }
    }
    
    return directives.trim().to_string();
}