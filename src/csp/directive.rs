use crate::domains::{Collection};
use crate::csp::domains;

// Make an individual CSP directive.
fn make(directive: &str, domains: &Vec<String>) -> String {
    let mut directive_line = String::new(); 
    directive_line.push_str(directive);
    directive_line.push_str(" ");

    for domain in domains {
        directive_line.push_str(domain.as_str());
        if domains.last().unwrap() != domain {
            directive_line.push_str(" ");
        }
    }

    directive_line.push_str(";");
    directive_line
}

// Generate a CSP directive line for the supplied directive based on the
// domains config.
pub fn generate(directive: String, domains: Collection) -> String {
    let found = domains::find(&directive.as_str(), &domains);

    if found.is_empty() {
        return String::from("");
    }

    make(&directive.as_str(), &found)
}

// -----
// Tests
// -----
#[cfg(test)]
mod lines_test {
    use crate::domains::{Collection, Item};

    #[test]
    fn test_build() {
        let directives: Vec<String> = vec![String::from("connect-src"), String::from("script-src")];

        let item = Item {
            domain: String::from("*.example.com"),
            directives,
        };

        let mut domain_list: Collection = Vec::new();
        domain_list.push(item);

        let connect_src: String = super::generate(String::from("connect-src"), domain_list);

        assert_eq!(connect_src, String::from("connect-src *.example.com;"));
    }

    #[test]
    fn test_build_line_no_directive() {
        let directives: Vec<String> = vec![String::from("connect-src"), String::from("script-src")];

        let item = Item {
            domain: String::from("*.example.com"),
            directives,
        };

        let mut domain_list: Collection = Vec::new();
        domain_list.push(item);

        let default_src: String = super::generate(String::from("default-src"), domain_list);

        assert_eq!(default_src, String::from(""));
    }
}
