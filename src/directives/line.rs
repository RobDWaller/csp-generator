use crate::domains;

fn domains_to_directive(directive: String, domains: Vec<domains::Item>) -> String {
    let mut directive_line = directive.clone();

    for domain in domains {
        if domain.directive.contains(&directive) {
            directive_line.push_str(" ");
            directive_line.push_str(domain.domain.as_str());
        }
    }

    directive_line.push_str("; ");
    directive_line
}

fn check_line(directive_line: String, check: String) -> String {
    if directive_line == check {
        return String::from("");
    }

    directive_line
}

fn create_check(mut directive: String) -> String {
    directive.push_str("; ");
    directive
}

pub fn build(directive: String, domains: domains::Collection) -> String {
    let directive_line = domains_to_directive(directive.clone(), domains.domains);

    check_line(directive_line, create_check(directive))
}

// -----
// Tests
// -----
#[cfg(test)]
mod lines_test {
    use crate::domains;

    #[test]
    fn test_build() {
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

        let connect_src: String = super::build(String::from("connect-src"), json);

        assert_eq!(connect_src, String::from("connect-src *.example.com; "));
    }

    #[test]
    fn test_build_line_no_directive() {
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

        let default_src: String = super::build(String::from("default-src"), json);

        assert_eq!(default_src, String::from(""));
    }
}
