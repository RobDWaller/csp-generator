use crate::csp_json;

fn directive_line(directive: &str, csp: csp_json::CspJson) -> String {
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

#[cfg(test)]
mod csp_string_test {
    use crate::csp_item;
    use crate::csp_json;

    #[test]
    fn test_directive_line() {

        let directive: Vec<String> = vec![
            String::from("connect-src"),
            String::from("script-src")
        ];

        let item = csp_item::CspItem{
            domain: String::from("*.example.com"),
            directive: directive
        };

        let mut domains: Vec<csp_item::CspItem> = Vec::new();
        domains.push(item);

        let json = csp_json::CspJson{
            domains: domains
        };

        let connect_src: String = super::directive_line("connect-src", json);

        assert_eq!(connect_src, String::from("connect-src: *.example.com;"));
    }
}