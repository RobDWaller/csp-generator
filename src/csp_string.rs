use crate::csp_json;

pub fn directive_line(directive: &str, csp: csp_json::CspJson) -> String {
    let mut directive_line: String = directive.to_string();
    directive_line.push_str(":");

    for domain in csp.domains {
        if domain.directive.contains(&directive.to_string()) {
            directive_line.push_str(" ");
            directive_line.push_str(domain.domain.as_str());
        }
    }

    directive_line.push_str(";");
    return directive_line;
}