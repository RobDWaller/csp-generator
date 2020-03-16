use crate::domains;

pub fn build_line(directive: String, domains: domains::Collection) -> String {
    let mut directive_line: String = directive.clone();
    let mut directive_check: String = directive.clone();
    directive_check.push_str("; ");

    for domain in domains.domains {
        if domain.directive.contains(&directive) {
            directive_line.push_str(" ");
            directive_line.push_str(domain.domain.as_str());
        }
    }

    directive_line.push_str("; ");

    if directive_line == directive_check {
        return String::from("");
    }

    directive_line
}

#[cfg(test)]
mod lines_test {
    use crate::domains;
    
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

        let default_src: String = super::build_line(String::from("default-src"), json);

        assert_eq!(default_src, String::from(""));
    }
}