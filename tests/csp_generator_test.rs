#[cfg(test)]
mod csp_generator_test {
    extern crate csp_generator;
    use csp_generator::config;

    #[test]
    fn test_enforce() {
        let json = r#"
            {
                "domains": [
                    {"domain": "example.com", "directive": ["connect-src"]},
                    {"domain": "test.com", "directive": ["connect-src", "script-src"]}
                ]
            }
        "#;

        let csp: String = csp_generator::enforce(config::get_directives(), json);

        assert_eq!(
            csp,
            String::from(
                "Content-Security-Policy script-src test.com; connect-src example.com test.com;"
            )
        );
    }

    #[test]
    #[should_panic(expected = "Could not parse JSON: EOF while parsing a value at line 1 column 0")]
    fn test_enforce_empty_fail() {
        let json = "";

        csp_generator::enforce(config::get_directives(), json);
    }

    #[test]
    #[should_panic(expected = "Could not parse JSON: missing field")]
    fn test_enforce_format_fail() {
        let json = r#"
            {
                "domains": [
                    {"domain": "example.com", "diroctive": ["connect-src"]},
                    {"domain": "test.com", "directive": ["connect-src", "script-src"]}
                ]
            }
        "#;

        csp_generator::enforce(config::get_directives(), json);
    }

    #[test]
    fn test_report_only() {
        let json = r#"
            {
                "domains": [
                    {"domain": "example.com", "directive": ["connect-src"]},
                    {"domain": "test.com", "directive": ["connect-src", "script-src"]}
                ]
            }
        "#;

        let csp: String = csp_generator::report_only(config::get_directives(), json);

        assert_eq!(
            csp,
            String::from(
                "Content-Security-Policy-Report-Only script-src test.com; connect-src example.com test.com;"
            )
        );
    }

    #[test]
    #[should_panic(expected = "Could not parse JSON: EOF while parsing a value at line 1 column 0")]
    fn test_report_only_empty_fail() {
        let json = "";

        csp_generator::report_only(config::get_directives(), json);
    }

    #[test]
    #[should_panic(expected = "Could not parse JSON: missing field")]
    fn test_report_only_format_fail() {
        let json = r#"
            {
                "domains": [
                    {"domain": "example.com", "directive": ["connect-src"]},
                    {"directive": ["connect-src", "script-src"]}
                ]
            }
        "#;

        csp_generator::report_only(config::get_directives(), json);
    }
}
